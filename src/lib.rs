mod exports;
mod memory;
mod module;
mod process;

pub use exports::*;

#[link(name = "kernel32")]
extern "system" {

    fn OpenProcess(
        dwDesiredAccess: u32,
        bInheritHandle: i32,
        dwProcessId: u32,
    ) -> *mut core::ffi::c_void;

    fn CloseHandle(hObject: *mut core::ffi::c_void) -> i32;

    fn GetSystemInfo(lpSystemInfo: *mut SystemInfo);

    fn CreateToolhelp32Snapshot(dwFlags: u32, th32ProcessID: u32) -> *mut core::ffi::c_void;

    fn Process32FirstW(hSnapshot: *mut core::ffi::c_void, lppe: *mut ProcessEntry32W) -> i32;

    fn Process32NextW(hSnapshot: *mut core::ffi::c_void, lppe: *mut ProcessEntry32W) -> i32;

    fn Module32FirstW(hSnapshot: *mut core::ffi::c_void, lpme: *mut ModuleEntry32W) -> i32;

    fn Module32NextW(hSnapshot: *mut core::ffi::c_void, lpme: *mut ModuleEntry32W) -> i32;

    fn VirtualProtectEx(
        hProcess: *mut core::ffi::c_void,
        lpAddress: *const core::ffi::c_void,
        dwSize: usize,
        flNewProtect: u32,
        lpflOldProtect: *mut u32,
    ) -> i32;

    fn VirtualQueryEx(
        hProcess: *mut core::ffi::c_void,
        lpAddress: *const core::ffi::c_void,
        lpBuffer: *mut MemoryBasicInformation,
        dwLength: usize,
    ) -> usize;

    fn ReadProcessMemory(
        hProcess: *mut core::ffi::c_void,
        lpBaseAddress: *const core::ffi::c_void,
        lpBuffer: *mut core::ffi::c_void,
        nSize: usize,
        lpNumberOfBytesRead: *mut usize,
    ) -> i32;

    fn WriteProcessMemory(
        hProcess: *mut core::ffi::c_void,
        lpBaseAddress: *const core::ffi::c_void,
        lpBuffer: *const core::ffi::c_void,
        nSize: usize,
        lpNumberOfBytesWritten: *mut usize,
    ) -> i32;

}

// #[link(name = "ntdll")]
// extern "system" {
//     fn NtQuerySystemInformation(
//         SystemInformationClass: i32,
//         SystemInformation: *mut core::ffi::c_void,
//         SystemInformationLength: u32,
//         ReturnLength: *mut u32,
//     ) -> i32;
// }

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]

struct ProcessEntry32W {
    dw_size: u32,
    cnt_usage: u32,
    th32_process_id: u32,
    th32_default_heap_id: usize,
    th32_module_id: u32,
    cnt_threads: u32,
    th32_parent_process_id: u32,
    pc_pri_class_base: i32,
    dw_flags: u32,
    sz_exe_file: [u16; 260],
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]

struct ModuleEntry32W {
    dw_size: u32,
    th32_module_id: u32,
    th32_process_id: u32,
    glblcnt_usage: u32,
    proccnt_usage: u32,
    mod_base_addr: *mut u8,
    mod_base_size: u32,
    h_module: *mut core::ffi::c_void,
    sz_module: [u16; 256],
    sz_exe_path: [u16; 260],
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct SystemProcessInformation {
    next_entry_offset: u32,
    number_of_threads: u32,
    reserved1: [u8; 48],
    image_name: UnicodeString,
    base_priority: i32,
    unique_process_id: isize,
    reserved2: *mut core::ffi::c_void,
    handle_count: u32,
    session_id: u32,
    reserved3: *mut core::ffi::c_void,
    peak_virtual_size: usize,
    virtual_size: usize,
    reserved4: u32,
    peak_working_set_size: usize,
    working_set_size: usize,
    reserved5: *mut core::ffi::c_void,
    quota_paged_pool_usage: usize,
    reserved6: *mut core::ffi::c_void,
    quota_non_paged_pool_usage: usize,
    pagefile_usage: usize,
    peak_pagefile_usage: usize,
    private_page_count: usize,
    reserved7: [i64; 6],
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct UnicodeString {
    length: u16,
    maximum_length: u16,
    buffer: *mut u16,
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct MemoryBasicInformation {
    base_address: *mut core::ffi::c_void,
    allocation_base: *mut core::ffi::c_void,
    allocation_protect: u32,
    #[cfg(target_arch = "x86_64")]
    partition_id: u16,
    region_size: usize,
    state: u32,
    protect: u32,
    type_: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SystemInfo {
    anonymous: SystemInfoDummyUnion,
    dw_page_size: u32,
    lp_minimum_application_address: *mut core::ffi::c_void,
    lp_maximum_application_address: *mut core::ffi::c_void,
    dw_active_processor_mask: usize,
    dw_number_of_processors: u32,
    dw_processor_type: u32,
    dw_allocation_granularity: u32,
    w_processor_level: u16,
    w_processor_revision: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]

union SystemInfoDummyUnion {
    dw_oem_id: u32,
    anonymous: SystemInfoDummyStruct,
}

#[repr(C)]
#[derive(Clone, Copy)]

struct SystemInfoDummyStruct {
    w_processor_architecture: u16,
    w_reserved: u16,
}
