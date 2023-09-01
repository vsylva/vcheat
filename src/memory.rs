use crate::*;

/// Query Memory Protection Constants
/// <https://learn.microsoft.com/en-us/windows/win32/Memory/memory-protection-constants>
pub fn virtual_protect(
    process_id: u32,
    address: *const core::ffi::c_void,
    new_protect: u32,
) -> Result<u32> {
    unsafe {
        let process_handle: *mut core::ffi::c_void = open_process_handle(process_id)?;

        let result: usize = VirtualQueryEx(
            process_handle,
            address,
            &mut core::mem::zeroed::<MemoryBasicInformation>(),
            core::mem::size_of::<MemoryBasicInformation>(),
        );

        if result == 0 {
            close_handle(process_handle)?;
            return Err(format!(
                "VirtualQueryEx failed with return value: {result:X}"
            ));
        }

        let mut old_protect: u32 = 0;

        // let mut new_protect_num: u32 = 0;

        // for protect in new_protect {
        //     new_protect_num |= match protect {
        //         ProtectType::PageExecute => 0x10,
        //         ProtectType::PageExecuteRead => 0x20,
        //         ProtectType::PageExecuteReadwrite => 0x40,
        //         ProtectType::PageExecuteWritecopy => 0x80,
        //         ProtectType::PageNoaccess => 1,
        //         ProtectType::PageReadonly => 2,
        //         ProtectType::PageReadwrite => 4,
        //         ProtectType::PageWritecopy => 8,
        //         ProtectType::PageTargetsInvalidOrNoUpdate => 0x40000000,
        //         ProtectType::PageGuard => 0x100,
        //         ProtectType::PageNocache => 0x200,
        //         ProtectType::PageWritecombine => 0x400,
        //     };
        // }

        let result: i32 = VirtualProtectEx(
            process_handle,
            address,
            core::mem::size_of::<*mut core::ffi::c_void>(),
            new_protect,
            // new_protect_num,
            &mut old_protect,
        );

        if result == 0 {
            close_handle(process_handle)?;
            return Err(format!(
                "VirtualProtectEx failed with return value: {result:X}"
            ));
        }

        close_handle(process_handle)?;

        Ok(old_protect)
    }
}

/// Some of the code in the function is based on S3pt3mb3r's code from
/// <https://github.com/pseuxide/toy-arms/blob/master/external/src/lib.rs>
/// <https://github.com/pseuxide/toy-arms/blob/master/toy-arms_utils/src/pattern_scan.rs>
pub fn read_process_memory(
    process_id: u32,
    address: *const core::ffi::c_void,
    size: usize,
) -> Result<Vec<u8>> {
    unsafe {
        let process_handle: *mut core::ffi::c_void = open_process_handle(process_id)?;

        let memory_basic_info: &mut MemoryBasicInformation = &mut MemoryBasicInformation {
            ..core::mem::zeroed()
        };

        let result: usize = VirtualQueryEx(
            process_handle,
            address,
            memory_basic_info,
            core::mem::size_of::<MemoryBasicInformation>(),
        );

        if result == 0 {
            close_handle(process_handle)?;
            return Err(format!(
                "VirtualQueryEx failed with return value: {result:X}"
            ));
        }

        let is_page_readable: bool = if memory_basic_info.state == 0x1000
            && memory_basic_info.protect & (0x02 | 0x04 | 0x20 | 0x40) != 0
        {
            true
        } else {
            false
        };

        let mut old_protect: u32 = 0;

        let mut new_protect: u32 = 0x04;

        if !is_page_readable {
            let result: i32 = VirtualProtectEx(
                process_handle,
                address,
                core::mem::size_of::<*mut core::ffi::c_void>(),
                new_protect,
                &mut old_protect,
            );

            if result == 0 {
                close_handle(process_handle)?;
                return Err(format!(
                    "VirtualProtectEx failed with return value: {result:X}"
                ));
            }
        }

        let mut buffer: Vec<u8> = vec![0; size];
        let mut bytes_read: usize = 0;
        let result: i32 = ReadProcessMemory(
            process_handle,
            address,
            buffer.as_mut_ptr().cast(),
            size,
            &mut bytes_read,
        );

        if bytes_read != size {
            return Err(format!(
                "lpNumberOfBytesRead: {:X} is not equal to size: {:X})",
                bytes_read, size
            ));
        }

        if result == 0 {
            close_handle(process_handle)?;
            return Err(format!(
                "ReadProcessMemory failed with return value: {result:X}"
            ));
        }

        if !is_page_readable {
            let result: i32 = VirtualProtectEx(
                process_handle,
                address,
                core::mem::size_of::<*mut core::ffi::c_void>(),
                old_protect,
                &mut new_protect,
            );

            if result == 0 {
                close_handle(process_handle)?;
                return Err(format!(
                    "VirtualProtectEx failed with return value: {result:X}"
                ));
            }
        }

        close_handle(process_handle)?;

        Ok(buffer)
    }
}

/// Some of the code in the function is based on S3pt3mb3r's code from
/// <https://github.com/pseuxide/toy-arms/blob/master/external/src/lib.rs>
/// <https://github.com/pseuxide/toy-arms/blob/master/toy-arms_utils/src/pattern_scan.rs>
pub fn write_process_memory(
    process_id: u32,
    address: *mut core::ffi::c_void,
    data: &[u8],
) -> Result<usize> {
    unsafe {
        let process_handle: *mut core::ffi::c_void = open_process_handle(process_id)?;

        let memory_basic_info: &mut MemoryBasicInformation = &mut MemoryBasicInformation {
            ..core::mem::zeroed()
        };

        let result: usize = VirtualQueryEx(
            process_handle,
            address,
            memory_basic_info,
            core::mem::size_of::<MemoryBasicInformation>(),
        );

        if result == 0 {
            close_handle(process_handle)?;
            return Err(format!(
                "VirtualQueryEx failed with return value: {result:X}"
            ));
        }

        let is_page_writeable: bool = if memory_basic_info.state == 0x1000
            && memory_basic_info.protect & (0x04 | 0x40) != 0
        {
            true
        } else {
            false
        };

        let mut old_protect: u32 = 0;

        let mut new_protect: u32 = 0x04;

        if !is_page_writeable {
            let result: i32 = VirtualProtectEx(
                process_handle,
                address,
                core::mem::size_of::<*mut core::ffi::c_void>(),
                new_protect,
                &mut old_protect,
            );

            if result == 0 {
                close_handle(process_handle)?;
                return Err(format!(
                    "VirtualProtectEx failed with return value: {result:X}"
                ));
            }
        }

        let mut number_of_bytes_written: usize = 0;
        let result: i32 = WriteProcessMemory(
            process_handle,
            address,
            data.as_ptr().cast(),
            data.len(),
            &mut number_of_bytes_written,
        );
        if result == 0 {
            close_handle(process_handle)?;
            return Err(format!(
                "WriteProcessMemory failed with return value: {result:X}"
            ));
        }

        if !is_page_writeable {
            let result: i32 = VirtualProtectEx(
                process_handle,
                address,
                core::mem::size_of::<*mut core::ffi::c_void>(),
                old_protect,
                &mut new_protect,
            );

            if result == 0 {
                close_handle(process_handle)?;
                return Err(format!(
                    "VirtualProtectEx failed with return value: {result:X}"
                ));
            }
        }

        close_handle(process_handle)?;

        Ok(number_of_bytes_written)
    }
}

/// Some of the code in the function is based on sonodima's code from
/// <https://github.com/sonodima/aobscan/blob/master/src/builder.rs>
/// <https://github.com/sonodima/aobscan/blob/master/src/pattern.rs>
pub fn aob_scan_single_threaded(
    pattern: &str,
    data: &[u8],
    return_on_first: bool,
) -> Result<Vec<usize>> {
    let mut signature: Vec<u8> = vec![];
    let mut mask: Vec<bool> = vec![];

    for pair in pattern.split_whitespace() {
        if pair == "?" || pair == "??" {
            mask.push(false);
            signature.push(0);
        } else {
            let number: u8 = match u8::from_str_radix(pair, 16) {
                Ok(ok) => ok,
                Err(err) => return Err(err.to_string()),
            };
            mask.push(true);
            signature.push(number);
        }
    }

    let mut start_offset: usize = mask.iter().take_while(|&&x| x == false).count();
    let end_offset: usize = mask.iter().rev().take_while(|&&x| x == false).count();

    if start_offset != mask.len() {
        signature = signature[start_offset..signature.len() - end_offset].to_vec();
        mask = mask[start_offset..mask.len() - end_offset].to_vec();
    } else {
        start_offset = 0;
    }

    let first_byte: u8 = signature[0];
    let first_mask: bool = mask[0];

    let mut offset_array: Vec<usize> = Vec::new();

    for i in 0..data.len() - signature.len() {
        if data[i] != first_byte && first_mask {
            continue;
        }

        if {
            let data: &[u8] = &data[i..];
            let mut status: bool = true;
            for (i, sig) in signature.iter().enumerate() {
                if !mask[i] {
                    continue;
                }

                if data[i] != *sig {
                    status = false;
                    break;
                }
            }
            status
        } {
            offset_array.push(i - start_offset);
            if return_on_first {
                break;
            }
        }
    }
    offset_array.sort();
    Ok(offset_array)
}

/// Some of the code in the function is based on sonodima's code from
/// <https://github.com/sonodima/aobscan/blob/master/src/builder.rs>
/// <https://github.com/sonodima/aobscan/blob/master/src/pattern.rs>
pub fn aob_scan_multi_threaded(
    pattern: &str,
    data: &[u8],
    return_on_first: bool,
    thread_count: usize,
) -> Result<Vec<usize>> {
    if pattern.is_empty() {
        return Err("Pattern cannot be empty".to_string());
    }

    if data.len() == 0 {
        return Err("Data cannot be empty".to_string());
    }

    if thread_count < 2 {
        return Err("Thread count must be greater than one".to_string());
    }

    let mut signature: Vec<u8> = Vec::<u8>::new();
    let mut mask: Vec<bool> = Vec::<bool>::new();

    for pair in pattern.split_whitespace() {
        if pair == "?" || pair == "??" {
            mask.push(false);
            signature.push(0);
        } else {
            let number: u8 = match u8::from_str_radix(pair, 16) {
                Ok(ok) => ok,
                Err(err) => return Err(err.to_string()),
            };
            mask.push(true);
            signature.push(number);
        }
    }

    let mut start_offset: usize = mask.iter().take_while(|&&x| x == false).count();
    let end_offset: usize = mask.iter().rev().take_while(|&&x| x == false).count();

    if start_offset != mask.len() {
        signature = signature[start_offset..signature.len() - end_offset].to_vec();
        mask = mask[start_offset..mask.len() - end_offset].to_vec();
    } else {
        start_offset = 0;
    }

    let running_thread_count: std::sync::Arc<std::sync::atomic::AtomicUsize> =
        std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let found: std::sync::Arc<std::sync::atomic::AtomicBool> =
        std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

    let finished: std::sync::Arc<std::sync::atomic::AtomicBool> =
        std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

    let offset_array: std::sync::Arc<std::sync::Mutex<Vec<usize>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Vec::<usize>::new()));

    let signature: &Vec<u8> = &signature;
    let mask: &Vec<bool> = &mask;

    std::thread::scope(|scope| {
        for index in 0..thread_count {
            let range: (usize, usize) = {
                let data_size: usize = data.len();
                let chunks: usize = thread_count;
                let overlap: usize = signature.len() - 1;
                let chunk_size: usize = data_size / chunks;
                let remainder: usize = data_size % chunks;

                let start: usize = index * chunk_size;

                let mut end: usize =
                    start + chunk_size + if index == chunks - 1 { remainder } else { 0 };

                let start: usize = start - if start >= overlap { overlap } else { 0 };

                end = end
                    + if end < data_size - overlap {
                        overlap
                    } else {
                        0
                    };

                (start, end)
            };

            let running_thread_count: std::sync::Arc<std::sync::atomic::AtomicUsize> =
                running_thread_count.clone();
            let finished: std::sync::Arc<std::sync::atomic::AtomicBool> = finished.clone();
            let found: std::sync::Arc<std::sync::atomic::AtomicBool> = found.clone();

            running_thread_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

            let addres_array: std::sync::Arc<std::sync::Mutex<Vec<usize>>> = offset_array.clone();

            scope.spawn(move || {
                let data: &[u8] = &data[range.0..range.1];

                let length: usize = data.len() - signature.len();

                let first_byte: u8 = signature[0];
                let first_mask: bool = mask[0];

                let mut found_in: bool = false;

                for i in 0..length {
                    if finished.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    if data[i] != first_byte && first_mask {
                        continue;
                    }

                    if {
                        let data: &[u8] = &data[i..];
                        let mut status: bool = true;
                        for (i, sig) in signature.iter().enumerate() {
                            if !mask[i] {
                                continue;
                            }

                            if data[i] != *sig {
                                status = false;
                                break;
                            }
                        }
                        status
                    } {
                        found_in = true;
                        if let Ok(mut val) = addres_array.lock() {
                            val.push(range.0 + i - start_offset);
                        } else {
                            return;
                        }
                        if return_on_first {
                            finished.store(true, std::sync::atomic::Ordering::Relaxed);
                            break;
                        }
                    }
                }

                if found_in {
                    found.store(true, std::sync::atomic::Ordering::SeqCst);
                }

                running_thread_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            });
        }
    });

    while running_thread_count.load(std::sync::atomic::Ordering::SeqCst) != 0 {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    found.load(std::sync::atomic::Ordering::SeqCst);
    let mut offset_array: Vec<usize> = if let Ok(val) = offset_array.lock() {
        val.to_vec()
    } else {
        return Err("Mutex lock failed".to_string());
    };
    offset_array.sort();
    Ok(offset_array)
}
