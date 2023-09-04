#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessInfo {
    pub process_id: u32,
    pub process_thread_count: u32,
    pub process_parent_process_id: u32,
    pub process_base_priority_class: i32,
    pub process_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemProcessInfo {
    pub process_thread_count: u32,
    pub process_name: String,
    pub process_base_priority_class: i32,
    pub process_id: isize,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleInfo {
    pub process_id: u32,
    pub module_base_address: *mut u8,
    pub module_size: u32,
    pub module_handle: *mut core::ffi::c_void,
    pub module_name: String,
    pub module_path: String,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryInfo {
    pub memory_base_address: *mut core::ffi::c_void,
    pub memory_allocation_base_address: *mut core::ffi::c_void,
    pub memory_allocation_protect: u32,
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub memory_partition_id: u16,
    pub memory_region_size: usize,
    pub memory_state: u32,
    pub memory_page_protect: u32,
    pub memory_type: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DmiInfo {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PageProtect {
    /// 0x10
    Execute,
    /// 0x20
    ExecuteRead,
    /// 0x40
    ExecuteReadWrite,
    /// 0x80
    ExecuteWritecopy,
    /// 0x01
    Noaccess,
    /// 0x02
    Readonly,
    /// 0x04
    ReadWrite,
    /// 0x08
    Writecopy,
    /// 0x40000000
    TargetsInvalid,
    /// 0x40000000
    TargetsNoUpdate,

    /// 0x100
    Guard,
    /// 0x200
    Nocache,
    /// 0x400
    Writecombine,

    /// 0x10000000
    EnclaveDecommit,
    /// 0x80000000
    EnclaveThreadControl,
    /// 0x20000000
    EnclaveUnvalidated,
}

impl Into<u32> for PageProtect {
    fn into(self) -> u32 {
        match self {
            Self::Execute => 0x10,
            Self::ExecuteRead => 0x20,
            Self::ExecuteReadWrite => 0x40,
            Self::ExecuteWritecopy => 0x80,
            Self::Noaccess => 0x01,
            Self::Readonly => 0x02,
            Self::ReadWrite => 0x04,
            Self::Writecopy => 0x08,
            Self::TargetsInvalid => 0x40000000,
            Self::TargetsNoUpdate => 0x40000000,

            Self::Guard => 0x100,
            Self::Nocache => 0x200,
            Self::Writecombine => 0x400,

            Self::EnclaveDecommit => 0x10000000,
            Self::EnclaveThreadControl => 0x80000000,
            Self::EnclaveUnvalidated => 0x20000000,
        }
    }
}

impl core::ops::BitOr<PageProtect> for PageProtect {
    type Output = u32;

    fn bitor(self, rhs: PageProtect) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<u32> for PageProtect {
    type Output = u32;

    fn bitor(self, rhs: u32) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<PageProtect> for u32 {
    type Output = u32;

    fn bitor(self, rhs: PageProtect) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl PartialEq<u32> for PageProtect {
    fn eq(&self, other: &u32) -> bool {
        Into::<u32>::into(self.clone()) == *other
    }
}

impl PartialEq<PageProtect> for u32 {
    fn eq(&self, other: &PageProtect) -> bool {
        *self == Into::<u32>::into(other.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemAllocation {
    /// 0x00001000
    Commit,
    /// 0x00002000
    Reserve,
    /// 0x00080000
    Reset,
    /// 0x1000000
    ResetUndo,

    /// 0x20000000
    LargePages,
    /// 0x00400000
    Physical,
    /// 0x00100000
    TopDown,
    /// 0x00200000
    WriteWatch,
}

impl Into<u32> for MemAllocation {
    fn into(self) -> u32 {
        match self {
            Self::Commit => 0x00001000,
            Self::Reserve => 0x00002000,
            Self::Reset => 0x00080000,
            Self::ResetUndo => 0x1000000,

            Self::LargePages => 0x20000000,
            Self::Physical => 0x00400000,
            Self::TopDown => 0x00100000,
            Self::WriteWatch => 0x00200000,
        }
    }
}

impl core::ops::BitOr<MemAllocation> for MemAllocation {
    type Output = u32;

    fn bitor(self, rhs: MemAllocation) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<u32> for MemAllocation {
    type Output = u32;

    fn bitor(self, rhs: u32) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<MemAllocation> for u32 {
    type Output = u32;

    fn bitor(self, rhs: MemAllocation) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl PartialEq<u32> for MemAllocation {
    fn eq(&self, other: &u32) -> bool {
        Into::<u32>::into(self.clone()) == *other
    }
}

impl PartialEq<MemAllocation> for u32 {
    fn eq(&self, other: &MemAllocation) -> bool {
        *self == Into::<u32>::into(other.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemFree {
    /// 0x00004000
    Decommit,
    /// 0x00008000
    Release,

    /// 0x00000001
    CoalescePlaceholders,
    /// 0x00000002
    PreservePlaceholder,
}

impl Into<u32> for MemFree {
    fn into(self) -> u32 {
        match self {
            Self::Decommit => 0x00004000,
            Self::Release => 0x00008000,

            Self::CoalescePlaceholders => 0x00000001,
            Self::PreservePlaceholder => 0x00000002,
        }
    }
}

impl core::ops::BitOr<MemFree> for MemFree {
    type Output = u32;

    fn bitor(self, rhs: MemFree) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<u32> for MemFree {
    type Output = u32;

    fn bitor(self, rhs: u32) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<MemFree> for u32 {
    type Output = u32;

    fn bitor(self, rhs: MemFree) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl PartialEq<u32> for MemFree {
    fn eq(&self, other: &u32) -> bool {
        Into::<u32>::into(self.clone()) == *other
    }
}

impl PartialEq<MemFree> for u32 {
    fn eq(&self, other: &MemFree) -> bool {
        *self == Into::<u32>::into(other.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StandardHandle {
    /// 0xFFFFFFF6
    InputHandle,
    /// 0xFFFFFFF5
    OutputHandle,
    /// 0xFFFFFFF4
    ErrorHandle,
}

impl Into<u32> for StandardHandle {
    fn into(self) -> u32 {
        match self {
            Self::InputHandle => 0xFFFFFFF6,
            Self::OutputHandle => 0xFFFFFFF5,
            Self::ErrorHandle => 0xFFFFFFF4,
        }
    }
}

impl core::ops::BitOr<StandardHandle> for StandardHandle {
    type Output = u32;

    fn bitor(self, rhs: StandardHandle) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<u32> for StandardHandle {
    type Output = u32;

    fn bitor(self, rhs: u32) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<StandardHandle> for u32 {
    type Output = u32;

    fn bitor(self, rhs: StandardHandle) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl PartialEq<u32> for StandardHandle {
    fn eq(&self, other: &u32) -> bool {
        Into::<u32>::into(self.clone()) == *other
    }
}

impl PartialEq<StandardHandle> for u32 {
    fn eq(&self, other: &StandardHandle) -> bool {
        *self == Into::<u32>::into(other.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConsoleMode {
    /// 0x0004
    EnableEchoInput,
    /// 0x0020
    EnableInsertMode,
    /// 0x0002
    EnableLineInput,
    /// 0x0010
    EnableMouseInput,
    /// 0x0001
    EnableProcessedInput,
    /// 0x0040
    EnableQuickEditMode,
    /// 0x0008
    EnableWindowInput,
    /// 0x0200
    EnableVirtualTerminalInput,

    /// 0x0001
    EnableProcessedOutput,
    /// 0x0002
    EnableWrapAtEolOutput,
    /// 0x0004
    EnableVirtualTerminalProcessing,
    /// 0x0008
    DisableNewlineAutoReturn,
    /// 0x0010
    EnableLvbGridWorldwide,
}

impl Into<u32> for ConsoleMode {
    fn into(self) -> u32 {
        match self {
            Self::EnableEchoInput => 0x0004,
            Self::EnableInsertMode => 0x0020,
            Self::EnableLineInput => 0x0002,
            Self::EnableMouseInput => 0x0010,
            Self::EnableProcessedInput => 0x0001,
            Self::EnableQuickEditMode => 0x0040,
            Self::EnableWindowInput => 0x0008,
            Self::EnableVirtualTerminalInput => 0x0200,

            Self::EnableProcessedOutput => 0x0001,
            Self::EnableWrapAtEolOutput => 0x0002,
            Self::EnableVirtualTerminalProcessing => 0x0004,
            Self::DisableNewlineAutoReturn => 0x0008,
            Self::EnableLvbGridWorldwide => 0x0010,
        }
    }
}

impl core::ops::BitOr<ConsoleMode> for ConsoleMode {
    type Output = u32;

    fn bitor(self, rhs: ConsoleMode) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<u32> for ConsoleMode {
    type Output = u32;

    fn bitor(self, rhs: u32) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl core::ops::BitOr<ConsoleMode> for u32 {
    type Output = u32;

    fn bitor(self, rhs: ConsoleMode) -> Self::Output {
        Into::<u32>::into(self) | Into::<u32>::into(rhs)
    }
}

impl PartialEq<u32> for ConsoleMode {
    fn eq(&self, other: &u32) -> bool {
        Into::<u32>::into(self.clone()) == *other
    }
}

impl PartialEq<ConsoleMode> for u32 {
    fn eq(&self, other: &ConsoleMode) -> bool {
        *self == Into::<u32>::into(other.clone())
    }
}
