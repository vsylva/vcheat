// #![deny(missing_docs)]
#![doc = r#"
[![Crates.io Version](https://img.shields.io/crates/v/vcheat?style=for-the-badge)](https://crates.io/crates/vcheat)
[![Static Badge](https://img.shields.io/badge/Github-vcheat-green?style=for-the-badge)](https://github.com/vSylva/vcheat/)
"#]

mod ffi;

#[doc = "Location of memory constants"]
pub mod types;

#[doc = "Commonly used by `.exe`"]
pub mod external;

#[doc = "Commonly used by `.dll`"]
pub mod internal;

pub type AnyResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type HANDLE = isize;
pub type BOOL = i32;

#[doc = r"Return value: `Offset`"]
pub unsafe fn pat_find<S: AsRef<str>>(pat: S, data: &[u8]) -> AnyResult<usize> {
    let mut pat_bytes: Vec<u8> = Vec::<u8>::new();

    for pair in pat.as_ref().split_whitespace() {
        if pair == "?" || pair == "??" || pair == "*" || pair == "**" {
            pat_bytes.push(0);
        } else {
            let num: u8 = u8::from_str_radix(pair, 16)?;

            pat_bytes.push(num);
        }
    }

    let data_len = data.len();
    let pat_bytes_len = pat_bytes.len();

    let mut skip_table = [pat_bytes.len(); 256];

    for (i, byte) in pat_bytes.iter().enumerate().take(pat_bytes.len() - 1).rev() {
        if skip_table[*byte as usize] == pat_bytes.len() {
            skip_table[*byte as usize] = pat_bytes.len() - 1 - i;
        }
    }

    let mut i = pat_bytes_len - 1;

    while i < data_len {
        let mut j = pat_bytes_len - 1;
        let mut k = i;

        while j > 0 && (data[k] == pat_bytes[j] || pat_bytes[j] == 0) {
            k -= 1;
            j -= 1;
        }

        if j == 0 && (data[k] == pat_bytes[j] || pat_bytes[j] == 0) {
            return Ok(k);
        }

        i += skip_table[data[i] as usize];
    }

    Err("\"pat\" not found".into())
}

#[doc = r"Return value: `Vec<Offset>`"]
pub unsafe fn pat_scan<S: AsRef<str>>(pat: S, data: &[u8]) -> AnyResult<Vec<usize>> {
    let mut pat_bytes: Vec<u8> = Vec::<u8>::new();

    for pair in pat.as_ref().split_whitespace() {
        if pair == "?" || pair == "??" || pair == "*" || pair == "**" {
            pat_bytes.push(0);
        } else {
            let num: u8 = u8::from_str_radix(pair, 16)?;

            pat_bytes.push(num);
        }
    }

    let data_len = data.len();
    let pat_bytes_len = pat_bytes.len();

    let mut skip_table = [pat_bytes.len(); 256];

    for (i, byte) in pat_bytes.iter().enumerate().take(pat_bytes.len() - 1).rev() {
        if skip_table[*byte as usize] == pat_bytes.len() {
            skip_table[*byte as usize] = pat_bytes.len() - 1 - i;
        }
    }

    let mut i = pat_bytes_len - 1;

    let mut offset_array = Vec::<usize>::new();

    while i < data_len {
        let mut j = pat_bytes_len - 1;
        let mut k = i;

        while j > 0 && (data[k] == pat_bytes[j] || pat_bytes[j] == 0) {
            k -= 1;
            j -= 1;
        }

        if j == 0 && (data[k] == pat_bytes[j] || pat_bytes[j] == 0) {
            offset_array.push(k);
        }

        i += skip_table[data[i] as usize];
    }

    Ok(offset_array)
}

#[doc = "Return value: `Vec<u8>`"]
pub unsafe fn read_mem(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    size: usize,
) -> AnyResult<Vec<u8>> {
    let mut buf: Vec<u8> = vec![0; size];

    if 0 == crate::ffi::ReadProcessMemory(
        proc_handle,
        addr,
        buf.as_mut_ptr().cast(),
        size,
        ::core::ptr::null_mut(),
    ) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(buf)
}

#[doc = "Return value: `Bytes num written`"]
pub unsafe fn write_mem<T>(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    buf: &[T],
) -> AnyResult<usize> {
    let mut bytes_num_written: usize = 0;

    if 0 == crate::ffi::WriteProcessMemory(
        proc_handle,
        addr,
        buf.as_ptr().cast(),
        ::core::mem::size_of::<T>() * buf.len(),
        &mut bytes_num_written,
    ) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(bytes_num_written)
}

#[doc = r#"Return value: `Bytes num written`

`hex_str: "0A 1B 2C 3D 4E 5F FF"`"#]
pub unsafe fn write_mem_hex_str<S: AsRef<str>>(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    hex_str: S,
) -> AnyResult<usize> {
    let mut bytes_num_written: usize = 0;

    let mut bytes: Vec<u8> = Vec::<u8>::new();

    for c in hex_str.as_ref().split_whitespace() {
        let num: u8 = u8::from_str_radix(c, 16)?;

        bytes.push(num);
    }

    if 0 == crate::ffi::WriteProcessMemory(
        proc_handle,
        addr,
        bytes.as_ptr().cast(),
        bytes.len(),
        &mut bytes_num_written,
    ) {
        return Err(::std::io::Error::last_os_error().into());
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
) -> AnyResult<usize> {
    let mut bytes_num_read: usize = 0;

    if 0 == crate::ffi::ReadProcessMemory(proc_handle, addr, buf.cast(), size, &mut bytes_num_read)
    {
        return Err(::std::io::Error::last_os_error().into());
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
) -> AnyResult<usize> {
    let mut bytes_num_written: usize = 0;

    if 0 == crate::ffi::WriteProcessMemory(
        proc_handle,
        addr,
        buf.cast(),
        size,
        &mut bytes_num_written,
    ) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(bytes_num_written)
}

pub unsafe fn alloc_console() -> AnyResult<()> {
    if 0 == ffi::AllocConsole() {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(())
}

pub unsafe fn free_console() -> AnyResult<()> {
    if 0 == ffi::FreeConsole() {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(())
}

#[doc = "Make the console support **colored characters**"]
pub unsafe fn colored_console() -> AnyResult<()> {
    let handle: HANDLE = ffi::GetStdHandle(0xFFFFFFF5);

    if -1 == handle as isize {
        return Err(::std::io::Error::last_os_error().into());
    }

    let mut mode: u32 = 0;

    if 0 == ffi::GetConsoleMode(handle, &mut mode) {
        return Err(::std::io::Error::last_os_error().into());
    }

    if 0 == ffi::SetConsoleMode(handle, mode | 4) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(())
}

pub unsafe fn get_proc_address<S: AsRef<str>>(
    mod_handle: HANDLE,
    proc_name: S,
) -> AnyResult<HANDLE> {
    let proc_addr = crate::ffi::GetProcAddress(
        mod_handle,
        format!("{}\0", proc_name.as_ref()).as_ptr().cast(),
    );

    if 0 == proc_addr as HANDLE {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(proc_addr)
}
