use crate::{ffi::*, types::*};

type Result<T> = std::result::Result<T, String>;

pub(crate) unsafe fn open_process_handle(process_id: u32) -> Result<*mut core::ffi::c_void> {
    let process_handle: *mut core::ffi::c_void = OpenProcess(0x1F0FFF, 0, process_id);

    if process_handle.is_null() {
        return Err(format!(
            "OpenProcess failed with result: {process_handle:X?}"
        ));
    }

    Ok(process_handle)
}

pub(crate) unsafe fn close_handle(handle: *mut core::ffi::c_void) -> Result<()> {
    if !handle.is_null() {
        if CloseHandle(handle) != 0 {
            Ok(())
        } else {
            Err("CloseHandle failed".to_string())
        }
    } else {
        Err("The handle is null, no need to close".to_string())
    }
}

#[cfg(target_arch = "x86")]
pub(crate) unsafe fn is_wow64_process(process_id: u32) -> Result<bool> {
    let process_handle: *mut core::ffi::c_void = open_process_handle(process_id)?;

    let mut is_wow64: i32 = 0;

    let result: i32 = IsWow64Process(process_handle, &mut is_wow64);

    close_handle(process_handle)?;

    if result == 0 {
        return Err(format!("IsWow64Process failed with result: {result:X?}"));
    }

    if is_wow64 != 0 {
        return Ok(true);
    }

    Ok(false)
}

pub(crate) unsafe fn get_all_processes_info() -> Result<Vec<ProcessInfo>> {
    let snapshot_handle: *mut core::ffi::c_void = CreateToolhelp32Snapshot(0x2, 0);

    if snapshot_handle.is_null() {
        return Err(format!(
            "CreateToolhelp32Snapshot failed with result: {snapshot_handle:X?}"
        ));
    }

    let process_entry: &mut ProcessEntry32W = &mut core::mem::zeroed::<ProcessEntry32W>();

    process_entry.dw_size = core::mem::size_of::<ProcessEntry32W>() as u32;

    let result: i32 = Process32FirstW(snapshot_handle, process_entry);

    if result == 0 {
        close_handle(snapshot_handle)?;
        return Err(format!("Process32FirstW failed with result: {result:X}"));
    }

    let mut process_entry_array: Vec<ProcessEntry32W> = Vec::<ProcessEntry32W>::new();

    process_entry_array.push(process_entry.clone());

    while Process32NextW(snapshot_handle, process_entry) != 0 {
        process_entry_array.push(process_entry.clone());
    }

    if !snapshot_handle.is_null() {
        close_handle(snapshot_handle)?
    }

    let mut process_info_array: Vec<ProcessInfo> = Vec::<ProcessInfo>::new();

    for p in process_entry_array {
        process_info_array.push(ProcessInfo {
            process_id: p.th32_process_id,
            process_thread_count: p.cnt_threads,
            process_parent_process_id: p.th32_parent_process_id,
            process_base_priority_class: p.pc_pri_class_base,
            process_name: core::ffi::CStr::from_ptr(
                String::from_utf16_lossy(p.sz_exe_file.as_ref())
                    .as_ptr()
                    .cast(),
            )
            .to_string_lossy()
            .to_string(),
        });
    }

    Ok(process_info_array)
}

/// The function is not stable, but it provides more information compared to non-NT functions
pub(crate) unsafe fn nt_get_all_processes_info() -> Result<Vec<SystemProcessInfo>> {
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
            "NtQuerySystemInformation failed with result: {result:X}"
        ));
    }

    let mut process_info_array: Vec<SystemProcessInformation> =
        Vec::<SystemProcessInformation>::new();

    let mut current_offset: isize = 0;

    let mut process_info: SystemProcessInformation =
        core::ptr::read::<SystemProcessInformation>(buffer.as_ptr().offset(current_offset).cast());

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
            process_thread_count: p.number_of_threads,
            process_name: String::from_utf16_lossy(std::slice::from_raw_parts(
                p.image_name.buffer,
                (p.image_name.length) as usize / 2,
            )),
            process_base_priority_class: p.base_priority,
            process_id: p.unique_process_id,
            process_handle_count: p.handle_count,
            process_session_id: p.session_id,
            process_peak_virtual_size: p.peak_virtual_size,
            process_virtual_size: p.virtual_size,
            process_peak_working_set_size: p.peak_working_set_size,
            process_working_set_size: p.working_set_size,
            process_quota_paged_pool_usage: p.quota_paged_pool_usage,
            process_quota_non_paged_pool_usage: p.quota_non_paged_pool_usage,
            process_pagefile_usage: p.pagefile_usage,
            process_peak_pagefile_usage: p.peak_pagefile_usage,
            process_private_page_count: p.private_page_count,
        });
    }

    Ok(nt_process_info_array)
}

pub(crate) unsafe fn alloc_console() -> Result<()> {
    if AllocConsole() == 0 {
        return Err("AllocConsole failed".to_string());
    }
    Ok(())
}

pub(crate) unsafe fn free_console() -> Result<()> {
    if FreeConsole() == 0 {
        return Err("FreeConsole failed".to_string());
    }
    Ok(())
}

pub(crate) unsafe fn set_console_mode(standard_handle: u32, console_mode: u32) -> Result<()> {
    let standard_handle: *mut core::ffi::c_void = GetStdHandle(standard_handle);

    if standard_handle == -1isize as *mut core::ffi::c_void {
        return Err("GetStdHandle failed".to_string());
    }

    let mut current_console_mode: u32 = 0;

    if GetConsoleMode(standard_handle, &mut current_console_mode) == 0 {
        return Err("GetConsoleModeType failed".to_string());
    }

    if 0 == SetConsoleMode(standard_handle, current_console_mode | console_mode) {
        return Err("SetConsoleModeType failed".to_string());
    }

    Ok(())
}

// fn get_last_error_message() -> Result<String> {
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
