#[unsafe_fn_body::unsafe_fn_body]
pub fn read_process_memory(
    process_handle: *mut ::core::ffi::c_void,
    address: *const ::core::ffi::c_void,
    size: usize,
) -> Result<Vec<u8>, String> {
    if process_handle.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if size == 0 {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let mut memory_basic_info: crate::ffi::MemoryBasicInformation =
        ::core::mem::zeroed::<crate::ffi::MemoryBasicInformation>();

    if 0 == crate::ffi::VirtualQueryEx(
        process_handle,
        address,
        &mut memory_basic_info,
        ::core::mem::size_of::<crate::ffi::MemoryBasicInformation>(),
    ) {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let mut is_page_readable: bool = false;

    if memory_basic_info.state == 0x1000
        && memory_basic_info.protect & (0x02 | 0x04 | 0x20 | 0x40) != 0
    {
        is_page_readable = true;
    }

    let mut old_page_protect: u32 = 0;

    let mut new_page_protect: u32 = 0x04;

    if !is_page_readable {
        if 0 == crate::ffi::VirtualProtectEx(
            process_handle,
            address,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>(),
            new_page_protect,
            &mut old_page_protect,
        ) {
            return Err(format!("[{}:{}]", file!(), line!()));
        }
    }

    let mut buffer: Vec<u8> = vec![0; size];

    let mut number_of_bytes_read: usize = 0;

    if 0 == crate::ffi::ReadProcessMemory(
        process_handle,
        address,
        buffer.as_mut_ptr().cast(),
        size,
        &mut number_of_bytes_read,
    ) {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if number_of_bytes_read != size {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if !is_page_readable {
        if 0 == crate::ffi::VirtualProtectEx(
            process_handle,
            address,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>(),
            old_page_protect,
            &mut new_page_protect,
        ) {
            return Err(format!("[{}:{}]", file!(), line!()));
        }
    }

    Ok(buffer)
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn read_process_memory_unchecked(
    process_handle: *mut ::core::ffi::c_void,
    address: *const ::core::ffi::c_void,
    size: usize,
) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; size];

    crate::ffi::ReadProcessMemory(
        process_handle,
        address,
        buffer.as_mut_ptr().cast(),
        size,
        ::core::ptr::null_mut(),
    );

    buffer
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn write_process_memory<T>(
    process_handle: *mut ::core::ffi::c_void,
    address: *mut ::core::ffi::c_void,
    data: &[T],
) -> Result<usize, String> {
    if process_handle.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if data.is_empty() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let mut memory_basic_info: crate::ffi::MemoryBasicInformation =
        ::core::mem::zeroed::<crate::ffi::MemoryBasicInformation>();

    if 0 == crate::ffi::VirtualQueryEx(
        process_handle,
        address,
        &mut memory_basic_info,
        ::core::mem::size_of::<crate::ffi::MemoryBasicInformation>(),
    ) {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let mut is_page_writeable: bool = false;

    if memory_basic_info.state == 0x1000 && memory_basic_info.protect & (0x04 | 0x40) != 0 {
        is_page_writeable = true;
    };

    let mut old_page_protect: u32 = 0;

    let mut new_page_protect: u32 = 0x04;

    if !is_page_writeable {
        if 0 == crate::ffi::VirtualProtectEx(
            process_handle,
            address,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>(),
            new_page_protect,
            &mut old_page_protect,
        ) {
            return Err(format!("[{}:{}]", file!(), line!()));
        }
    }

    let mut number_of_bytes_written: usize = 0;

    let size: usize = data.len() * ::core::mem::size_of::<T>();

    if 0 == crate::ffi::WriteProcessMemory(
        process_handle,
        address,
        data.as_ptr().cast(),
        size,
        &mut number_of_bytes_written,
    ) {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if number_of_bytes_written != size {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if !is_page_writeable {
        if 0 == crate::ffi::VirtualProtectEx(
            process_handle,
            address,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>(),
            old_page_protect,
            &mut new_page_protect,
        ) {
            return Err(format!("[{}:{}]", file!(), line!()));
        }
    }

    Ok(number_of_bytes_written)
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn write_process_memory_unchecked<T>(
    process_handle: *mut ::core::ffi::c_void,
    address: *mut ::core::ffi::c_void,
    data: &[T],
) {
    crate::ffi::WriteProcessMemory(
        process_handle,
        address,
        data.as_ptr().cast(),
        data.len() * ::core::mem::size_of::<T>(),
        ::core::ptr::null_mut(),
    );
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn aob_scan_single_threaded(
    pattern: &str,
    data: &[u8],
    stop_on_first: bool,
) -> Result<Option<Vec<usize>>, String> {
    if pattern.is_empty() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if data.is_empty() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let mut signature: Vec<u8> = Vec::<u8>::new();

    let mut mask: Vec<bool> = Vec::<bool>::new();

    for pair in pattern.split_whitespace() {
        if pair == "?" || pair == "??" {
            mask.push(false);

            signature.push(0);
        } else {
            let number: u8 = u8::from_str_radix(pair, 16).map_err(|err| err.to_string())?;

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

        let data: &[u8] = &data[i..];

        let mut found: bool = true;

        for (i, sig) in signature.iter().enumerate() {
            if !mask[i] {
                continue;
            }

            if data[i] != sig.to_owned() {
                found = false;

                break;
            }
        }

        if found {
            offset_array.push(i - start_offset);

            if stop_on_first {
                break;
            }
        }
    }

    if offset_array.is_empty() {
        return Ok(None);
    }

    Ok(Some(offset_array))
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn aob_scan_multi_threaded(
    pattern: &str,
    data: &[u8],
    stop_on_first: bool,
    thread_count: u32,
) -> Result<Vec<usize>, String> {
    if pattern.is_empty() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if data.is_empty() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if thread_count < 2 {
        return Err(format!("[{}:{}]\t\"{:?}\"", file!(), line!(), thread_count));
    }

    let thread_count: usize = thread_count as usize;

    let mut signature: Vec<u8> = Vec::<u8>::new();

    let mut mask: Vec<bool> = Vec::<bool>::new();

    for pair in pattern.split_whitespace() {
        if pair == "?" || pair == "??" {
            mask.push(false);

            signature.push(0);
        } else {
            let number: u8 = u8::from_str_radix(pair, 16).map_err(|err| err.to_string())?;

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

    let running_thread_count: ::std::sync::Arc<::std::sync::atomic::AtomicUsize> =
        ::std::sync::Arc::new(::std::sync::atomic::AtomicUsize::new(0));

    let found: ::std::sync::Arc<::std::sync::atomic::AtomicBool> =
        ::std::sync::Arc::new(::std::sync::atomic::AtomicBool::new(false));

    let finished: ::std::sync::Arc<::std::sync::atomic::AtomicBool> =
        ::std::sync::Arc::new(::std::sync::atomic::AtomicBool::new(false));

    let offset_array: ::std::sync::Arc<::std::sync::Mutex<Vec<usize>>> =
        ::std::sync::Arc::new(::std::sync::Mutex::new(Vec::<usize>::new()));

    let signature: &[u8] = signature.as_ref();

    let mask: &[bool] = mask.as_ref();

    ::std::thread::scope(|scope| {
        for index in 0..thread_count {
            let data_size: usize = data.len();

            let chunks: usize = thread_count;

            let overlap: usize = signature.len() - 1;

            let chunk_size: usize = data_size / chunks;

            let remainder: usize = data_size % chunks;

            let start: usize = index * chunk_size;

            let end: usize = start + chunk_size + if index == chunks - 1 { remainder } else { 0 };

            let range_start: usize = start - if start >= overlap { overlap } else { 0 };

            let range_end = end
                + if end < data_size - overlap {
                    overlap
                } else {
                    0
                };

            let running_thread_count: ::std::sync::Arc<::std::sync::atomic::AtomicUsize> =
                running_thread_count.clone();

            let finished: ::std::sync::Arc<::std::sync::atomic::AtomicBool> = finished.clone();

            let found: ::std::sync::Arc<::std::sync::atomic::AtomicBool> = found.clone();

            running_thread_count.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst);

            let offset_array: ::std::sync::Arc<::std::sync::Mutex<Vec<usize>>> =
                offset_array.clone();

            scope.spawn(move || {
                let data: &[u8] = &data[range_start..range_end];

                let length: usize = data.len() - signature.len();

                let first_byte: u8 = signature[0];

                let first_mask: bool = mask[0];

                let mut found_in_thread: bool = false;

                for i in 0..length {
                    if finished.load(::std::sync::atomic::Ordering::Relaxed) {
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
                        found_in_thread = true;

                        if let Ok(mut val) = offset_array.lock() {
                            val.push(range_start + i - start_offset);
                        } else {
                            return;
                        }

                        if stop_on_first {
                            finished.store(true, ::std::sync::atomic::Ordering::Relaxed);

                            break;
                        }
                    }
                }

                if found_in_thread {
                    found.store(true, ::std::sync::atomic::Ordering::SeqCst);
                }

                running_thread_count.fetch_sub(1, ::std::sync::atomic::Ordering::SeqCst);
            });
        }
    });

    let millis = ::std::time::Duration::from_millis(33);

    while running_thread_count.load(::std::sync::atomic::Ordering::SeqCst) != 0 {
        ::std::thread::sleep(millis);
    }

    let result = offset_array.lock().map_err(|err| err.to_string())?.to_vec();

    Ok(result)
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn rust_alloc(size: usize) -> Result<*mut u8, String> {
    let layout: ::std::alloc::Layout =
        ::std::alloc::Layout::from_size_align(size, ::std::mem::size_of::<u8>())
            .map_err(|err| err.to_string())?;

    let allocated_address: *mut u8 = ::std::alloc::alloc(layout);

    if allocated_address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    Ok(allocated_address)
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn rust_free(address: *mut u8, size: usize) -> Result<(), String> {
    if address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let layout: ::std::alloc::Layout =
        ::std::alloc::Layout::from_size_align(size, ::std::mem::size_of::<u8>())
            .map_err(|err| err.to_string())?;

    ::std::alloc::dealloc(address, layout);

    Ok(())
}

#[unsafe_fn_body::unsafe_fn_body]
pub unsafe fn virtual_alloc(
    address: *mut ::core::ffi::c_void,
    size: usize,
    mem_allocation: u32,
    page_protect: u32,
) -> Result<*mut ::core::ffi::c_void, String> {
    let allocated_address = crate::ffi::VirtualAlloc(address, size, mem_allocation, page_protect);

    if allocated_address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    Ok(allocated_address)
}

#[unsafe_fn_body::unsafe_fn_body]
pub unsafe fn virtual_free(
    address: *mut ::core::ffi::c_void,
    mut size: usize,
    mem_free: u32,
) -> Result<(), String> {
    if address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if mem_free == crate::consts::mem_free::RELEASE {
        size = 0
    }

    if 0 == crate::ffi::VirtualFree(address, size, mem_free) {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    Ok(())
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn virtual_alloc_ex(
    process_handle: *mut ::core::ffi::c_void,
    address: *mut ::core::ffi::c_void,
    size: usize,
    mem_allocation: u32,
    page_protect: u32,
) -> Result<*mut ::core::ffi::c_void, String> {
    if process_handle.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let allocated_address =
        crate::ffi::VirtualAllocEx(process_handle, address, size, mem_allocation, page_protect);

    if allocated_address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    Ok(allocated_address)
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn virtual_free_ex(
    process_handle: *mut ::core::ffi::c_void,
    address: *mut ::core::ffi::c_void,
    mut size: usize,
    mem_free: u32,
) -> Result<(), String> {
    if process_handle.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if mem_free == crate::consts::mem_free::RELEASE {
        size = 0
    }

    if 0 == crate::ffi::VirtualFreeEx(process_handle, address, size, mem_free) {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    Ok(())
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn virtual_query(
    process_handle: *mut ::core::ffi::c_void,
    address: *mut ::core::ffi::c_void,
) -> Result<crate::MemoryInformation, String> {
    if process_handle.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let mut memory_basic_info: crate::ffi::MemoryBasicInformation =
        ::core::mem::zeroed::<crate::ffi::MemoryBasicInformation>();

    if 0 == crate::ffi::VirtualQueryEx(
        process_handle,
        address,
        &mut memory_basic_info,
        ::core::mem::size_of::<crate::ffi::MemoryBasicInformation>(),
    ) {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let memory_info: crate::MemoryInformation = crate::MemoryInformation {
        base_address: memory_basic_info.base_address,
        allocation_base_address: memory_basic_info.allocation_base,
        allocation_protect: memory_basic_info.allocation_protect,
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        partition_id: memory_basic_info.partition_id,
        region_size: memory_basic_info.region_size,
        state: memory_basic_info.state,
        page_protect: memory_basic_info.protect,
        type_: memory_basic_info.type_,
    };

    Ok(memory_info)
}

#[unsafe_fn_body::unsafe_fn_body]
pub fn virtual_protect(
    process_handle: *mut ::core::ffi::c_void,
    address: *const ::core::ffi::c_void,
    new_page_protect: u32,
) -> Result<u32, String> {
    if process_handle.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    if address.is_null() {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    let mut old_page_protect: u32 = 0;

    if 0 == crate::ffi::VirtualProtectEx(
        process_handle,
        address,
        ::core::mem::size_of::<*mut ::core::ffi::c_void>(),
        new_page_protect,
        &mut old_page_protect,
    ) {
        return Err(format!("[{}:{}]", file!(), line!()));
    }

    Ok(old_page_protect)
}
