use crate::*;

pub(crate) unsafe fn get_all_process_modules_info(process_id: u32) -> Result<Vec<ModuleEntry32W>> {
    let snapshot_handle = CreateToolhelp32Snapshot(0x8 | 0x10, process_id);

    if snapshot_handle.is_null() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("CreateToolhelp32Snapshot failed with return value: {snapshot_handle:X?}"),
        )));
    }

    let module_entry = &mut core::mem::zeroed() as *mut ModuleEntry32W;

    (*module_entry).dw_size = core::mem::size_of::<ModuleEntry32W>() as u32;

    let result = Module32FirstW(snapshot_handle, module_entry);

    if result == 0 {
        CloseHandle(snapshot_handle);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Module32FirstW failed with return value: {result:X}"),
        )));
    }

    let mut module_entry_array = Vec::<ModuleEntry32W>::new();

    module_entry_array.push(*module_entry);

    while Module32NextW(snapshot_handle, module_entry) != 0 {
        module_entry_array.push(*module_entry);
    }

    CloseHandle(snapshot_handle);

    Ok(module_entry_array)
}
