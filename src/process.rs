use crate::*;

pub(crate) unsafe fn get_all_process_info() -> Result<Vec<ProcessEntry32W>> {
    let snapshot_handle = CreateToolhelp32Snapshot(0x2, 0);

    if snapshot_handle.is_null() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("CreateToolhelp32Snapshot failed with return value: {snapshot_handle:X?}"),
        )));
    }

    let process_entry = &mut core::mem::zeroed() as *mut ProcessEntry32W;

    (*process_entry).dw_size = core::mem::size_of::<ProcessEntry32W>() as u32;

    let result = Process32FirstW(snapshot_handle, process_entry);

    if result == 0 {
        CloseHandle(snapshot_handle);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Process32FirstW failed with return value: {result:X}"),
        )));
    }

    let mut process_entry_array = Vec::<ProcessEntry32W>::new();

    process_entry_array.push(*process_entry);

    while Process32NextW(snapshot_handle, process_entry) != 0 {
        process_entry_array.push(*process_entry);
    }

    CloseHandle(snapshot_handle);

    Ok(process_entry_array)
}

// pub(crate) unsafe fn nt_get_all_process_info() -> Result<Vec<SystemProcessInformation>> {
//     let mut return_length: u32 = 0;

//     NtQuerySystemInformation(5, core::ptr::null_mut(), 0, &mut return_length);

//     let mut buffer: Vec<u8> = Vec::new();
//     buffer.resize((return_length as usize) * 2, 0u8);

//     let result = NtQuerySystemInformation(
//         5,
//         buffer.as_mut_ptr().cast(),
//         buffer.capacity() as u32,
//         &mut return_length,
//     );

//     if result != 0 {
//         return Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             format!("NtQuerySystemInformation failed with return value: {result:X}"),
//         )));
//     }

//     let mut process_info_array = Vec::<SystemProcessInformation>::new();

//     let mut current_offset: u32 = 0;

//     let mut process_info = core::ptr::read::<SystemProcessInformation>(
//         buffer.as_ptr().offset(current_offset as isize).cast(),
//     );

//     while process_info.next_entry_offset != 0 {
//         if process_info.unique_process_id != 0 {
//             process_info_array.push(process_info);
//         }

//         current_offset += process_info.next_entry_offset;

//         process_info = core::ptr::read::<SystemProcessInformation>(
//             buffer.as_ptr().offset(current_offset as isize).cast(),
//         );
//     }

//     Ok(process_info_array)
// }
