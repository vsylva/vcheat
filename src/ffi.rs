use crate::{BOOL, HANDLE, HMODULE};

#[link(name = "Psapi")]
extern "system" {
    pub(crate) fn GetModuleInformation(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpmodinfo: *mut MODULEINFO,
        cb: u32,
    ) -> BOOL;
}

#[link(name = "Kernel32")]
extern "system" {

    pub(crate) fn GetExitCodeThread(hThread: HANDLE, lpExitCode: *mut u32) -> BOOL;

    pub(crate) fn GetCurrentProcess() -> HANDLE;

    pub(crate) fn GetModuleHandleW(lpModuleName: *const u16) -> HMODULE;

    pub(crate) fn WaitForSingleObject(hHandle: HANDLE, dwMilliseconds: u32) -> u32;

    pub(crate) fn CreateRemoteThread(
        hProcess: HANDLE,
        lpThreadAttributes: *const SecurityAttributes,
        dwStackSize: usize,
        lpStartAddress: LpthreadStartRoutine,
        lpParameter: *const ::core::ffi::c_void,
        dwCreationFlags: u32,
        lpThreadId: *mut u32,
    ) -> HANDLE;

    pub(crate) fn OpenProcess(
        dwDesiredAccess: u32,
        bInheritHandle: BOOL,
        dwProcessId: u32,
    ) -> HANDLE;

    pub(crate) fn CloseHandle(hObject: HANDLE) -> BOOL;

    pub(crate) fn CreateToolhelp32Snapshot(dwFlags: u32, th32ProcessID: u32) -> HANDLE;

    pub(crate) fn Process32FirstW(hSnapshot: HANDLE, lppe: *mut ProcessEntry32W) -> BOOL;

    pub(crate) fn Process32NextW(hSnapshot: HANDLE, lppe: *mut ProcessEntry32W) -> BOOL;

    pub(crate) fn Module32FirstW(hSnapshot: HANDLE, lpme: *mut ModuleEntry32W) -> BOOL;

    pub(crate) fn Module32NextW(hSnapshot: HANDLE, lpme: *mut ModuleEntry32W) -> BOOL;

    pub(crate) fn VirtualProtectEx(
        hProcess: HANDLE,
        lpAddress: *const ::core::ffi::c_void,
        dwSize: usize,
        flNewProtect: u32,
        lpflOldProtect: *mut u32,
    ) -> BOOL;

    pub(crate) fn VirtualQueryEx(
        hProcess: HANDLE,
        lpAddress: *const ::core::ffi::c_void,
        lpBuffer: *mut MemoryBasicInformation,
        dwLength: usize,
    ) -> usize;

    pub(crate) fn VirtualAllocEx(
        hProcess: HANDLE,
        lpAddress: *const ::core::ffi::c_void,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut ::core::ffi::c_void;

    pub(crate) fn VirtualFreeEx(
        hProcess: HANDLE,
        lpAddress: *mut ::core::ffi::c_void,
        dwSize: usize,
        dwFreeType: u32,
    ) -> BOOL;

    pub(crate) fn ReadProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: *const ::core::ffi::c_void,
        lpBuffer: *mut ::core::ffi::c_void,
        nSize: usize,
        lpNumberOfBytesRead: *mut usize,
    ) -> BOOL;

    pub(crate) fn WriteProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: *const ::core::ffi::c_void,
        lpBuffer: *const ::core::ffi::c_void,
        nSize: usize,
        lpNumberOfBytesWritten: *mut usize,
    ) -> BOOL;

    pub(crate) fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: u32) -> i32;

    pub(crate) fn GetStdHandle(nStdHandle: u32) -> HANDLE;

    pub(crate) fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: *mut u32) -> BOOL;

    pub(crate) fn GetProcAddress(
        hModule: HMODULE,
        lpProcName: *const i8,
    ) -> *mut ::core::ffi::c_void;

    pub(crate) fn AllocConsole() -> BOOL;

    pub(crate) fn FreeConsole() -> BOOL;

    // internal
    pub(crate) fn VirtualAlloc(
        lpAddress: *const ::core::ffi::c_void,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut ::core::ffi::c_void;

    pub(crate) fn VirtualFree(
        lpAddress: *mut ::core::ffi::c_void,
        dwSize: usize,
        dwFreeType: u32,
    ) -> BOOL;

    pub(crate) fn VirtualQuery(
        lpAddress: *const ::core::ffi::c_void,
        lpBuffer: *mut MemoryBasicInformation,
        dwLength: usize,
    ) -> usize;

    pub(crate) fn VirtualProtect(
        lpAddress: *const ::core::ffi::c_void,
        dwSize: usize,
        flNewProtect: u32,
        lpflOldProtect: *mut u32,
    ) -> BOOL;

    pub(crate) fn LoadLibraryW(lpLibFileName: *const u16) -> HMODULE;

    pub(crate) fn FreeLibrary(hLibModule: HMODULE) -> BOOL;

    pub(crate) fn FreeLibraryAndExitThread(hLibModule: HMODULE, dwExitCode: u32) -> !;
}

pub(crate) type LpthreadStartRoutine =
    unsafe extern "system" fn(lpThreadParameter: *mut ::core::ffi::c_void) -> u32;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SecurityAttributes {
    pub(crate) n_length: u32,
    pub(crate) lp_security_descriptor: *const ::core::ffi::c_void,
    pub(crate) b_inherit_handle: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ModuleEntry32W {
    pub(crate) dw_size: u32,
    pub(crate) th32_module_id: u32,
    pub(crate) th32_process_id: u32,
    pub(crate) glbl_cnt_usage: u32,
    pub(crate) proc_cnt_usage: u32,
    pub(crate) mod_base_addr: *mut u8,
    pub(crate) mod_base_size: u32,
    pub(crate) h_module: HMODULE,
    pub(crate) sz_module: [u16; 256],
    pub(crate) sz_exe_path: [u16; 260],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct MemoryBasicInformation {
    pub(crate) base_address: *mut ::core::ffi::c_void,
    pub(crate) allocation_base: *mut ::core::ffi::c_void,
    pub(crate) allocation_protect: u32,
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub(crate) partition_id: u16,
    pub(crate) region_size: usize,
    pub(crate) state: u32,
    pub(crate) protect: u32,
    pub(crate) type_: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct MODULEINFO {
    pub(crate) lp_base_of_dll: *mut ::core::ffi::c_void,
    pub(crate) size_of_image: u32,
    pub(crate) entry_point: *mut core::ffi::c_void,
}
