use crate::{HANDLE, HMODULE};

#[doc = "Return value: `Handle`"]
#[inline]
pub unsafe fn open_proc(pid: u32) -> Result<HANDLE, ::std::io::Error> {
    let proc_handle = crate::ffi::OpenProcess(0x1F0FFF, 0, pid);

    if 0 == proc_handle {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(proc_handle)
}

#[doc = r"Return value: `PID`"]
pub unsafe fn get_pid<S: AsRef<str>>(name: S) -> Result<u32, ::std::io::Error> {
    let sh: isize = crate::ffi::CreateToolhelp32Snapshot(0x2, 0x0);

    if sh as isize == -1 {
        return Err(::std::io::Error::last_os_error());
    }

    let mut entry: crate::ffi::ProcessEntry32W =
        ::core::mem::zeroed::<crate::ffi::ProcessEntry32W>();

    entry.dw_size = ::core::mem::size_of::<crate::ffi::ProcessEntry32W>() as u32;

    if 0 == crate::ffi::Process32FirstW(sh, &mut entry) {
        crate::close_handle(sh)?;

        return Err(::std::io::Error::last_os_error());
    }

    if String::from_utf16(&entry.sz_exe_file)
        .map(|ok| ok.trim_end_matches("\0").to_owned())
        .map_err(|err| ::std::io::Error::other(err))?
        .eq_ignore_ascii_case(name.as_ref())
    {
        crate::close_handle(sh)?;

        return Ok(entry.th32_process_id);
    }

    while 0 != crate::ffi::Process32NextW(sh, &mut entry) {
        if String::from_utf16(&entry.sz_exe_file)
            .map(|ok| ok.trim_end_matches("\0").to_owned())
            .map_err(|err| ::std::io::Error::other(err))?
            .eq_ignore_ascii_case(name.as_ref())
        {
            crate::close_handle(sh)?;

            return Ok(entry.th32_process_id);
        }
    }

    crate::close_handle(sh)?;

    Err(::std::io::ErrorKind::NotFound.into())
}

#[doc = "Return value: `Vec<(Name, PID)>`"]
pub unsafe fn get_all_proc_info() -> Result<Vec<(String, u32)>, ::std::io::Error> {
    let sh: isize = crate::ffi::CreateToolhelp32Snapshot(0x2, 0x0);

    if sh as isize == -1 {
        return Err(::std::io::Error::last_os_error());
    }

    let mut entry: crate::ffi::ProcessEntry32W =
        ::core::mem::zeroed::<crate::ffi::ProcessEntry32W>();

    entry.dw_size = ::core::mem::size_of::<crate::ffi::ProcessEntry32W>() as u32;

    if 0 == crate::ffi::Process32FirstW(sh, &mut entry) {
        crate::close_handle(sh)?;

        return Err(::std::io::Error::last_os_error());
    }

    let mut name = String::from_utf16(&entry.sz_exe_file)
        .map(|ok| ok.trim_end_matches("\0").to_owned())
        .map_err(|err| ::std::io::Error::other(err))?;

    let mut entries: Vec<(String, u32)> = Vec::new();

    entries.push((name, entry.th32_process_id));

    while 0 != crate::ffi::Process32NextW(sh, &mut entry) {
        name = String::from_utf16(&entry.sz_exe_file)
            .map(|ok| ok.trim_end_matches("\0").to_owned())
            .map_err(|err| ::std::io::Error::other(err))?;

        entries.push((name, entry.th32_process_id))
    }

    crate::close_handle(sh)?;

    Ok(entries)
}

#[doc = "Return value: `(Handle, BaseAddress, Size)`"]
pub unsafe fn get_mod_info<S: AsRef<str>>(
    pid: u32,
    name: S,
) -> Result<(HMODULE, *mut u8, u32), ::std::io::Error> {
    let sh: isize = crate::ffi::CreateToolhelp32Snapshot(0x8 | 0x10, pid);

    if sh as isize == -1 {
        return Err(::std::io::Error::last_os_error());
    }

    let mut entry: crate::ffi::ModuleEntry32W = ::core::mem::zeroed::<crate::ffi::ModuleEntry32W>();

    entry.dw_size = ::core::mem::size_of::<crate::ffi::ModuleEntry32W>() as u32;

    if 0 == crate::ffi::Module32FirstW(sh, &mut entry) {
        crate::close_handle(sh)?;

        return Err(::std::io::Error::last_os_error());
    }

    if String::from_utf16(&entry.sz_module)
        .map(|ok| ok.trim_end_matches("\0").to_owned())
        .map_err(|err| ::std::io::Error::other(err))?
        .eq_ignore_ascii_case(name.as_ref())
    {
        crate::close_handle(sh)?;

        return Ok((entry.h_module, entry.mod_base_addr, entry.mod_base_size));
    }

    while 0 != crate::ffi::Module32NextW(sh, &mut entry) {
        if String::from_utf16(&entry.sz_module)
            .map(|ok| ok.trim_end_matches("\0").to_owned())
            .map_err(|err| ::std::io::Error::other(err))?
            .eq_ignore_ascii_case(name.as_ref())
        {
            crate::close_handle(sh)?;

            return Ok((entry.h_module, entry.mod_base_addr, entry.mod_base_size));
        }
    }

    crate::close_handle(sh)?;

    Err(::std::io::ErrorKind::NotFound.into())
}

#[doc = "Return value: `Vec<(Name, Handle, BaseAddress, Size)>`"]
pub unsafe fn get_all_mod_info(
    pid: u32,
) -> Result<Vec<(String, HMODULE, *mut u8, u32)>, ::std::io::Error> {
    let sh: isize = crate::ffi::CreateToolhelp32Snapshot(0x8 | 0x10, pid);

    if sh as isize == -1 {
        return Err(::std::io::Error::last_os_error());
    }

    let mut entry: crate::ffi::ModuleEntry32W = ::core::mem::zeroed::<crate::ffi::ModuleEntry32W>();

    entry.dw_size = ::core::mem::size_of::<crate::ffi::ModuleEntry32W>() as u32;

    if 0 == crate::ffi::Module32FirstW(sh, &mut entry) {
        crate::close_handle(sh)?;

        return Err(::std::io::Error::last_os_error());
    }

    let mut name = String::from_utf16(&entry.sz_module)
        .map(|ok| ok.trim_end_matches("\0").to_owned())
        .map_err(|err| ::std::io::Error::other(err))?;

    let mut entries: Vec<(String, HMODULE, *mut u8, u32)> = Vec::new();

    entries.push((
        name,
        entry.h_module,
        entry.mod_base_addr,
        entry.mod_base_size,
    ));

    while 0 != crate::ffi::Module32NextW(sh, &mut entry) {
        name = String::from_utf16(&entry.sz_module)
            .map(|ok| ok.trim_end_matches("\0").to_owned())
            .map_err(|err| ::std::io::Error::other(err))?;

        entries.push((
            name,
            entry.h_module,
            entry.mod_base_addr,
            entry.mod_base_size,
        ))
    }

    crate::close_handle(sh)?;

    Ok(entries)
}

#[doc = "Return value: `Allocated memory address`"]
pub unsafe fn alloc_mem(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    size: usize,
    mem_alloc_ty: u32,
    page_prot_ty: u32,
) -> Result<*mut ::core::ffi::c_void, ::std::io::Error> {
    let addr = crate::ffi::VirtualAllocEx(proc_handle, addr, size, mem_alloc_ty, page_prot_ty);

    if ::core::ptr::null_mut() == addr {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(addr)
}

#[doc = "If the fourth parameter is **RELEASE/0x8000**, the third parameter must be 0"]
pub unsafe fn free_mem(
    proc_handle: HANDLE,
    addr: *mut ::core::ffi::c_void,
    mut size: usize,
    mem_free_ty: u32,
) -> Result<(), ::std::io::Error> {
    if crate::mem_free_ty::RELEASE == mem_free_ty {
        size = 0;
    }

    if 0 == crate::ffi::VirtualFreeEx(proc_handle, addr, size, mem_free_ty) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(())
}

#[doc = "Return value: `(BaseAddress, RegionSize, AllocationProtect, Type, State, Protect)`"]
pub unsafe fn query_mem(
    proc_handle: HANDLE,
    addr: *mut ::core::ffi::c_void,
) -> Result<(*mut ::core::ffi::c_void, usize, u32, u32, u32, u32), ::std::io::Error> {
    let mut mbi: crate::ffi::MemoryBasicInformation =
        ::core::mem::zeroed::<crate::ffi::MemoryBasicInformation>();

    if 0 == crate::ffi::VirtualQueryEx(
        proc_handle,
        addr,
        &mut mbi,
        ::core::mem::size_of::<crate::ffi::MemoryBasicInformation>(),
    ) {
        return Err(::std::io::Error::last_os_error());
    };

    Ok((
        mbi.base_address,
        mbi.region_size,
        mbi.allocation_protect,
        mbi.type_,
        mbi.state,
        mbi.protect,
    ))
}

#[doc = "Return value: `Previous access protection`"]
pub unsafe fn protect_mem(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    size: usize,
    page_prot_ty: u32,
) -> Result<u32, ::std::io::Error> {
    let mut prev_prot: u32 = Default::default();

    if 0 == crate::ffi::VirtualProtectEx(proc_handle, addr, size, page_prot_ty, &mut prev_prot) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(prev_prot)
}

pub unsafe fn inject_dll<S: AsRef<str>>(
    proc_handle: HANDLE,
    path: S,
) -> Result<(), ::std::io::Error> {
    let buf = crate::common::rs_to_cwsb(path.as_ref());

    let len = buf.len() * ::core::mem::size_of::<u16>();

    let proc_name = b"LoadLibraryW\0";

    let proc = crate::ffi::GetProcAddress(
        crate::ffi::GetModuleHandleW(
            String::from("kernel32.dll\0")
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr()
                .cast(),
        ),
        proc_name.as_ptr().cast(),
    );

    if proc.is_null() {
        return Err(::std::io::Error::last_os_error());
    }

    let addr = crate::external::alloc_mem(
        proc_handle,
        ::core::ptr::null_mut(),
        len * ::core::mem::size_of::<u16>(),
        crate::mem_alloc_ty::COMMIT | crate::mem_alloc_ty::RESERVE,
        crate::page_prot_ty::READ_WRITE,
    )?;

    crate::write_mem(proc_handle, addr, &buf)?;

    let th = crate::ffi::CreateRemoteThread(
        proc_handle,
        ::core::ptr::null_mut(),
        0,
        ::core::mem::transmute(proc),
        addr,
        0,
        ::core::ptr::null_mut(),
    );

    if 0 == th {
        let err = ::std::io::Error::last_os_error();

        free_mem(proc_handle, addr, 0, crate::mem_free_ty::RELEASE)?;

        return Err(err);
    }

    crate::ffi::WaitForSingleObject(th, 0xFFFFFFF);

    let mut code = 0;

    if 0 == crate::ffi::GetExitCodeThread(th, &mut code) {
        return Err(::std::io::Error::last_os_error());
    }

    if 0 == code {
        return Err(::std::io::ErrorKind::NotFound.into());
    }

    free_mem(proc_handle, addr, 0, crate::mem_free_ty::RELEASE)?;

    crate::close_handle(th)?;

    Ok(())
}

pub unsafe fn eject_dll(
    proc_handle: HANDLE,
    mod_handle: HMODULE,
    should_exit_thread: bool,
) -> Result<(), ::std::io::Error> {
    let cs: &'static str;

    if should_exit_thread {
        cs = "FreeLibraryAndExitThread\0";
    } else {
        cs = "FreeLibrary\0";
    }

    let proc = crate::ffi::GetProcAddress(
        crate::ffi::GetModuleHandleW(
            String::from("kernel32.dll\0")
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr()
                .cast(),
        ),
        cs.as_ptr().cast(),
    );

    let th = crate::ffi::CreateRemoteThread(
        proc_handle,
        ::core::ptr::null_mut(),
        0,
        ::core::mem::transmute(proc),
        mod_handle as *const ::core::ffi::c_void,
        0,
        ::core::ptr::null_mut(),
    );

    if 0 == th {
        return Err(::std::io::Error::last_os_error());
    }

    crate::ffi::WaitForSingleObject(th, 0xFFFFFFFF);

    if !should_exit_thread {
        let mut code = 0;

        if 0 == crate::ffi::GetExitCodeThread(th, &mut code) {
            return Err(::std::io::Error::last_os_error());
        }

        if 0 == code {
            return Err(::std::io::ErrorKind::NotFound.into());
        }
    }

    crate::close_handle(th)?;

    Ok(())
}
