use crate::*;

pub fn get_all_process_modules_info(
    process_id: u32,
    read_module_data: bool,
) -> Result<Vec<VModuleInfo>> {
    let mut modules_info = Vec::<VModuleInfo>::new();
    unsafe {
        for m in module::get_all_process_modules_info(process_id)? {
            modules_info.push(VModuleInfo {
                process_id: m.th32_process_id,
                module_address: m.mod_base_addr,
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
                module_data: if read_module_data {
                    get_module_data(m.th32_process_id, m.mod_base_addr.cast(), m.mod_base_size).ok()
                } else {
                    None
                },
            })
        }
    }

    Ok(modules_info)
}

pub fn get_module_data(
    process_id: u32,
    module_address: *mut core::ffi::c_void,
    module_size: u32,
) -> Result<Vec<u8>> {
    unsafe {
        let process_handle = get_process_handle(process_id)?;

        let module_data = read_memory(process_handle, module_address, module_size as usize)?;

        CloseHandle(process_handle);

        Ok(module_data)
    }
}
