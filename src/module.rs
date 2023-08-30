use crate::*;

pub fn get_all_process_modules_info(process_id: u32) -> Result<Vec<ModuleInfo>> {
    unsafe {
        #[cfg(any(
            all(target_arch = "arm", target_pointer_width = "32"),
            target_arch = "x86"
        ))]
        {
            let is_wow64_process = is_wow64_process(process_id)?;
            if !is_wow64_process {
                return Err(format!("The process({process_id}) is 64-bit"));
            }
        }

        let snapshot_handle = CreateToolhelp32Snapshot(0x8 | 0x10, process_id);

        if snapshot_handle.is_null() {
            return Err(format!(
                "CreateToolhelp32Snapshot failed with return value: {snapshot_handle:X?}"
            ));
        }

        let module_entry = &mut core::mem::zeroed() as *mut ModuleEntry32W;
        (*module_entry).dw_size = core::mem::size_of::<ModuleEntry32W>() as u32;

        let result = Module32FirstW(snapshot_handle, module_entry);

        if result == 0 {
            close_handle(snapshot_handle)?;
            return Err(format!(
                "Module32FirstW failed with return value: {result:X}"
            ));
        }

        let mut module_entry_array = Vec::<ModuleEntry32W>::new();

        module_entry_array.push(module_entry.read());

        while Module32NextW(snapshot_handle, module_entry) != 0 {
            module_entry_array.push(module_entry.read());
        }

        if !snapshot_handle.is_null() {
            close_handle(snapshot_handle)?
        }

        let mut module_info_array = Vec::<ModuleInfo>::new();

        for m in module_entry_array {
            module_info_array.push(ModuleInfo {
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
            })
        }

        Ok(module_info_array)
    }
}
