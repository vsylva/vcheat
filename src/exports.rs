pub(crate) mod memory {
    type Result<T> = std::result::Result<T, String>;

    pub fn read_process_memory(
        process_id: u32,
        target_address: *const core::ffi::c_void,
        size: usize,
    ) -> Result<Vec<u8>> {
        unsafe { crate::memory::read_process_memory(process_id, target_address, size) }
    }

    pub fn write_process_memory(
        process_id: u32,
        target_address: *mut core::ffi::c_void,
        data: &[u8],
    ) -> Result<usize> {
        unsafe { crate::memory::write_process_memory(process_id, target_address, data) }
    }

    pub fn aob_scan_single_threaded(
        pattern: &str,
        data: &[u8],
        return_on_first: bool,
    ) -> Result<Vec<usize>> {
        crate::memory::aob_scan_single_threaded(pattern, data, return_on_first)
    }

    pub fn aob_scan_multi_threaded(
        pattern: &str,
        data: &[u8],
        return_on_first: bool,
        thread_count: u32,
    ) -> Result<Vec<usize>> {
        crate::memory::aob_scan_multi_threaded(pattern, data, return_on_first, thread_count)
    }

    pub fn standard_alloc(size: usize) -> Result<*mut u8> {
        unsafe { crate::memory::standard_alloc(size) }
    }

    pub fn standard_free(target_address: *mut u8, size: usize) -> Result<()> {
        unsafe { crate::memory::standard_free(target_address, size) }
    }

    pub fn virtual_alloc<T: Into<u32>, U: Into<u32>>(
        target_address: *mut core::ffi::c_void,
        size: usize,
        mem_allocation: T,
        page_protect: U,
    ) -> Result<*mut core::ffi::c_void> {
        unsafe {
            crate::memory::virtual_alloc(
                target_address,
                size,
                mem_allocation.into(),
                page_protect.into(),
            )
        }
    }

    pub fn virtual_free<T: Into<u32>>(
        target_address: *mut core::ffi::c_void,
        size: usize,
        mem_free: T,
    ) -> Result<()> {
        unsafe {
            let mem_free = mem_free.into();

            if mem_free == 0x00008000 {
                return crate::memory::virtual_free(target_address, 0, mem_free);
            }

            crate::memory::virtual_free(target_address, size, mem_free)
        }
    }

    pub fn virtual_query(
        process_id: u32,
        target_address: *mut core::ffi::c_void,
    ) -> Result<crate::types::MemoryInfo> {
        unsafe { crate::memory::virtual_query(process_id, target_address) }
    }

    pub fn virtual_protect<T: Into<u32>>(
        process_id: u32,
        target_address: *const core::ffi::c_void,
        new_page_protect: T,
    ) -> Result<u32> {
        unsafe {
            crate::memory::virtual_protect(process_id, target_address, new_page_protect.into())
        }
    }
}

pub(crate) mod module {
    type Result<T> = std::result::Result<T, String>;

    pub fn get_all_process_modules_info(process_id: u32) -> Result<Vec<crate::types::ModuleInfo>> {
        unsafe { crate::module::get_all_process_modules_info(process_id) }
    }
    pub fn load_library<S: AsRef<std::path::Path>>(path: S) -> Result<*mut core::ffi::c_void> {
        unsafe { crate::module::load_library(path) }
    }

    pub fn load_system_library<S: AsRef<str>>(file_name: S) -> Result<*mut core::ffi::c_void> {
        unsafe { crate::module::load_system_library(file_name) }
    }

    pub fn get_proc_address<S: AsRef<str>>(
        module_handle: *mut core::ffi::c_void,
        proc_name: S,
    ) -> *mut core::ffi::c_void {
        unsafe { crate::module::get_proc_address(module_handle, proc_name) }
    }
}

pub(crate) mod process {

    type Result<T> = std::result::Result<T, String>;

    pub fn open_process_handle(process_id: u32) -> Result<*mut core::ffi::c_void> {
        unsafe { crate::process::open_process_handle(process_id) }
    }

    pub fn close_handle(handle: *mut core::ffi::c_void) -> Result<()> {
        unsafe { crate::process::close_handle(handle) }
    }

    #[cfg(target_arch = "x86")]
    pub fn is_wow64_process(process_id: u32) -> Result<bool> {
        unsafe { crate::process::is_wow64_process(process_id) }
    }

    pub fn get_all_processes_info() -> Result<Vec<crate::types::ProcessInfo>> {
        unsafe { crate::process::get_all_processes_info() }
    }

    pub fn nt_get_all_processes_info() -> Result<Vec<crate::types::SystemProcessInfo>> {
        unsafe { crate::process::nt_get_all_processes_info() }
    }

    pub fn alloc_console() -> Result<()> {
        unsafe { crate::process::alloc_console() }
    }

    pub fn free_console() -> Result<()> {
        unsafe { crate::process::free_console() }
    }

    pub fn set_console_mode<T: Into<u32>>(
        standard_handle: crate::types::StandardHandle,
        console_mode: T,
    ) -> Result<()> {
        unsafe { crate::process::set_console_mode(standard_handle.into(), console_mode.into()) }
    }
}

pub(crate) mod system {
    type Result<T> = std::result::Result<T, String>;

    pub fn get_logical_cpu_count() -> u32 {
        unsafe { crate::system::get_logical_cpu_count() }
    }

    pub fn get_dmi_info() -> Result<crate::types::DmiInfo> {
        unsafe { crate::system::get_dmi_info() }
    }
}

pub(crate) mod types {

    pub use crate::types::*;
}
