#[doc = "Constant collection"]
pub mod mem_protect {
    pub const ENCLAVE_DECOMMIT: u32 = 0x1000_0000;

    pub const ENCLAVE_THREAD_CONTROL: u32 = 0x8000_0000;

    pub const ENCLAVE_UNVALIDATED: u32 = 0x2000_0000;

    pub const EXECUTE: u32 = 0x10;

    pub const EXECUTE_READ: u32 = 0x20;

    pub const EXECUTE_READ_WRITE: u32 = 0x40;

    pub const EXECUTE_WRITECOPY: u32 = 0x80;

    pub const GUARD: u32 = 0x100;

    pub const NOACCESS: u32 = 0x01;

    pub const NOCACHE: u32 = 0x200;

    pub const READONLY: u32 = 0x02;

    pub const READ_WRITE: u32 = 0x04;

    pub const TARGETS_INVALID: u32 = 0x4000_0000;

    pub const TARGETS_NO_UPDATE: u32 = 0x4000_0000;

    pub const WRITECOMBINE: u32 = 0x400;

    pub const WRITECOPY: u32 = 0x08;
}

#[doc = "Constant collection"]
pub mod mem_alloc {
    pub const COMMIT: u32 = 0x0000_1000;

    pub const LARGE_PAGES: u32 = 0x2000_0000;

    pub const PHYSICAL: u32 = 0x0040_0000;

    pub const RESERVE: u32 = 0x0000_2000;

    pub const RESET: u32 = 0x0008_0000;

    pub const RESET_UNDO: u32 = 0x0100_0000;

    pub const TOP_DOWN: u32 = 0x0010_0000;

    pub const WRITE_WATCH: u32 = 0x0020_0000;
}

#[doc = "Constant collection"]
pub mod mem_free {
    pub const COALESCE_PLACEHOLDERS: u32 = 0x0000_0001;

    pub const DECOMMIT: u32 = 0x0000_4000;

    pub const PRESERVE_PLACEHOLDER: u32 = 0x0000_0002;

    pub const RELEASE: u32 = 0x00008000;
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcInfo {
    pub name: String,
    pub id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModInfo {
    pub name: String,
    pub handle: crate::HMODULE,
    pub addr: *mut ::core::ffi::c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemInfo {
    pub protect: u32,
    pub state: u32,
    pub region_size: usize,
}
