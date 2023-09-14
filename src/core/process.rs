use crate::{core::types::*, ffi::*};

pub(crate) unsafe fn open_process(process_id: u32) -> crate::Result<*mut core::ffi::c_void> {
    let process_handle: *mut core::ffi::c_void = OpenProcess(0x1F0FFF, 0, process_id);

    if process_handle.is_null() {
        return Err("The function OpenProcess failed".to_string());
    }

    Ok(process_handle)
}

pub(crate) unsafe fn close_handle(handle: *mut core::ffi::c_void) -> crate::Result<()> {
    if handle.is_null() {
        return Ok(());
    }

    let result = CloseHandle(handle);

    if result == 0 {
        return Err("The function CloseHandle failed".to_string());
    }

    Ok(())
}

pub(crate) unsafe fn is_wow64_process(
    process_handle: *mut core::ffi::c_void,
) -> crate::Result<bool> {
    let mut is_wow64: i32 = 0;

    let result: i32 = IsWow64Process(process_handle, &mut is_wow64);

    if result == 0 {
        return Err("The function IsWow64Process failed".to_string());
    }

    Ok(is_wow64 != 0)
}

pub(crate) unsafe fn get_processes_info() -> crate::Result<Vec<ProcessInfo>> {
    let snapshot_handle: *mut core::ffi::c_void = CreateToolhelp32Snapshot(0x2, 0x0);

    if snapshot_handle.is_null() {
        return Err("The function CreateToolhelp32Snapshot failed".to_string());
    }

    let process_entry: &mut ProcessEntry32W = &mut core::mem::zeroed::<ProcessEntry32W>();

    process_entry.dw_size = core::mem::size_of::<ProcessEntry32W>() as u32;

    let result: i32 = Process32FirstW(snapshot_handle, process_entry);

    if result == 0 {
        close_handle(snapshot_handle)?;

        return Err("The function Process32FirstW failed".to_string());
    }

    let mut process_entry_array: Vec<ProcessEntry32W> = Vec::<ProcessEntry32W>::new();

    process_entry_array.push(process_entry.to_owned());

    while Process32NextW(snapshot_handle, process_entry) != 0 {
        process_entry_array.push(process_entry.to_owned());
    }

    close_handle(snapshot_handle)?;

    let mut process_info_array: Vec<ProcessInfo> = Vec::<ProcessInfo>::new();

    for p in process_entry_array {
        process_info_array.push(ProcessInfo {
            id: p.th32_process_id,
            thread_count: p.cnt_threads,
            parent_process_id: p.th32_parent_process_id,
            base_priority_class: p.pc_pri_class_base,
            name: String::from_utf16_lossy(&p.sz_exe_file).replace("\0", ""),
        });
    }

    Ok(process_info_array)
}

pub(crate) unsafe fn nt_get_processes_info() -> crate::Result<Vec<SystemProcessInfo>> {
    let mut return_length: u32 = 0;

    NtQuerySystemInformation(5, core::ptr::null_mut(), 0, &mut return_length);

    let mut buffer: Vec<u8> = vec![0; return_length as usize * 2];

    let result: i32 = NtQuerySystemInformation(
        5,
        buffer.as_mut_ptr().cast(),
        return_length * 2,
        &mut return_length,
    );

    if result != 0 {
        return Err(format!(
            "The function NtQuerySystemInformation failed with a return value of: {result:X}"
        ));
    }

    let mut process_info_array: Vec<SystemProcessInformation> =
        Vec::<SystemProcessInformation>::new();

    let mut current_offset: isize = 0;

    let mut process_info: SystemProcessInformation =
        core::ptr::read::<SystemProcessInformation>(buffer.as_ptr().cast());

    while process_info.next_entry_offset != 0 {
        if process_info.unique_process_id != 0 {
            process_info_array.push(process_info.clone());
        }

        current_offset += process_info.next_entry_offset as isize;

        if buffer.as_ptr().offset(current_offset).is_null() {
            break;
        }

        process_info = core::ptr::read::<SystemProcessInformation>(
            buffer.as_ptr().offset(current_offset).cast(),
        );
    }

    let mut nt_process_info_array: Vec<SystemProcessInfo> = Vec::<SystemProcessInfo>::new();

    for p in process_info_array {
        nt_process_info_array.push(SystemProcessInfo {
            thread_count: p.number_of_threads,
            name: String::from_utf16_lossy(std::slice::from_raw_parts(
                p.image_name.buffer,
                (p.image_name.length) as usize / 2,
            ))
            .replace("\0", ""),
            base_priority_class: p.base_priority,
            id: p.unique_process_id,
            handle_count: p.handle_count,
            session_id: p.session_id,
            peak_virtual_size: p.peak_virtual_size,
            virtual_size: p.virtual_size,
            peak_working_set_size: p.peak_working_set_size,
            working_set_size: p.working_set_size,
            quota_paged_pool_usage: p.quota_paged_pool_usage,
            quota_non_paged_pool_usage: p.quota_non_paged_pool_usage,
            pagefile_usage: p.pagefile_usage,
            peak_pagefile_usage: p.peak_pagefile_usage,
            private_page_count: p.private_page_count,
        });
    }

    Ok(nt_process_info_array)
}

pub(crate) unsafe fn alloc_console() -> crate::Result<()> {
    if AllocConsole() == 0 {
        return Err("The function AllocConsole failed".to_string());
    }

    Ok(())
}

pub(crate) unsafe fn free_console() -> crate::Result<()> {
    if FreeConsole() == 0 {
        return Err("The function FreeConsole failed".to_string());
    }

    Ok(())
}

pub(crate) unsafe fn set_console_mode(
    standard_handle: u32,
    console_mode: u32,
) -> crate::Result<()> {
    let standard_handle: *mut core::ffi::c_void = GetStdHandle(standard_handle);

    if standard_handle == -1isize as *mut core::ffi::c_void {
        return Err("The function GetStdHandle failed".to_string());
    }

    let mut current_console_mode: u32 = 0;

    if GetConsoleMode(standard_handle, &mut current_console_mode) == 0 {
        return Err("The function GetConsoleMode failed".to_string());
    }

    if 0 == SetConsoleMode(standard_handle, current_console_mode | console_mode) {
        return Err("The function SetConsoleMode failed".to_string());
    }

    Ok(())
}

// fn get_last_error_message() -> crate::Result<String> {
//     unsafe {
//         let language_id = ((0 as u32) << 10) | (0x01 as u32);

//         let mut buffer: *mut u16 = core::ptr::null_mut();
//         let buffer_size = FormatMessageW(
//             0x00000100 | 0x00001000 | 0x00000200,
//             core::ptr::null(),
//             GetLastError(),
//             language_id,
//             &mut buffer as *mut _ as *mut u16,
//             0,
//             core::ptr::null_mut(),
//         );

//         if buffer_size <= 0 {
//             return Err("FormatMessageW failed".to_string());
//         }

//         Ok(String::from_utf16_lossy(core::slice::from_raw_parts(
//             buffer,
//             buffer_size as usize,
//         )))
//     }
// }
