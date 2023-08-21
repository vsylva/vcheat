use crate::*;

pub fn get_all_processes_info() -> Result<Vec<ProcessInfo>> {
    unsafe {
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

        process_entry_array.push(process_entry.read());

        while Process32NextW(snapshot_handle, process_entry) != 0 {
            process_entry_array.push(process_entry.read());
        }

        CloseHandle(snapshot_handle);

        let mut process_info_array = Vec::<ProcessInfo>::new();

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
}

pub fn get_process_handle(process_id: u32) -> Result<*mut core::ffi::c_void> {
    unsafe {
        let process_handle = OpenProcess(0x000F0000 | 0x00100000 | 0xFFFF, 0, process_id);

        if process_handle.is_null() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("OpenProcess failed with return value: {process_handle:X?}"),
            )));
        }

        Ok(process_handle)
    }
}

/// The function is not stable, but it provides more information compared to non-NT functions
pub fn nt_get_all_processes_info() -> Result<Vec<SystemProcessInfo>> {
    unsafe {
        let mut return_length: u32 = 0;

        NtQuerySystemInformation(5, core::ptr::null_mut(), 0, &mut return_length);

        return_length *= 2;

        let mut buffer = Vec::<u8>::with_capacity(return_length as usize);
        buffer.set_len(return_length as usize);

        let result = NtQuerySystemInformation(
            5,
            buffer.as_mut_ptr().cast(),
            buffer.capacity() as u32,
            &mut return_length,
        );

        if result != 0 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("NtQuerySystemInformation failed with return value: {result:X}"),
            )));
        }

        let mut process_info_array = Vec::<SystemProcessInformation>::new();

        let mut current_offset: u32 = 0;

        let mut process_info = core::ptr::read::<SystemProcessInformation>(
            buffer.as_ptr().offset(current_offset as isize).cast(),
        );

        while process_info.next_entry_offset != 0 {
            if process_info.unique_process_id != 0 {
                process_info_array.push(process_info);
            }

            current_offset += process_info.next_entry_offset;

            process_info = core::ptr::read::<SystemProcessInformation>(
                buffer.as_ptr().offset(current_offset as isize).cast(),
            );
        }

        let mut nt_process_info_array = Vec::<SystemProcessInfo>::new();

        for p in process_info_array {
            nt_process_info_array.push(SystemProcessInfo {
                process_thread_count: p.number_of_threads,
                process_name: String::from_utf16_lossy(std::slice::from_raw_parts(
                    p.image_name.buffer,
                    (p.image_name.length) as usize / 2,
                )),
                process_base_priority_class: p.base_priority,
                process_id: p.unique_process_id as u32,
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
}
