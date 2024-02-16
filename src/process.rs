use unsafe_fn_body::unsafe_fn_body;

#[unsafe_fn_body::unsafe_fn_body]
pub fn open_process(process_id: u32) -> Result<*mut ::core::ffi::c_void, String> {
    let process_handle: *mut ::core::ffi::c_void = crate::ffi::OpenProcess(0x1F0FFF, 0, process_id);

    if process_handle.is_null() {
        return Err(crate::location!());
    }

    Ok(process_handle)
}

#[unsafe_fn_body]
pub fn close_handle(handle: *mut ::core::ffi::c_void) -> Result<(), String> {
    if handle.is_null() {
        return Err(crate::location!());
    }

    let result = crate::ffi::CloseHandle(handle);

    if result == 0 {
        return Err(crate::location!());
    }

    Ok(())
}

#[unsafe_fn_body]
pub fn close_handle_unchecked(handle: *mut ::core::ffi::c_void) -> i32 {
    crate::ffi::CloseHandle(handle)
}

#[unsafe_fn_body]
pub fn is_wow64_process(process_handle: *mut ::core::ffi::c_void) -> Result<bool, String> {
    if process_handle.is_null() {
        return Err(crate::location!());
    }

    let mut is_wow64: i32 = 0;

    let result: i32 = crate::ffi::IsWow64Process(process_handle, &mut is_wow64);

    if result == 0 {
        return Err(crate::location!());
    }

    Ok(is_wow64 != 0)
}

#[unsafe_fn_body]
pub fn get_processes_info() -> Result<Vec<crate::ProcessInformation>, String> {
    let snapshot_handle: *mut ::core::ffi::c_void = crate::ffi::CreateToolhelp32Snapshot(0x2, 0x0);

    if snapshot_handle as i32 == -1 {
        return Err(crate::location!(snapshot_handle));
    }

    let mut process_entry: crate::ffi::ProcessEntry32W =
        ::core::mem::zeroed::<crate::ffi::ProcessEntry32W>();

    process_entry.dw_size = ::core::mem::size_of::<crate::ffi::ProcessEntry32W>() as u32;

    let result: i32 = crate::ffi::Process32FirstW(snapshot_handle, &mut process_entry);

    if result == 0 {
        close_handle(snapshot_handle)?;

        return Err(crate::location!());
    }

    let mut process_entry_array: Vec<crate::ffi::ProcessEntry32W> =
        Vec::<crate::ffi::ProcessEntry32W>::new();

    process_entry_array.push(process_entry.to_owned());

    while crate::ffi::Process32NextW(snapshot_handle, &mut process_entry) != 0 {
        process_entry_array.push(process_entry.to_owned());
    }

    close_handle(snapshot_handle)?;

    let mut process_info_array: Vec<crate::ProcessInformation> =
        Vec::<crate::ProcessInformation>::new();

    for process_entry in process_entry_array {
        process_info_array.push(crate::ProcessInformation {
            id: process_entry.th32_process_id,
            thread_count: process_entry.cnt_threads,
            parent_process_id: process_entry.th32_parent_process_id,
            base_priority_class: process_entry.pc_pri_class_base,
            name: {
                let result: ::std::ffi::OsString =
                    ::std::os::windows::prelude::OsStringExt::from_wide(&process_entry.sz_exe_file);

                match result.to_str() {
                    Some(some) => some.trim_end_matches('\0').to_string(),

                    None => {
                        close_handle(snapshot_handle)?;

                        return Err(crate::location!());
                    }
                }
            },
        });
    }

    Ok(process_info_array)
}

#[unsafe_fn_body]
pub fn get_process_info(process_name: &str) -> Result<crate::ProcessInformation, String> {
    if process_name.is_empty() {
        return Err(crate::location!());
    }

    let process_name: String = process_name.to_lowercase();

    let snapshot_handle: *mut ::core::ffi::c_void = crate::ffi::CreateToolhelp32Snapshot(0x2, 0x0);

    if snapshot_handle as i32 == -1 {
        return Err(crate::location!(snapshot_handle));
    }

    let mut process_entry: crate::ffi::ProcessEntry32W =
        ::core::mem::zeroed::<crate::ffi::ProcessEntry32W>();

    process_entry.dw_size = ::core::mem::size_of::<crate::ffi::ProcessEntry32W>() as u32;

    let result: i32 = crate::ffi::Process32FirstW(snapshot_handle, &mut process_entry);

    if result == 0 {
        close_handle(snapshot_handle)?;

        return Err(crate::location!());
    }

    let process_entry_name: String = {
        let result: ::std::ffi::OsString =
            ::std::os::windows::prelude::OsStringExt::from_wide(&process_entry.sz_exe_file);

        match result.to_str() {
            Some(some) => some.trim_end_matches('\0').to_string(),

            None => {
                close_handle(snapshot_handle)?;

                return Err(crate::location!());
            }
        }
    };

    if process_entry_name.to_lowercase() == process_name {
        close_handle(snapshot_handle)?;

        return Ok(crate::ProcessInformation {
            id: process_entry.th32_process_id,
            thread_count: process_entry.cnt_threads,
            parent_process_id: process_entry.th32_parent_process_id,
            base_priority_class: process_entry.pc_pri_class_base,
            name: process_entry_name,
        });
    }

    while crate::ffi::Process32NextW(snapshot_handle, &mut process_entry) != 0 {
        let process_entry_name: String = {
            let result: ::std::ffi::OsString =
                ::std::os::windows::prelude::OsStringExt::from_wide(&process_entry.sz_exe_file);

            match result.to_str() {
                Some(some) => some.trim_end_matches('\0').to_string(),
                None => return Err(crate::location!()),
            }
        };

        if process_entry_name.to_lowercase() == process_name {
            close_handle(snapshot_handle)?;

            return Ok(crate::ProcessInformation {
                id: process_entry.th32_process_id,
                thread_count: process_entry.cnt_threads,
                parent_process_id: process_entry.th32_parent_process_id,
                base_priority_class: process_entry.pc_pri_class_base,
                name: process_entry_name,
            });
        }
    }

    close_handle(snapshot_handle)?;

    Err(crate::location!())
}

#[unsafe_fn_body]
pub fn nt_get_processes_info() -> Result<Vec<crate::SystemProcessInformation>, String> {
    let mut return_length: u32 = 0;

    crate::ffi::NtQuerySystemInformation(5, ::core::ptr::null_mut(), 0, &mut return_length);

    let mut buffer: Vec<u8> = vec![0; return_length as usize];

    let result: i32 = crate::ffi::NtQuerySystemInformation(
        5,
        buffer.as_mut_ptr().cast(),
        return_length,
        &mut ::core::mem::zeroed::<u32>(),
    );

    if result != 0 {
        return Err(crate::location!(result));
    }

    let mut system_process_info_array: Vec<crate::ffi::SystemProcessInformation> =
        Vec::<crate::ffi::SystemProcessInformation>::new();

    let mut next_ptr = buffer.as_ptr();

    let mut system_process_info: crate::ffi::SystemProcessInformation =
        ::core::ptr::read::<crate::ffi::SystemProcessInformation>(next_ptr.cast());

    loop {
        if system_process_info.next_entry_offset == 0 {
            break;
        }

        if system_process_info.unique_process_id != 0 {
            system_process_info_array.push(system_process_info.clone());
        }

        next_ptr = next_ptr.offset(system_process_info.next_entry_offset as isize);

        if next_ptr.is_null() {
            break;
        }

        system_process_info =
            ::core::ptr::read::<crate::ffi::SystemProcessInformation>(next_ptr.cast());
    }

    let mut nt_system_process_info_array: Vec<crate::SystemProcessInformation> =
        Vec::<crate::SystemProcessInformation>::new();

    for spi in system_process_info_array {
        let nt_system_process_info = crate::SystemProcessInformation {
            thread_count: spi.number_of_threads,
            name: {
                let name_data: &[u16] = ::std::slice::from_raw_parts(
                    spi.image_name.buffer,
                    (spi.image_name.length) as usize / 2,
                );

                let result: ::std::ffi::OsString =
                    ::std::os::windows::prelude::OsStringExt::from_wide(&name_data);

                match result.to_str() {
                    Some(some) => some.trim_end_matches('\0').to_string(),
                    None => return Err(crate::location!()),
                }
            },
            base_priority_class: spi.base_priority,
            id: spi.unique_process_id,
            handle_count: spi.handle_count,
            session_id: spi.session_id,
            peak_virtual_size: spi.peak_virtual_size,
            virtual_size: spi.virtual_size,
            peak_working_set_size: spi.peak_working_set_size,
            working_set_size: spi.working_set_size,
            quota_paged_pool_usage: spi.quota_paged_pool_usage,
            quota_non_paged_pool_usage: spi.quota_non_paged_pool_usage,
            pagefile_usage: spi.pagefile_usage,
            peak_pagefile_usage: spi.peak_pagefile_usage,
            private_page_count: spi.private_page_count,
        };

        nt_system_process_info_array.push(nt_system_process_info);
    }

    Ok(nt_system_process_info_array)
}

#[unsafe_fn_body]
pub fn alloc_console() -> Result<(), String> {
    if crate::ffi::AllocConsole() == 0 {
        return Err(crate::location!());
    }

    Ok(())
}

#[unsafe_fn_body]
pub fn alloc_console_unchecked() -> i32 {
    crate::ffi::AllocConsole()
}

#[unsafe_fn_body]
pub fn free_console() -> Result<(), String> {
    if crate::ffi::FreeConsole() == 0 {
        return Err(crate::location!());
    }

    Ok(())
}

#[unsafe_fn_body]
pub fn free_console_unchecked() -> i32 {
    crate::ffi::FreeConsole()
}

#[unsafe_fn_body]
pub fn set_console_mode(standard_handle: u32, console_mode: u32) -> Result<(), String> {
    let standard_handle: *mut ::core::ffi::c_void = crate::ffi::GetStdHandle(standard_handle);

    if standard_handle as isize == -1 {
        return Err(crate::location!());
    }

    let mut current_console_mode: u32 = 0;

    if 0 == crate::ffi::GetConsoleMode(standard_handle, &mut current_console_mode) {
        return Err(crate::location!());
    }

    if 0 == crate::ffi::SetConsoleMode(standard_handle, current_console_mode | console_mode) {
        return Err(crate::location!());
    }

    Ok(())
}

#[unsafe_fn_body]
pub fn set_console_colors() -> Result<(), String> {
    set_console_mode(
        crate::consts::standard_handle::OUTPUT_HANDLE,
        crate::consts::console_mode::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    )
}
