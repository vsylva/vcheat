#![doc = "[![Crates.io](https://img.shields.io/crates/v/vcheat)](https://crates.io/crates/vcheat)

Rust Language Game Hacking Library
```c
// https://github.com/vSylva/vcheat/tree/main/examples
cargo run --example
```"]

pub mod consts;
mod ffi;
pub mod memory;
pub mod module;
pub mod process;
pub mod system;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemInfomaion {
    pub processor_architecture: u16,
    pub reserved: u16,
    pub page_size: u32,
    pub minimum_application_address: *mut ::core::ffi::c_void,
    pub maximum_application_address: *mut ::core::ffi::c_void,
    pub active_processor_mask: usize,
    pub number_of_processors: u32,
    pub processor_type: u32,
    pub allocation_granularity: u32,
    pub processor_level: u16,
    pub processor_revision: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct ProcessInformation {
    pub id: u32,
    pub thread_count: u32,
    pub parent_process_id: u32,
    pub base_priority_class: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct SystemProcessInformation {
    pub thread_count: u32,
    pub name: String,
    pub base_priority_class: i32,
    pub id: isize,
    pub handle_count: u32,
    pub session_id: u32,
    pub peak_virtual_size: usize,
    pub virtual_size: usize,
    pub peak_working_set_size: usize,
    pub working_set_size: usize,
    pub quota_paged_pool_usage: usize,
    pub quota_non_paged_pool_usage: usize,
    pub pagefile_usage: usize,
    pub peak_pagefile_usage: usize,
    pub private_page_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct ModuleInformation {
    pub process_id: u32,
    pub base_address: *mut u8,
    pub size: u32,
    pub handle: *mut ::core::ffi::c_void,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryInformation {
    pub base_address: *mut ::core::ffi::c_void,
    pub allocation_base_address: *mut ::core::ffi::c_void,
    pub allocation_protect: u32,
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub partition_id: u16,
    pub region_size: usize,
    pub state: u32,
    pub page_protect: u32,
    pub type_: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

pub struct DmiInformation {
    pub bios_version: String,
    pub bios_release_date: String,
    pub bios_vendor: String,
    pub bios_embedded_controller_firmware_version: String,

    pub system_manufacturer: String,
    pub system_product: String,
    pub system_version: String,
    pub system_serial_number: String,
    pub system_uuid: ([u8; 16], String),
    pub system_guid: ([u8; 16], String),
    pub system_sku_number: String,
    pub system_family: String,
}
