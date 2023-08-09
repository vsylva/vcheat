mod memory;
mod module;
mod process;

use crate::*;

pub use memory::*;
pub use module::*;
pub use process::*;

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VProcessInfo {
    pub process_id: u32,
    pub process_thread_count: u32,
    pub process_parent_process_id: u32,
    pub process_base_priority_class: i32,
    pub process_name: String,
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VModuleInfo {
    pub process_id: u32,
    pub module_address: *mut u8,
    pub module_size: u32,
    pub module_handle: *mut core::ffi::c_void,
    pub module_name: String,
    pub module_path: String,
    pub module_data: Option<Vec<u8>>,
}

// #[repr(C)]
// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
// pub struct VNTProcessInfo {
//     pub process_thread_count: u32,
//     pub process_name: String,
//     pub process_base_priority_class: i32,
//     pub process_id: u32,
//     pub process_handle_count: u32,
//     pub process_session_id: u32,
//     pub process_peak_virtual_size: usize,
//     pub process_virtual_size: usize,
//     pub process_peak_working_set_size: usize,
//     pub process_working_set_size: usize,
//     pub process_quota_paged_pool_usage: usize,
//     pub process_quota_non_paged_pool_usage: usize,
//     pub process_pagefile_usage: usize,
//     pub process_peak_pagefile_usage: usize,
//     pub process_private_page_count: usize,
// }

pub unsafe fn read_memory(
    process_handle: *mut core::ffi::c_void,
    address: *const core::ffi::c_void,
    size: usize,
) -> Result<Vec<u8>> {
    let result = VirtualQueryEx(
        process_handle,
        address,
        &mut MemoryBasicInformation {
            ..core::mem::zeroed()
        },
        core::mem::size_of::<MemoryBasicInformation>(),
    );

    if result == 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("VirtualQueryEx failed with return value: {result:X}"),
        )));
    }

    let mut old_protect = 0u32;

    let mut new_protect = 4u32;

    let result = VirtualProtectEx(
        process_handle,
        address,
        core::mem::size_of::<*mut core::ffi::c_void>(),
        new_protect,
        &mut old_protect,
    );

    if result == 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("VirtualProtectEx failed with return value: {result:X}"),
        )));
    }

    let mut buffer: Vec<u8> = Vec::new();

    buffer.resize(size, 0u8);

    let result = ReadProcessMemory(
        process_handle,
        address,
        buffer.as_mut_ptr().cast(),
        size,
        core::ptr::null_mut(),
    );

    if result == 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ReadProcessMemory failed with return value: {result:X}"),
        )));
    }

    let result = VirtualProtectEx(
        process_handle,
        address,
        core::mem::size_of::<*mut core::ffi::c_void>(),
        old_protect,
        &mut new_protect,
    );

    if result == 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("VirtualProtectEx failed with return value: {result:X}"),
        )));
    }

    Ok(buffer)
}

pub unsafe fn write_memory(
    process_handle: *mut core::ffi::c_void,
    address: *mut core::ffi::c_void,
    buffer: &mut Vec<u8>,
) -> Result<()> {
    let result = WriteProcessMemory(
        process_handle,
        address,
        buffer.as_ptr().cast(),
        buffer.len(),
        core::ptr::null_mut(),
    );
    if result == 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("WriteProcessMemory failed with return value: {result:X}"),
        )));
    }

    Ok(())
}

pub fn get_logical_cpu_count() -> u32 {
    unsafe {
        let system_info = &mut core::mem::zeroed() as *mut SystemInfo;

        GetSystemInfo(system_info);

        (*system_info).dw_number_of_processors
    }
}
