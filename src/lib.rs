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

mod ffi;
mod memory;
mod module;
mod process;
mod system;

pub(crate) use ffi::*;
pub use memory::*;
pub use module::*;
pub use process::*;
pub use system::*;

type Result<T> = std::result::Result<T, String>;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessInfo {
    pub process_id: u32,
    pub process_thread_count: u32,
    pub process_parent_process_id: u32,
    pub process_base_priority_class: i32,
    pub process_name: String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemProcessInfo {
    pub process_thread_count: u32,
    pub process_name: String,
    pub process_base_priority_class: i32,
    pub process_id: u32,
    pub process_handle_count: u32,
    pub process_session_id: u32,
    pub process_peak_virtual_size: usize,
    pub process_virtual_size: usize,
    pub process_peak_working_set_size: usize,
    pub process_working_set_size: usize,
    pub process_quota_paged_pool_usage: usize,
    pub process_quota_non_paged_pool_usage: usize,
    pub process_pagefile_usage: usize,
    pub process_peak_pagefile_usage: usize,
    pub process_private_page_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleInfo {
    pub process_id: u32,
    pub module_address: *mut u8,
    pub module_size: u32,
    pub module_handle: *mut core::ffi::c_void,
    pub module_name: String,
    pub module_path: String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DmiInfo {
    pub bios_version: Option<String>,
    pub bios_release_date: Option<String>,
    pub bios_vendor: Option<String>,
    pub bios_embedded_controller_firmware_version: Option<String>,

    pub system_manufacturer: Option<String>,
    pub system_product: Option<String>,
    pub system_version: Option<String>,
    pub system_serial_number: Option<String>,
    pub system_uuid: Option<(Vec<u8>, String)>,
    pub system_guid: Option<(Vec<u8>, String)>,
    pub system_sku_number: Option<String>,
    pub system_family: Option<String>,
}
