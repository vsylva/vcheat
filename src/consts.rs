pub mod page_protect {

    /// 0x10
    pub const EXECUTE: u32 = 0x10;
    /// 0x20
    pub const EXECUTE_READ: u32 = 0x20;
    /// 0x40
    pub const EXECUTE_READ_WRITE: u32 = 0x40;
    /// 0x80
    pub const EXECUTE_WRITECOPY: u32 = 0x80;
    /// 0x01
    pub const NOACCESS: u32 = 0x01;
    /// 0x02
    pub const READONLY: u32 = 0x02;
    /// 0x04
    pub const READ_WRITE: u32 = 0x04;
    /// 0x08
    pub const WRITECOPY: u32 = 0x08;
    /// 0x40000000
    pub const TARGETS_INVALID: u32 = 0x4000_0000;
    /// 0x40000000
    pub const TARGETS_NO_UPDATE: u32 = 0x4000_0000;

    /// 0x100
    pub const GUARD: u32 = 0x100;
    /// 0x200
    pub const NOCACHE: u32 = 0x200;
    /// 0x400
    pub const WRITECOMBINE: u32 = 0x400;

    /// 0x10000000
    pub const ENCLAVE_DECOMMIT: u32 = 0x1000_0000;
    /// 0x80000000
    pub const ENCLAVE_THREAD_CONTROL: u32 = 0x8000_0000;
    /// 0x20000000
    pub const ENCLAVE_UNVALIDATED: u32 = 0x2000_0000;
}

pub mod mem_allocation {

    /// 0x00001000
    pub const COMMIT: u32 = 0x0000_1000;
    /// 0x00002000
    pub const RESERVE: u32 = 0x0000_2000;
    /// 0x00080000
    pub const RESET: u32 = 0x0008_0000;
    /// 0x1000000
    pub const RESET_UNDO: u32 = 0x0100_0000;

    /// 0x20000000
    pub const LARGE_PAGES: u32 = 0x2000_0000;
    /// 0x00400000
    pub const PHYSICAL: u32 = 0x0040_0000;
    /// 0x00100000
    pub const TOP_DOWN: u32 = 0x0010_0000;
    /// 0x00200000
    pub const WRITE_WATCH: u32 = 0x0020_0000;
}

pub mod mem_free {
    /// 0x00004000
    pub const DECOMMIT: u32 = 0x0000_4000;
    /// 0x00008000
    pub const RELEASE: u32 = 0x00008000;

    /// 0x00000001
    pub const COALESCE_PLACEHOLDERS: u32 = 0x0000_0001;
    /// 0x00000002
    pub const PRESERVE_PLACEHOLDER: u32 = 0x0000_0002;
}

pub mod standard_handle {
    /// 0xFFFFFFF6
    pub const INPUT_HANDLE: u32 = 0xFFFF_FFF6;
    /// 0xFFFFFFF5
    pub const OUTPUT_HANDLE: u32 = 0xFFFFFFF5;
    /// 0xFFFFFFF4
    pub const ERROR_HANDLE: u32 = 0xFFFFFFF4;
}

pub mod console_mode {
    /// 0x0004
    pub const ENABLE_ECHO_INPUT: u32 = 0x0004;
    /// 0x0020
    pub const ENABLE_INSERT_MODE: u32 = 0x0020;
    /// 0x0002
    pub const ENABLE_LINE_INPUT: u32 = 0x0002;
    /// 0x0010
    pub const ENABLE_MOUSE_INPUT: u32 = 0x0010;
    /// 0x0001
    pub const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
    /// 0x0040
    pub const ENABLE_QUICK_EDIT_MODE: u32 = 0x0040;
    /// 0x0008
    pub const ENABLE_WINDOW_INPUT: u32 = 0x0008;
    /// 0x0200
    pub const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0200;

    /// 0x0001
    pub const ENABLE_PROCESSED_OUTPUT: u32 = 0x0001;
    /// 0x0002
    pub const ENABLE_WRAP_AT_EOL_OUTPUT: u32 = 0x0002;
    /// 0x0004
    pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
    /// 0x0008
    pub const DISABLE_NEWLINE_AUTO_RETURN: u32 = 0x0008;
    /// 0x0010
    pub const ENABLE_LVB_GRID_WORLDWIDE: u32 = 0x0010;
}
