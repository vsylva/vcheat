use crate::{core::types::*, ffi::*};

pub(crate) unsafe fn get_modules_info(process_id: u32) -> crate::Result<Vec<ModuleInfo>> {
    let snapshot_handle: *mut core::ffi::c_void = CreateToolhelp32Snapshot(0x8 | 0x10, process_id);

    if snapshot_handle.is_null() {
        return Err("The function CreateToolhelp32Snapshot failed".to_string());
    }

    let module_entry: &mut ModuleEntry32W = &mut core::mem::zeroed::<ModuleEntry32W>();

    module_entry.dw_size = core::mem::size_of::<ModuleEntry32W>() as u32;

    if 0 == Module32FirstW(snapshot_handle, module_entry) {
        crate::core::process::close_handle(snapshot_handle)?;

        return Err("The function Module32FirstW failed".to_string());
    }

    let mut module_entry_array: Vec<ModuleEntry32W> = Vec::<ModuleEntry32W>::new();

    module_entry_array.push(module_entry.to_owned());

    while 0 != Module32NextW(snapshot_handle, module_entry) {
        module_entry_array.push(module_entry.to_owned());
    }

    crate::core::process::close_handle(snapshot_handle)?;

    let mut module_info_array: Vec<ModuleInfo> = Vec::<ModuleInfo>::new();

    for module_entry in module_entry_array {
        module_info_array.push(ModuleInfo {
            process_id: module_entry.th32_process_id,
            base_address: module_entry.mod_base_addr,
            size: module_entry.mod_base_size,
            handle: module_entry.h_module,
            name: String::from_utf16_lossy(&module_entry.sz_module).replace("\0", ""),
            path: String::from_utf16_lossy(&module_entry.sz_exe_path).replace("\0", ""),
        })
    }

    Ok(module_info_array)
}

pub(crate) unsafe fn load_library(dll_path: &str) -> crate::Result<*mut core::ffi::c_void> {
    if dll_path.len() > 260 {
        return Err("The path length of a dynamic library cannot be greater than 260".to_string());
    }

    let dll_path: String = match std::path::Path::new(dll_path).canonicalize() {
        Ok(ok) => ok,

        Err(err) => return Err(err.to_string()),
    }
    .display()
    .to_string()
    .trim_start_matches(r"\\?\")
    .to_string();

    let mut dll_path_buffer: Vec<u16> = dll_path.encode_utf16().collect::<Vec<u16>>();

    dll_path_buffer.push(0);

    let module_handle: *mut core::ffi::c_void = LoadLibraryW(dll_path_buffer.as_ptr());

    if module_handle.is_null() {
        return Err("The function LoadLibraryW failed".to_string());
    }

    Ok(module_handle)
}

pub(crate) unsafe fn load_system_library(dll_name: &str) -> crate::Result<*mut core::ffi::c_void> {
    let mut sys_dir_path_buffer: Vec<u16> = Vec::new();

    sys_dir_path_buffer.resize(260, 0);

    if 0 == GetSystemDirectoryW(sys_dir_path_buffer.as_mut_ptr(), 260) {
        return Err("The function GetSystemDirectoryW failed".to_string());
    }

    let mut dll_path: String = String::from_utf16_lossy(&sys_dir_path_buffer).replace("\0", "");

    dll_path.push_str(r"\");

    dll_path.push_str(dll_name);

    load_library(&dll_path)
}

pub(crate) unsafe fn free_library(module_handle: *mut core::ffi::c_void) -> crate::Result<()> {
    if 0 == FreeLibrary(module_handle) {
        return Err("The function FreeLibrary failed".to_string());
    }

    Ok(())
}

// fn convert_wide_string(wide_string: &[u16]) -> String {
//     wide_string
//         .iter()
//         .cloned()
//         .filter(|&c| c != 0 && c != 0xFFFD)
//         .map(|c| char::from_u32(c as u32).unwrap())
//         .collect::<String>()
// }

pub(crate) unsafe fn get_proc_address(
    module_handle: *mut core::ffi::c_void,
    proc_name: &str,
) -> crate::Result<*mut core::ffi::c_void> {
    let mut proc_name_bytes = proc_name.as_bytes().to_vec();

    proc_name_bytes.push(0x0);

    let proc_address = GetProcAddress(module_handle, proc_name_bytes.as_mut_ptr().cast());

    if proc_address.is_null() {
        return Err("The function GetProcAddress failed".to_string());
    }

    Ok(proc_address)
}

pub(crate) unsafe fn inject_dll(
    process_handle: *mut core::ffi::c_void,
    dll_path: &str,
) -> crate::Result<()> {
    if dll_path.len() > 260 {
        return Err("The path length of a dynamic library cannot be greater than 260".to_string());
    }

    let dll_path: String = match std::path::Path::new(dll_path).canonicalize() {
        Ok(ok) => ok,

        Err(err) => return Err(err.to_string()),
    }
    .display()
    .to_string()
    .trim_start_matches(r"\\?\")
    .to_string();

    let kernel32_handle = crate::core::module::load_system_library("kernel32.dll")?;

    let proc_load_library_w =
        crate::core::module::get_proc_address(kernel32_handle, "LoadLibraryW")?;

    let mut dll_path_buffer = dll_path.encode_utf16().collect::<Vec<u16>>();

    let mut dll_path_buffer_len = dll_path_buffer.len() * core::mem::size_of::<u16>();

    if !dll_path_buffer.ends_with(&[0]) {
        dll_path_buffer.push(0);

        dll_path_buffer_len += core::mem::size_of::<u16>();
    }

    let dll_path_address = crate::core::memory::virtual_alloc_ex(
        process_handle,
        core::ptr::null_mut(),
        dll_path_buffer_len,
        MemAllocation::Commit.into(),
        PageProtect::ReadWrite.into(),
    )?;

    crate::core::memory::write_process_memory(process_handle, dll_path_address, &dll_path_buffer)?;

    let remote_thread_handle = CreateRemoteThread(
        process_handle,
        core::ptr::null_mut(),
        0,
        core::mem::transmute(proc_load_library_w),
        dll_path_address.cast(),
        0,
        &mut 0,
    );

    if remote_thread_handle.is_null() {
        crate::core::memory::virtual_free_ex(
            process_handle,
            dll_path_address,
            0,
            MemFree::Release.into(),
        )?;

        return Err("The function CreateRemoteThread failed".to_string());
    }

    let result = WaitForSingleObject(remote_thread_handle, 0xFFFFFFFF);

    if result != 0 {
        crate::core::memory::virtual_free_ex(
            process_handle,
            dll_path_address,
            0,
            MemFree::Release.into(),
        )?;

        crate::core::process::close_handle(remote_thread_handle)?;

        return Err(
            "The function WaitForSingleObject failed with a return value of: {result:X}"
                .to_string(),
        );
    }

    crate::core::memory::virtual_free_ex(
        process_handle,
        dll_path_address,
        0,
        MemFree::Release.into(),
    )?;

    crate::core::process::close_handle(remote_thread_handle)?;

    Ok(())
}

pub(crate) unsafe fn eject_dll(
    process_handle: *mut core::ffi::c_void,
    module_handle: *mut core::ffi::c_void,
) -> crate::Result<()> {
    let kernel32_handle = crate::core::module::load_system_library("kernel32.dll")?;

    let proc_free_library = crate::core::module::get_proc_address(kernel32_handle, "FreeLibrary")?;

    let remote_thread_handle = CreateRemoteThread(
        process_handle,
        core::ptr::null_mut(),
        0,
        core::mem::transmute(proc_free_library),
        module_handle,
        0,
        &mut 0,
    );

    if remote_thread_handle.is_null() {
        return Err("The function CreateRemoteThread failed".to_string());
    }

    let result = WaitForSingleObject(remote_thread_handle, 0xFFFFFFFF);

    if result != 0 {
        crate::core::process::close_handle(remote_thread_handle)?;

        return Err(
            "The function WaitForSingleObject failed with a return value of: {result:X}"
                .to_string(),
        );
    }

    crate::core::process::close_handle(remote_thread_handle)?;

    Ok(())
}
