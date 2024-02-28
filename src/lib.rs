#![doc = r#"
[![Crates.io Version](https://img.shields.io/crates/v/vcheat?style=for-the-badge)](https://crates.io/crates/vcheat)
[![Static Badge](https://img.shields.io/badge/Github-vcheat-green?style=for-the-badge)](https://github.com/vSylva/vcheat/)

Hacking Library

```rust
// tests/external.rs

#[test]
fn get_pid() {
    unsafe {
        vcheat::external::get_pid("explorer.exe").unwrap();
    }
}

#[test]
fn get_all_proc_info() {
    unsafe {
        vcheat::external::get_all_proc_info().unwrap();
    }
}

#[test]
fn get_mod_info() {
    unsafe {
        let proc_id = vcheat::external::get_pid("explorer.exe").unwrap();

        vcheat::external::get_mod_info(proc_id, "explorer.exe").unwrap();
    }
}

#[test]
fn get_all_mod_info() {
    unsafe {
        let proc_id = vcheat::external::get_pid("explorer.exe").unwrap();

        vcheat::external::get_all_mod_info(proc_id).unwrap();
    }
}

#[test]
fn read_write_mem() {
    unsafe {
        let proc_id = vcheat::external::get_pid("explorer.exe").unwrap();

        let (mod_handle, mod_addr, mod_size) =
            vcheat::external::get_mod_info(proc_id, "explorer.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(proc_id).unwrap();

        vcheat::external::protect_mem(
            proc_handle,
            mod_handle as *const ::core::ffi::c_void,
            mod_size as usize,
            vcheat::page_prot_ty::READ_WRITE,
        )
        .unwrap();

        let mod_data = vcheat::read_mem(
            proc_handle,
            mod_handle as *const ::core::ffi::c_void,
            mod_size as usize,
        )
        .unwrap();

        let bnw = vcheat::write_mem(proc_handle, mod_addr.cast(), &mod_data).unwrap();

        assert_eq!(mod_size as usize, bnw);

        vcheat::close_handle(proc_handle).unwrap();
    }
}

#[test]
fn inject_dll() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();
        let proc_handle = vcheat::external::open_proc(pid).unwrap();
        vcheat::external::inject_dll(proc_handle, r"test.dll").unwrap();
        vcheat::close_handle(proc_handle).unwrap();
    }
}

#[test]
fn eject_dll() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();
        let proc_handle = vcheat::external::open_proc(pid).unwrap();
        let (mod_handle, _, _) = vcheat::external::get_mod_info(pid, "test.dll").unwrap();
        vcheat::external::eject_dll(proc_handle, mod_handle, false).unwrap();
        vcheat::external::eject_dll(proc_handle, mod_handle, false).unwrap();
        vcheat::close_handle(proc_handle).unwrap();
    }
}
```
"#]

mod common;
pub mod external;
mod ffi;
pub mod internal;

type HMODULE = isize;
type HANDLE = isize;
type BOOL = i32;

pub mod page_prot_ty {
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

pub mod mem_alloc_ty {
    pub const COMMIT: u32 = 0x0000_1000;

    pub const LARGE_PAGES: u32 = 0x2000_0000;

    pub const PHYSICAL: u32 = 0x0040_0000;

    pub const RESERVE: u32 = 0x0000_2000;

    pub const RESET: u32 = 0x0008_0000;

    pub const RESET_UNDO: u32 = 0x0100_0000;

    pub const TOP_DOWN: u32 = 0x0010_0000;

    pub const WRITE_WATCH: u32 = 0x0020_0000;
}

pub mod mem_free_ty {
    pub const COALESCE_PLACEHOLDERS: u32 = 0x0000_0001;

    pub const DECOMMIT: u32 = 0x0000_4000;

    pub const PRESERVE_PLACEHOLDER: u32 = 0x0000_0002;

    pub const RELEASE: u32 = 0x00008000;
}

#[inline]

pub unsafe fn close_handle(handle: HANDLE) -> Result<(), ::std::io::Error> {
    if 0 == crate::ffi::CloseHandle(handle) {
        return Err(::std::io::Error::last_os_error());
    };

    Ok(())
}

#[doc = r#"Return value: `Offset`

Author: **sonodima**

<https://github.com/sonodima/aobscan/blob/master/src/builder.rs>
<https://github.com/sonodima/aobscan/blob/master/src/pattern.rs>"#]
pub unsafe fn pat_find(pat: &str, data: &[u8]) -> Result<usize, ::std::io::Error> {
    let mut sig: Vec<u8> = Vec::<u8>::new();

    let mut mask: Vec<bool> = Vec::<bool>::new();

    for pair in pat.split_whitespace() {
        if pair == "?" || pair == "??" || pair == "*" || pair == "**" {
            mask.push(false);

            sig.push(0);
        } else {
            let num: u8 =
                u8::from_str_radix(pair, 16).map_err(|err| ::std::io::Error::other(err))?;

            mask.push(true);

            sig.push(num);
        }
    }

    let mut start_offset: usize = mask.iter().take_while(|x| **x == false).count();

    let end_offset: usize = mask.iter().rev().take_while(|x| **x == false).count();

    if start_offset != mask.len() {
        sig = sig[start_offset..sig.len() - end_offset].to_vec();

        mask = mask[start_offset..mask.len() - end_offset].to_vec();
    } else {
        start_offset = 0;
    }

    let first_byte: u8 = sig[0];

    let first_mask: bool = mask[0];

    for i in 0..data.len() - sig.len() {
        if data[i] != first_byte && first_mask {
            continue;
        }

        let data: &[u8] = &data[i..];

        let mut found: bool = true;

        for (i, sig) in sig.iter().enumerate() {
            if !mask[i] {
                continue;
            }

            if data[i] != sig.to_owned() {
                found = false;

                break;
            }
        }

        if found {
            return Ok(i - start_offset);
        }
    }

    Err(::std::io::ErrorKind::NotFound.into())
}

#[doc = r#"Return value: `Vec<Offset>`

Author: **sonodima**

<https://github.com/sonodima/aobscan/blob/master/src/builder.rs>
<https://github.com/sonodima/aobscan/blob/master/src/pattern.rs>"#]
pub unsafe fn pat_scan(pat: &str, data: &[u8]) -> Result<Vec<usize>, ::std::io::Error> {
    let mut sig: Vec<u8> = Vec::<u8>::new();

    let mut mask: Vec<bool> = Vec::<bool>::new();

    for pair in pat.split_whitespace() {
        if pair == "?" || pair == "??" || pair == "*" || pair == "**" {
            mask.push(false);

            sig.push(0);
        } else {
            let num: u8 =
                u8::from_str_radix(pair, 16).map_err(|err| ::std::io::Error::other(err))?;

            mask.push(true);

            sig.push(num);
        }
    }

    let mut start_offset: usize = mask.iter().take_while(|x| **x == false).count();

    let end_offset: usize = mask.iter().rev().take_while(|x| **x == false).count();

    if start_offset != mask.len() {
        sig = sig[start_offset..sig.len() - end_offset].to_vec();

        mask = mask[start_offset..mask.len() - end_offset].to_vec();
    } else {
        start_offset = 0;
    }

    let first_byte: u8 = sig[0];

    let first_mask: bool = mask[0];

    let mut offset_list: Vec<usize> = Vec::new();

    for i in 0..data.len() - sig.len() {
        if data[i] != first_byte && first_mask {
            continue;
        }

        let data: &[u8] = &data[i..];

        let mut found: bool = true;

        for (i, sig) in sig.iter().enumerate() {
            if !mask[i] {
                continue;
            }

            if data[i] != sig.to_owned() {
                found = false;

                break;
            }
        }

        if found {
            offset_list.push(i - start_offset);
        }
    }

    Ok(offset_list)
}

#[doc = "Return value: `Vec<u8>`"]
pub unsafe fn read_mem(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    size: usize,
) -> Result<Vec<u8>, ::std::io::Error> {
    let mut buf = vec![0; size];

    let mut num_read: usize = Default::default();

    if 0 == crate::ffi::ReadProcessMemory(
        proc_handle,
        addr,
        buf.as_mut_ptr().cast(),
        size,
        &mut num_read,
    ) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(buf)
}

#[doc = "Return value: `Bytes num written`"]
pub unsafe fn write_mem<T>(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    buf: &[T],
) -> Result<usize, ::std::io::Error> {
    let mut bnw: usize = 0;

    if 0 == crate::ffi::WriteProcessMemory(
        proc_handle,
        addr,
        buf.as_ptr() as *const ::core::ffi::c_void,
        ::core::mem::size_of::<T>() * buf.len(),
        &mut bnw,
    ) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(bnw)
}

pub unsafe fn alloc_console() -> Result<(), ::std::io::Error> {
    if 0 == ffi::AllocConsole() {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(())
}

pub unsafe fn free_console() -> Result<(), ::std::io::Error> {
    if 0 == ffi::FreeConsole() {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(())
}

pub unsafe fn colored_console() -> Result<(), ::std::io::Error> {
    let handle: HANDLE = ffi::GetStdHandle(0xFFFFFFF5);

    if -1 == handle as isize {
        return Err(::std::io::Error::last_os_error());
    }

    let mut mode: u32 = 0;

    if 0 == ffi::GetConsoleMode(handle, &mut mode) {
        return Err(::std::io::Error::last_os_error());
    }

    if 0 == ffi::SetConsoleMode(handle, mode | 4) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(())
}
