#[link(name = "kernel32")]

extern "system" {

    pub(crate) fn WaitForSingleObject(hHandle: *mut core::ffi::c_void, dwMilliseconds: u32) -> u32;

    pub(crate) fn CreateRemoteThread(
        hProcess: *mut core::ffi::c_void,
        lpThreadAttributes: *const SecurityAttributes,
        dwStackSize: usize,
        lpStartAddress: LpthreadStartRoutine,
        lpParameter: *const core::ffi::c_void,
        dwCreationFlags: u32,
        lpThreadId: *mut u32,
    ) -> *mut core::ffi::c_void;

    pub(crate) fn IsWow64Process(hProcess: *mut core::ffi::c_void, Wow64Process: *mut i32) -> i32;

    pub(crate) fn OpenProcess(
        dwDesiredAccess: u32,
        bInheritHandle: i32,
        dwProcessId: u32,
    ) -> *mut core::ffi::c_void;

    pub(crate) fn CloseHandle(hObject: *mut core::ffi::c_void) -> i32;

    pub(crate) fn GetSystemInfo(lpSystemInfo: *mut SystemInfo);

    pub(crate) fn CreateToolhelp32Snapshot(
        dwFlags: u32,
        th32ProcessID: u32,
    ) -> *mut core::ffi::c_void;

    pub(crate) fn Process32FirstW(
        hSnapshot: *mut core::ffi::c_void,
        lppe: *mut ProcessEntry32W,
    ) -> i32;

    pub(crate) fn Process32NextW(
        hSnapshot: *mut core::ffi::c_void,
        lppe: *mut ProcessEntry32W,
    ) -> i32;

    pub(crate) fn Module32FirstW(
        hSnapshot: *mut core::ffi::c_void,
        lpme: *mut ModuleEntry32W,
    ) -> i32;

    pub(crate) fn Module32NextW(
        hSnapshot: *mut core::ffi::c_void,
        lpme: *mut ModuleEntry32W,
    ) -> i32;

    pub(crate) fn VirtualProtectEx(
        hProcess: *mut core::ffi::c_void,
        lpAddress: *const core::ffi::c_void,
        dwSize: usize,
        flNewProtect: u32,
        lpflOldProtect: *mut u32,
    ) -> i32;

    pub(crate) fn VirtualQueryEx(
        hProcess: *mut core::ffi::c_void,
        lpAddress: *const core::ffi::c_void,
        lpBuffer: *mut MemoryBasicInformation,
        dwLength: usize,
    ) -> usize;

    pub(crate) fn VirtualAlloc(
        lpAddress: *mut core::ffi::c_void,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut core::ffi::c_void;

    pub(crate) fn VirtualAllocEx(
        hProcess: *mut core::ffi::c_void,
        lpAddress: *mut core::ffi::c_void,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut core::ffi::c_void;

    pub(crate) fn VirtualFreeEx(
        hProcess: *mut core::ffi::c_void,
        lpAddress: *mut core::ffi::c_void,
        dwSize: usize,
        dwFreeType: u32,
    ) -> i32;

    pub(crate) fn VirtualFree(
        lpAddress: *mut core::ffi::c_void,
        dwSize: usize,
        dwFreeType: u32,
    ) -> i32;

    pub(crate) fn ReadProcessMemory(
        hProcess: *mut core::ffi::c_void,
        lpBaseAddress: *const core::ffi::c_void,
        lpBuffer: *mut core::ffi::c_void,
        nSize: usize,
        lpNumberOfBytesRead: *mut usize,
    ) -> i32;

    pub(crate) fn WriteProcessMemory(
        hProcess: *mut core::ffi::c_void,
        lpBaseAddress: *const core::ffi::c_void,
        lpBuffer: *const core::ffi::c_void,
        nSize: usize,
        lpNumberOfBytesWritten: *mut usize,
    ) -> i32;

    pub(crate) fn GetSystemFirmwareTable(
        FirmwareTableProviderSignature: u32,
        FirmwareTableID: u32,
        pFirmwareTableBuffer: *mut u8,
        BufferSize: u32,
    ) -> u32;

    // pub(crate) fn GetSystemDirectoryA(lpBuffer: *mut i8, uSize: u32) -> u32;

    pub(crate) fn GetSystemDirectoryW(lpBuffer: *mut u16, uSize: u32) -> u32;

    // pub(crate) fn LoadLibraryA(lpLibFileName: *const i8) -> *mut core::ffi::c_void;

    pub(crate) fn LoadLibraryW(lpLibFileName: *const u16) -> *mut core::ffi::c_void;

    pub(crate) fn FreeLibrary(hLibModule: *mut core::ffi::c_void) -> i32;

    pub(crate) fn AllocConsole() -> i32;

    pub(crate) fn FreeConsole() -> i32;

    pub(crate) fn SetConsoleMode(hConsoleHandle: *mut core::ffi::c_void, dwMode: u32) -> i32;

    pub(crate) fn GetStdHandle(nStdHandle: u32) -> *mut core::ffi::c_void;

    pub(crate) fn GetConsoleMode(hConsoleHandle: *mut core::ffi::c_void, lpMode: *mut u32) -> i32;

    pub(crate) fn GetProcAddress(
        hModule: *mut core::ffi::c_void,
        lpProcName: *const i8,
    ) -> *mut core::ffi::c_void;

    // pub(crate) fn GetLastError() -> u32;

    // pub(crate) fn FormatMessageW(
    //     dwFlags: u32,
    //     lpSource: *const core::ffi::c_void,
    //     dwMessageId: u32,
    //     dwLanguageId: u32,
    //     lpBuffer: *mut u16,
    //     nSize: u32,
    //     Arguments: *const *const i8,
    // ) -> u32;

    // pub(crate) fn SetLastError(dwErrCode: u32);

}

#[link(name = "ntdll")]

extern "system" {

    pub(crate) fn NtQuerySystemInformation(
        SystemInformationClass: i32,
        SystemInformation: *mut core::ffi::c_void,
        SystemInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> i32;

}

pub(crate) type LpthreadStartRoutine =
    unsafe extern "system" fn(lpThreadParameter: *mut core::ffi::c_void) -> u32;

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct SecurityAttributes {
    pub(crate) n_length: u32,
    pub(crate) lp_security_descriptor: *const core::ffi::c_void,
    pub(crate) b_inherit_handle: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct ProcessEntry32W {
    pub(crate) dw_size: u32,
    pub(crate) cnt_usage: u32,
    pub(crate) th32_process_id: u32,
    pub(crate) th32_default_heap_id: usize,
    pub(crate) th32_module_id: u32,
    pub(crate) cnt_threads: u32,
    pub(crate) th32_parent_process_id: u32,
    pub(crate) pc_pri_class_base: i32,
    pub(crate) dw_flags: u32,
    pub(crate) sz_exe_file: [u16; 260],
}

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct ModuleEntry32W {
    pub(crate) dw_size: u32,
    pub(crate) th32_module_id: u32,
    pub(crate) th32_process_id: u32,
    pub(crate) glblcnt_usage: u32,
    pub(crate) proccnt_usage: u32,
    pub(crate) mod_base_addr: *mut u8,
    pub(crate) mod_base_size: u32,
    pub(crate) h_module: *mut core::ffi::c_void,
    pub(crate) sz_module: [u16; 256],
    pub(crate) sz_exe_path: [u16; 260],
}

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct SystemProcessInformation {
    pub(crate) next_entry_offset: u32,
    pub(crate) number_of_threads: u32,
    pub(crate) reserved1: [u8; 48],
    pub(crate) image_name: UnicodeString,
    pub(crate) base_priority: i32,
    pub(crate) unique_process_id: isize,
    pub(crate) reserved2: *mut core::ffi::c_void,
    pub(crate) handle_count: u32,
    pub(crate) session_id: u32,
    pub(crate) reserved3: *mut core::ffi::c_void,
    pub(crate) peak_virtual_size: usize,
    pub(crate) virtual_size: usize,
    pub(crate) reserved4: u32,
    pub(crate) peak_working_set_size: usize,
    pub(crate) working_set_size: usize,
    pub(crate) reserved5: *mut core::ffi::c_void,
    pub(crate) quota_paged_pool_usage: usize,
    pub(crate) reserved6: *mut core::ffi::c_void,
    pub(crate) quota_non_paged_pool_usage: usize,
    pub(crate) pagefile_usage: usize,
    pub(crate) peak_pagefile_usage: usize,
    pub(crate) private_page_count: usize,
    pub(crate) reserved7: [i64; 6],
}

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct UnicodeString {
    pub(crate) length: u16,
    pub(crate) maximum_length: u16,
    pub(crate) buffer: *mut u16,
}

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct MemoryBasicInformation {
    pub(crate) base_address: *mut core::ffi::c_void,
    pub(crate) allocation_base: *mut core::ffi::c_void,
    pub(crate) allocation_protect: u32,
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub(crate) partition_id: u16,
    pub(crate) region_size: usize,
    pub(crate) state: u32,
    pub(crate) protect: u32,
    pub(crate) type_: u32,
}

#[repr(C)]

pub(crate) struct SystemInfo {
    pub(crate) anonymous: SystemInfoDummyUnion,
    pub(crate) dw_page_size: u32,
    pub(crate) lp_minimum_application_address: *mut core::ffi::c_void,
    pub(crate) lp_maximum_application_address: *mut core::ffi::c_void,
    pub(crate) dw_active_processor_mask: usize,
    pub(crate) dw_number_of_processors: u32,
    pub(crate) dw_processor_type: u32,
    pub(crate) dw_allocation_granularity: u32,
    pub(crate) w_processor_level: u16,
    pub(crate) w_processor_revision: u16,
}

#[repr(C)]

pub(crate) union SystemInfoDummyUnion {
    pub(crate) dw_oem_id: u32,
    pub(crate) anonymous: std::mem::ManuallyDrop<SystemInfoDummyStruct>,
}

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct SystemInfoDummyStruct {
    pub(crate) w_processor_architecture: u16,
    pub(crate) w_reserved: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct RawSMBIOSData {
    pub(crate) used20_calling_method: u8,
    pub(crate) smbiosmajor_version: u8,
    pub(crate) smbiosminor_version: u8,
    pub(crate) dmi_revision: u8,
    pub(crate) length: u32,
    pub(crate) smbiostable_data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug, Clone)]

pub(crate) struct DmiHeader {
    pub(crate) ctype: u8,
    pub(crate) length: u8,
    pub(crate) handle: u16,
}
