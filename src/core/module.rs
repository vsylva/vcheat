use crate::{core, ffi};

pub(crate) unsafe fn get_modules_info(
    process_id: u32,
) -> crate::Result<Vec<core::ModuleInformation>> {
    let snapshot_handle: *mut ::core::ffi::c_void =
        ffi::CreateToolhelp32Snapshot(0x8 | 0x10, process_id);

    if snapshot_handle as isize == -1 {
        return Err(format!(
            "The function CreateToolhelp32Snapshot failed with a return value of {:p}",
            snapshot_handle
        ));
    }

    let mut module_entry: ffi::ModuleEntry32W = ::core::mem::zeroed::<ffi::ModuleEntry32W>();

    module_entry.dw_size = ::core::mem::size_of::<ffi::ModuleEntry32W>() as u32;

    if 0 == ffi::Module32FirstW(snapshot_handle, &mut module_entry) {
        crate::core::process::close_handle(snapshot_handle)?;

        return Err("The function Module32FirstW failed".to_string());
    }

    let mut module_entry_array: Vec<ffi::ModuleEntry32W> = Vec::<ffi::ModuleEntry32W>::new();

    module_entry_array.push(module_entry.to_owned());

    while 0 != ffi::Module32NextW(snapshot_handle, &mut module_entry) {
        module_entry_array.push(module_entry.to_owned());
    }

    crate::core::process::close_handle(snapshot_handle)?;

    let mut module_info_array: Vec<core::ModuleInformation> = Vec::<core::ModuleInformation>::new();

    for module_entry in module_entry_array {
        module_info_array.push(core::ModuleInformation {
            process_id: module_entry.th32_process_id,
            base_address: module_entry.mod_base_addr,
            size: module_entry.mod_base_size,
            handle: module_entry.h_module,
            name: {
                let result: ::std::ffi::OsString =
                    ::std::os::windows::prelude::OsStringExt::from_wide(&module_entry.sz_module);

                match result.to_str() {
                    Some(some) => some.trim_end_matches('\0').to_string(),
                    None => return Err("None".to_string()),
                }
            },
            path: {
                let result: ::std::ffi::OsString =
                    ::std::os::windows::prelude::OsStringExt::from_wide(&module_entry.sz_exe_path);

                match result.to_str() {
                    Some(some) => some.trim_end_matches('\0').to_string(),
                    None => return Err("None".to_string()),
                }
            },
        })
    }

    Ok(module_info_array)
}

pub(crate) unsafe fn get_module_info(
    process_id: u32,
    module_name: &str,
) -> crate::Result<core::ModuleInformation> {
    if module_name.is_empty() {
        return Err("module_name can not be empty".to_string());
    }

    let module_name: String = module_name.to_lowercase();

    let snapshot_handle: *mut ::core::ffi::c_void =
        ffi::CreateToolhelp32Snapshot(0x8 | 0x10, process_id);

    if snapshot_handle as i32 == -1 {
        return Err(format!(
            "The function CreateToolhelp32Snapshot failed with a return value of {:p}",
            snapshot_handle
        ));
    }

    let mut module_entry: ffi::ModuleEntry32W = ::core::mem::zeroed::<ffi::ModuleEntry32W>();

    module_entry.dw_size = ::core::mem::size_of::<ffi::ModuleEntry32W>() as u32;

    if 0 == ffi::Module32FirstW(snapshot_handle, &mut module_entry) {
        crate::core::process::close_handle(snapshot_handle)?;

        return Err("The function Module32FirstW failed".to_string());
    }

    let module_entry_name: String = {
        let result: ::std::ffi::OsString =
            ::std::os::windows::prelude::OsStringExt::from_wide(&module_entry.sz_module);

        match result.to_str() {
            Some(some) => some.trim_end_matches('\0').to_string(),
            None => return Err("None".to_string()),
        }
    };

    if module_entry_name.to_lowercase() == module_name {
        crate::core::process::close_handle(snapshot_handle)?;

        return Ok(core::ModuleInformation {
            process_id: module_entry.th32_process_id,
            base_address: module_entry.mod_base_addr,
            size: module_entry.mod_base_size,
            handle: module_entry.h_module,
            name: module_entry_name,
            path: {
                let result: ::std::ffi::OsString =
                    ::std::os::windows::prelude::OsStringExt::from_wide(&module_entry.sz_exe_path);

                match result.to_str() {
                    Some(some) => some.trim_end_matches('\0').to_string(),
                    None => return Err("None".to_string()),
                }
            },
        });
    }

    while 0 != ffi::Module32NextW(snapshot_handle, &mut module_entry) {
        let module_entry_name: String = {
            let result: ::std::ffi::OsString =
                ::std::os::windows::prelude::OsStringExt::from_wide(&module_entry.sz_module);

            match result.to_str() {
                Some(some) => some.trim_end_matches('\0').to_string(),
                None => return Err("None".to_string()),
            }
        };

        if module_entry_name.to_lowercase() == module_name {
            crate::core::process::close_handle(snapshot_handle)?;

            return Ok(core::ModuleInformation {
                process_id: module_entry.th32_process_id,
                base_address: module_entry.mod_base_addr,
                size: module_entry.mod_base_size,
                handle: module_entry.h_module,
                name: module_entry_name,
                path: {
                    let result: ::std::ffi::OsString =
                        ::std::os::windows::prelude::OsStringExt::from_wide(
                            &module_entry.sz_exe_path,
                        );

                    match result.to_str() {
                        Some(some) => some.trim_end_matches('\0').to_string(),
                        None => return Err("None".to_string()),
                    }
                },
            });
        }
    }

    crate::core::process::close_handle(snapshot_handle)?;

    Err("The function Module32W failed".to_string())
}

pub(crate) unsafe fn load_library(dll_path: &str) -> crate::Result<*mut ::core::ffi::c_void> {
    if dll_path.is_empty() {
        return Err("dll_path cannot be empty".to_string());
    }

    if dll_path.len() > 260 {
        return Err("The length of dll_path cannot be greater than 260".to_string());
    }

    let dll_path_buf: ::std::path::PathBuf = ::std::path::Path::new(dll_path)
        .canonicalize()
        .map_err(|err| err.to_string())?;

    let mut dll_path: String = match dll_path_buf.to_str() {
        Some(some) => some.trim_start_matches(r"\\?\").to_string(),
        None => return Err("None".to_string()),
    };

    dll_path.push('\0');

    let dll_path_buffer: Vec<u16> = dll_path.encode_utf16().collect::<Vec<u16>>();

    let module_handle: *mut ::core::ffi::c_void = ffi::LoadLibraryW(dll_path_buffer.as_ptr());

    if module_handle.is_null() {
        return Err("The function LoadLibraryW failed".to_string());
    }

    Ok(module_handle)
}

pub(crate) unsafe fn load_system_library(
    dll_name: &str,
) -> crate::Result<*mut ::core::ffi::c_void> {
    if dll_name.is_empty() {
        return Err("dll_name cannot be empty".to_string());
    }

    let mut sys_dir_path_buffer: Vec<u16> = Vec::new();

    sys_dir_path_buffer.resize(260, 0);

    if 0 == ffi::GetSystemDirectoryW(sys_dir_path_buffer.as_mut_ptr(), 260) {
        return Err("The function GetSystemDirectoryW failed".to_string());
    }

    let mut dll_path: String = {
        let result: ::std::ffi::OsString =
            ::std::os::windows::prelude::OsStringExt::from_wide(&sys_dir_path_buffer);

        match result.to_str() {
            Some(some) => some.trim_end_matches('\0').to_string(),
            None => return Err("None".to_string()),
        }
    };

    dll_path.push_str(r"\");

    dll_path.push_str(dll_name);

    load_library(&dll_path)
}

pub(crate) unsafe fn free_library(module_handle: *mut ::core::ffi::c_void) -> crate::Result<()> {
    if module_handle.is_null() {
        return Err("Incorrect parameter module_handle".to_string());
    }

    if 0 == ffi::FreeLibrary(module_handle) {
        return Err("The function FreeLibrary failed".to_string());
    }

    Ok(())
}

pub(crate) unsafe fn get_proc_address(
    module_handle: *mut ::core::ffi::c_void,
    proc_name: &str,
) -> crate::Result<*mut ::core::ffi::c_void> {
    if module_handle.is_null() {
        return Err("Incorrect parameter module_handle".to_string());
    }

    if proc_name.is_empty() {
        return Err("proc_name cannot be empty".to_string());
    }

    let mut proc_name_bytes: Vec<u8> = proc_name.as_bytes().to_vec();

    proc_name_bytes.push(b'\0');

    let proc_address = ffi::GetProcAddress(module_handle, proc_name_bytes.as_mut_ptr().cast());

    if proc_address.is_null() {
        return Err("The function GetProcAddress failed".to_string());
    }

    Ok(proc_address)
}

pub(crate) unsafe fn inject_dll(
    process_handle: *mut ::core::ffi::c_void,
    dll_path: &str,
) -> crate::Result<()> {
    if process_handle.is_null() {
        return Err("Incorrect parameter process_handle".to_string());
    }

    if dll_path.is_empty() {
        return Err("dll_path cannot be empty".to_string());
    }

    if dll_path.len() > 260 {
        return Err("The length of dll_path cannot be greater than 260".to_string());
    }

    let dll_path_buf: ::std::path::PathBuf = ::std::path::Path::new(dll_path)
        .canonicalize()
        .map_err(|err| err.to_string())?;

    let mut dll_path: String = match dll_path_buf.to_str() {
        Some(some) => some.trim_start_matches(r"\\?\").to_string(),
        None => return Err("None".to_string()),
    };

    dll_path.push('\0');

    let dll_path_buffer = dll_path.encode_utf16().collect::<Vec<u16>>();

    let dll_path_buffer_size = dll_path_buffer.len() * ::core::mem::size_of::<u16>();

    let kernel32_handle = crate::core::module::load_system_library("kernel32.dll")?;

    let load_library_w_proc =
        crate::core::module::get_proc_address(kernel32_handle, "LoadLibraryW")?;

    let dll_path_buffer_alloc = crate::core::memory::virtual_alloc_ex(
        process_handle,
        ::core::ptr::null_mut(),
        dll_path_buffer_size,
        core::mem_allocation::COMMIT,
        core::page_protect::READ_WRITE,
    )?;

    crate::core::memory::write_process_memory(
        process_handle,
        dll_path_buffer_alloc,
        &dll_path_buffer,
    )?;

    let remote_thread_handle = ffi::CreateRemoteThread(
        process_handle,
        ::core::ptr::null_mut(),
        0,
        ::core::mem::transmute(load_library_w_proc),
        dll_path_buffer_alloc.cast(),
        0,
        ::core::ptr::null_mut(),
    );

    if remote_thread_handle.is_null() {
        crate::core::memory::virtual_free_ex(
            process_handle,
            dll_path_buffer_alloc,
            0,
            core::mem_free::RELEASE,
        )?;

        return Err("The function CreateRemoteThread failed".to_string());
    }

    let _result = ffi::WaitForSingleObject(remote_thread_handle, 0xFFFFFFFF);

    // if result != 0 {
    //     crate::core::memory::virtual_free_ex(
    //         process_handle,
    //         dll_path_buffer_alloc,
    //         0,
    //         core::mem_free::RELEASE,
    //     )?;

    //     crate::core::process::close_handle(remote_thread_handle)?;

    //     return Err(
    //         "The function WaitForSingleObject failed with a return value of: {result:X}"
    //             .to_string(),
    //     );
    // }

    crate::core::memory::virtual_free_ex(
        process_handle,
        dll_path_buffer_alloc,
        0,
        core::mem_free::RELEASE,
    )?;

    crate::core::process::close_handle(remote_thread_handle)?;

    Ok(())
}

pub(crate) unsafe fn eject_dll(
    process_handle: *mut ::core::ffi::c_void,
    module_handle: *mut ::core::ffi::c_void,
) -> crate::Result<()> {
    if process_handle.is_null() {
        return Err("Incorrect parameter process_handle".to_string());
    }

    if module_handle.is_null() {
        return Err("Incorrect parameter module_handle".to_string());
    }

    let kernel32_handle = crate::core::module::load_system_library("kernel32.dll")?;

    let proc_free_library = crate::core::module::get_proc_address(kernel32_handle, "FreeLibrary")?;

    let remote_thread_handle = ffi::CreateRemoteThread(
        process_handle,
        ::core::ptr::null_mut(),
        0,
        ::core::mem::transmute(proc_free_library),
        module_handle,
        0,
        ::core::ptr::null_mut(),
    );

    if remote_thread_handle.is_null() {
        return Err("The function CreateRemoteThread failed".to_string());
    }

    let _result = ffi::WaitForSingleObject(remote_thread_handle, 0xFFFFFFFF);

    // if result != 0 {
    //     crate::core::process::close_handle(remote_thread_handle)?;

    //     return Err(
    //         "The function WaitForSingleObject failed with a return value of: {result:X}"
    //             .to_string(),
    //     );
    // }

    crate::core::process::close_handle(remote_thread_handle)?;

    Ok(())
}
