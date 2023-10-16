//! # 中文
//!
//! [![Crates.io](https://img.shields.io/crates/v/vcheat)](https://crates.io/crates/vcheat)
//!
//! "vcheat" 是用 Rust 语言编写的适用于 Windows 平台的进程作弊库
//!
//! ```example
//! // https://github.com/vSylva/vcheat/tree/main/examples
//! cargo run --example
//! ```
//!
//! # English
//!
//! [![Crates.io](https://img.shields.io/crates/v/vcheat)](https://crates.io/crates/vcheat)
//!
//! "vcheat" is a process cheating library designed for the Windows platform and written in Rust programming language
//!
//! ```example
//! // https://github.com/vSylva/vcheat/tree/main/examples
//! cargo run --example
//! ```

mod core;
mod ffi;

pub use crate::core::*;

pub mod memory {

    pub fn read_process_memory(
        process_handle: *mut ::core::ffi::c_void,
        address: *const ::core::ffi::c_void,
        size: usize,
    ) -> Result<Vec<u8>, String> {
        unsafe { crate::core::memory::read_process_memory(process_handle, address, size) }
    }

    pub fn read_process_memory_unchecked(
        process_handle: *mut ::core::ffi::c_void,
        address: *const ::core::ffi::c_void,
        size: usize,
    ) -> Vec<u8> {
        unsafe { crate::core::memory::read_process_memory_unchecked(process_handle, address, size) }
    }

    pub fn write_process_memory<T>(
        process_handle: *mut ::core::ffi::c_void,
        address: *mut ::core::ffi::c_void,
        data: &[T],
    ) -> Result<usize, String> {
        unsafe { crate::core::memory::write_process_memory(process_handle, address, data) }
    }

    pub fn write_process_memory_unchecked<T>(
        process_handle: *mut ::core::ffi::c_void,
        address: *mut ::core::ffi::c_void,
        data: &[T],
    ) {
        unsafe {
            crate::core::memory::write_process_memory_unchecked(process_handle, address, data)
        }
    }

    pub fn aob_scan_single_threaded(
        pattern: &str,
        data: &[u8],
        return_on_first: bool,
    ) -> Result<Vec<usize>, String> {
        crate::core::memory::aob_scan_single_threaded(pattern, data, return_on_first)
    }

    pub fn aob_scan_multi_threaded(
        pattern: &str,
        data: &[u8],
        return_on_first: bool,
        thread_count: u32,
    ) -> Result<Vec<usize>, String> {
        crate::core::memory::aob_scan_multi_threaded(pattern, data, return_on_first, thread_count)
    }

    pub fn standard_alloc(size: usize) -> Result<*mut u8, String> {
        unsafe { crate::core::memory::standard_alloc(size) }
    }

    pub fn standard_free(address: *mut u8, size: usize) -> Result<(), String> {
        unsafe { crate::core::memory::standard_free(address, size) }
    }

    pub fn virtual_alloc(
        address: *mut ::core::ffi::c_void,
        size: usize,
        mem_allocation: u32,
        page_protect: u32,
    ) -> Result<*mut ::core::ffi::c_void, String> {
        unsafe {
            crate::core::memory::virtual_alloc(
                address,
                size,
                mem_allocation.into(),
                page_protect.into(),
            )
        }
    }

    pub fn virtual_free(
        address: *mut ::core::ffi::c_void,
        size: usize,
        mem_free: u32,
    ) -> Result<(), String> {
        unsafe { crate::core::memory::virtual_free(address, size, mem_free.into()) }
    }

    pub fn virtual_alloc_ex(
        process_handle: *mut ::core::ffi::c_void,
        address: *mut ::core::ffi::c_void,
        size: usize,
        mem_allocation: u32,
        page_protect: u32,
    ) -> Result<*mut ::core::ffi::c_void, String> {
        unsafe {
            crate::core::memory::virtual_alloc_ex(
                process_handle,
                address,
                size,
                mem_allocation.into(),
                page_protect.into(),
            )
        }
    }

    pub fn virtual_free_ex(
        process_handle: *mut ::core::ffi::c_void,
        address: *mut ::core::ffi::c_void,
        size: usize,
        mem_free: u32,
    ) -> Result<(), String> {
        unsafe {
            let mem_free = mem_free.into();

            if mem_free == 0x00008000 {
                return crate::core::memory::virtual_free_ex(process_handle, address, 0, mem_free);
            }

            crate::core::memory::virtual_free(address, size, mem_free)
        }
    }

    pub fn virtual_query(
        process_handle: *mut ::core::ffi::c_void,
        address: *mut ::core::ffi::c_void,
    ) -> Result<crate::core::MemoryInformation, String> {
        unsafe { crate::core::memory::virtual_query(process_handle, address) }
    }

    pub fn virtual_protect(
        process_handle: *mut ::core::ffi::c_void,
        address: *const ::core::ffi::c_void,
        new_page_protect: u32,
    ) -> Result<u32, String> {
        unsafe {
            crate::core::memory::virtual_protect(process_handle, address, new_page_protect.into())
        }
    }
}

pub mod module {

    pub fn get_modules_info(
        process_id: u32,
    ) -> Result<Vec<crate::core::ModuleInformation>, String> {
        unsafe { crate::core::module::get_modules_info(process_id) }
    }

    pub fn get_module_info(
        process_id: u32,
        module_name: &str,
    ) -> Result<crate::core::ModuleInformation, String> {
        unsafe { crate::core::module::get_module_info(process_id, module_name) }
    }

    pub fn load_library(dll_path: &str) -> Result<*mut ::core::ffi::c_void, String> {
        unsafe { crate::core::module::load_library(dll_path) }
    }

    pub fn load_system_library(dll_name: &str) -> Result<*mut ::core::ffi::c_void, String> {
        unsafe { crate::core::module::load_system_library(dll_name) }
    }

    pub fn free_library(module_handle: *mut ::core::ffi::c_void) -> Result<(), String> {
        unsafe { crate::core::module::free_library(module_handle) }
    }

    pub fn get_proc_address(
        module_handle: *mut ::core::ffi::c_void,
        proc_name: &str,
    ) -> Result<*mut ::core::ffi::c_void, String> {
        unsafe { crate::core::module::get_proc_address(module_handle, proc_name.as_ref()) }
    }

    pub fn inject_dll(
        process_handle: *mut ::core::ffi::c_void,
        dll_path: &str,
    ) -> Result<(), String> {
        unsafe { crate::core::module::inject_dll(process_handle, dll_path) }
    }

    pub fn eject_dll(
        process_handle: *mut ::core::ffi::c_void,
        module_handle: *mut ::core::ffi::c_void,
    ) -> Result<(), String> {
        unsafe { crate::core::module::eject_dll(process_handle, module_handle) }
    }
}

pub mod process {

    pub fn open_process(process_id: u32) -> Result<*mut ::core::ffi::c_void, String> {
        unsafe { crate::core::process::open_process(process_id) }
    }

    pub fn close_handle(handle: *mut ::core::ffi::c_void) -> Result<(), String> {
        unsafe { crate::core::process::close_handle(handle) }
    }

    pub fn close_handle_unchecked(handle: *mut ::core::ffi::c_void) -> i32 {
        unsafe { crate::core::process::close_handle_unchecked(handle) }
    }

    pub fn is_wow64_process(process_handle: *mut ::core::ffi::c_void) -> Result<bool, String> {
        unsafe { crate::core::process::is_wow64_process(process_handle) }
    }

    pub fn get_processes_info() -> Result<Vec<crate::core::ProcessInformation>, String> {
        unsafe { crate::core::process::get_processes_info() }
    }

    pub fn get_process_info(process_name: &str) -> Result<crate::core::ProcessInformation, String> {
        unsafe { crate::core::process::get_process_info(process_name) }
    }

    /// The function is not stable, but it provides more information compared to non-NT functions
    pub fn nt_get_processes_info() -> Result<Vec<crate::core::SystemProcessInformation>, String> {
        unsafe { crate::core::process::nt_get_processes_info() }
    }

    pub fn alloc_console() -> Result<(), String> {
        unsafe { crate::core::process::alloc_console() }
    }

    pub fn alloc_console_unchecked() -> i32 {
        unsafe { crate::core::process::alloc_console_unchecked() }
    }

    pub fn free_console() -> Result<(), String> {
        unsafe { crate::core::process::free_console() }
    }

    pub fn free_console_unchecked() -> i32 {
        unsafe { crate::core::process::free_console_unchecked() }
    }

    pub fn set_console_mode(standard_handle: u32, console_mode: u32) -> Result<(), String> {
        unsafe { crate::core::process::set_console_mode(standard_handle, console_mode) }
    }

    pub fn set_console_colors() -> Result<(), String> {
        unsafe { crate::core::process::set_console_colors() }
    }
}

pub mod system {

    pub fn get_system_info() -> crate::core::SystemInfo {
        unsafe { crate::core::system::get_system_info() }
    }

    pub fn get_dmi_info() -> Result<crate::core::DmiInformation, String> {
        unsafe { crate::core::system::get_dmi_info() }
    }
}
