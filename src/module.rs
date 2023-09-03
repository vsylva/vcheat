use crate::{ffi::*, process::*, types::*};

type Result<T> = std::result::Result<T, String>;

pub(crate) unsafe fn get_all_process_modules_info(process_id: u32) -> Result<Vec<ModuleInfo>> {
    #[cfg(target_arch = "x86")]
    {
        let is_wow64_process: bool = is_wow64_process(process_id)?;
        if !is_wow64_process {
            return Err(format!("The process({process_id}) is 64-bit"));
        }
    }

    let snapshot_handle: *mut core::ffi::c_void = CreateToolhelp32Snapshot(0x8 | 0x10, process_id);

    if snapshot_handle.is_null() {
        return Err(format!(
            "CreateToolhelp32Snapshot failed with result: {snapshot_handle:X?}"
        ));
    }

    let module_entry: &mut ModuleEntry32W = &mut core::mem::zeroed::<ModuleEntry32W>();

    module_entry.dw_size = core::mem::size_of::<ModuleEntry32W>() as u32;

    let result: i32 = Module32FirstW(snapshot_handle, module_entry);

    if result == 0 {
        close_handle(snapshot_handle)?;
        return Err(format!("Module32FirstW failed with result: {result:X}"));
    }

    let mut module_entry_array: Vec<ModuleEntry32W> = Vec::<ModuleEntry32W>::new();

    module_entry_array.push(module_entry.clone());

    while Module32NextW(snapshot_handle, module_entry) != 0 {
        module_entry_array.push(module_entry.clone());
    }

    if !snapshot_handle.is_null() {
        close_handle(snapshot_handle)?
    }

    let mut module_info_array: Vec<ModuleInfo> = Vec::<ModuleInfo>::new();

    for m in module_entry_array {
        module_info_array.push(ModuleInfo {
            process_id: m.th32_process_id,
            module_base_address: m.mod_base_addr,
            module_size: m.mod_base_size,
            module_handle: m.h_module,
            module_name: core::ffi::CStr::from_ptr(
                String::from_utf16_lossy(m.sz_module.as_ref())
                    .as_ptr()
                    .cast(),
            )
            .to_string_lossy()
            .to_string(),
            module_path: core::ffi::CStr::from_ptr(
                String::from_utf16_lossy(m.sz_exe_path.as_ref())
                    .as_ptr()
                    .cast(),
            )
            .to_string_lossy()
            .to_string(),
        })
    }

    Ok(module_info_array)
}

pub(crate) unsafe fn load_library<S: AsRef<std::path::Path>>(
    path: S,
) -> Result<*mut core::ffi::c_void> {
    if !path.as_ref().exists() {
        return Err("Path does not exist".to_string());
    }

    let module_handle: *mut core::ffi::c_void = if path
        .as_ref()
        .display()
        .to_string()
        .chars()
        .any(|c| !c.is_ascii())
    {
        let mut path: Vec<u16> = path
            .as_ref()
            .to_string_lossy()
            .encode_utf16()
            .collect::<Vec<u16>>();

        path.push(0);

        LoadLibraryW(path.as_ptr())
    } else {
        let mut path = path.as_ref().to_string_lossy().bytes().collect::<Vec<u8>>();

        path.push(0);

        LoadLibraryA(path.as_ptr().cast())
    };

    if module_handle.is_null() {
        return Err("LoadLibrary failed".to_string());
    }

    Ok(module_handle)
}

pub(crate) unsafe fn load_system_library<S: AsRef<str>>(
    file_name: S,
) -> Result<*mut core::ffi::c_void> {
    let mut sys_dir_path: String = if file_name.as_ref().chars().any(|c| !c.is_ascii()) {
        let mut sys_dir_path_buffer: Vec<u16> = Vec::new();
        sys_dir_path_buffer.resize(260, 0);

        if 0 == GetSystemDirectoryW(sys_dir_path_buffer.as_mut_ptr(), 260) {
            return Err("GetSystemDirectoryW failed".to_string());
        }

        String::from_utf16_lossy(&sys_dir_path_buffer)
    } else {
        let mut sys_dir_path_buffer: Vec<i8> = Vec::new();
        sys_dir_path_buffer.resize(260, 0);

        if 0 == GetSystemDirectoryA(sys_dir_path_buffer.as_mut_ptr(), 260) {
            return Err("GetSystemDirectoryA failed".to_string());
        }

        core::ffi::CStr::from_ptr(sys_dir_path_buffer.as_ptr())
            .to_string_lossy()
            .to_string()
    };

    sys_dir_path.push_str(r"\");

    sys_dir_path.push_str(file_name.as_ref());

    load_library(sys_dir_path)
}

pub(crate) unsafe fn get_proc_address<S: AsRef<str>>(
    module_handle: *mut core::ffi::c_void,
    proc_name: S,
) -> *mut core::ffi::c_void {
    let mut bytes = proc_name.as_ref().as_bytes().to_vec();
    bytes.push(0);

    GetProcAddress(module_handle, bytes.as_mut_ptr().cast())
}
