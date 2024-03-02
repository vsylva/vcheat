// #![deny(missing_docs)]
#![doc = r#"
[![Crates.io Version](https://img.shields.io/crates/v/vcheat?style=for-the-badge)](https://crates.io/crates/vcheat)
[![Static Badge](https://img.shields.io/badge/Github-vcheat-green?style=for-the-badge)](https://github.com/vSylva/vcheat/)
"#]

#[doc = "Commonly used by `.exe`"]
pub mod external;

#[doc = "Commonly used by `.dll`"]
pub mod internal;

mod common;
mod ffi;

#[doc = "Module where the constant is located"]
pub mod types;

type HMODULE = isize;
type HANDLE = isize;
type BOOL = i32;

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
    let mut buf: Vec<u8> = vec![0; size];

    if 0 == crate::ffi::ReadProcessMemory(
        proc_handle,
        addr,
        buf.as_mut_ptr().cast(),
        size,
        ::core::ptr::null_mut(),
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
    let mut bytes_num_written: usize = 0;

    if 0 == crate::ffi::WriteProcessMemory(
        proc_handle,
        addr,
        buf.as_ptr().cast(),
        ::core::mem::size_of::<T>() * buf.len(),
        &mut bytes_num_written,
    ) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(bytes_num_written)
}

#[doc = r#"Return value: `Bytes num read`

**Supports Generics**"#]
pub unsafe fn read_mem_t<T>(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    buf: *mut T,
    size: usize,
) -> Result<usize, ::std::io::Error> {
    let mut bytes_num_read: usize = 0;

    if 0 == crate::ffi::ReadProcessMemory(proc_handle, addr, buf.cast(), size, &mut bytes_num_read)
    {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(bytes_num_read)
}

#[doc = r#"Return value: `Bytes num written`

**Supports Generics**"#]
pub unsafe fn write_mem_t<T>(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    buf: *const T,
    size: usize,
) -> Result<usize, ::std::io::Error> {
    let mut bytes_num_written: usize = 0;

    if 0 == crate::ffi::WriteProcessMemory(
        proc_handle,
        addr,
        buf.cast(),
        size,
        &mut bytes_num_written,
    ) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(bytes_num_written)
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

#[doc = "Make the console support **colored characters**"]
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
