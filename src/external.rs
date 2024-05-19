use crate::{AnyResult, HANDLE};

#[doc = "Return value: `Handle`"]
#[inline]
pub unsafe fn open_proc(pid: u32) -> AnyResult<HANDLE> {
    let proc_handle = crate::ffi::OpenProcess(0x1F0FFF, 0, pid);

    if 0 == proc_handle as isize {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(proc_handle)
}

#[inline]
pub unsafe fn close_handle(handle: HANDLE) -> AnyResult<()> {
    if 0 == crate::ffi::CloseHandle(handle) {
        return Err(::std::io::Error::last_os_error().into());
    };

    Ok(())
}

#[doc = "Return value: `Process id`"]
pub unsafe fn get_pid<S: AsRef<str>>(proc_name: S) -> AnyResult<u32> {
    let snapshot_handle: HANDLE = crate::ffi::CreateToolhelp32Snapshot(0x2, 0x0);

    if snapshot_handle as isize == -1 {
        return Err(::std::io::Error::last_os_error().into());
    }

    let mut proc_info: crate::ffi::ProcessEntry32W =
        ::core::mem::zeroed::<crate::ffi::ProcessEntry32W>();

    proc_info.dw_size = ::core::mem::size_of::<crate::ffi::ProcessEntry32W>() as u32;

    if 0 == crate::ffi::Process32FirstW(snapshot_handle, &mut proc_info) {
        close_handle(snapshot_handle)?;

        return Err(::std::io::Error::last_os_error().into());
    }

    if String::from_utf16(&proc_info.sz_exe_file)?
        .trim_end_matches("\0")
        .eq_ignore_ascii_case(proc_name.as_ref())
    {
        close_handle(snapshot_handle)?;

        return Ok(proc_info.th32_process_id);
    }

    while 0 != crate::ffi::Process32NextW(snapshot_handle, &mut proc_info) {
        if String::from_utf16(&proc_info.sz_exe_file)?
            .trim_end_matches("\0")
            .eq_ignore_ascii_case(proc_name.as_ref())
        {
            close_handle(snapshot_handle)?;

            return Ok(proc_info.th32_process_id);
        }
    }

    close_handle(snapshot_handle)?;

    Err(format!("{} not found", proc_name.as_ref()).into())
}

#[doc = "Return value: `Vec<types::ProcInfo>`"]
pub unsafe fn get_all_proc_info() -> AnyResult<Vec<crate::types::ProcInfo>> {
    let snapshot_handle: HANDLE = crate::ffi::CreateToolhelp32Snapshot(0x2, 0x0);

    if snapshot_handle as isize == -1 {
        return Err(::std::io::Error::last_os_error().into());
    }

    let mut proc_info: crate::ffi::ProcessEntry32W =
        ::core::mem::zeroed::<crate::ffi::ProcessEntry32W>();

    proc_info.dw_size = ::core::mem::size_of::<crate::ffi::ProcessEntry32W>() as u32;

    if 0 == crate::ffi::Process32FirstW(snapshot_handle, &mut proc_info) {
        close_handle(snapshot_handle)?;

        return Err(::std::io::Error::last_os_error().into());
    }

    let mut proc_name = String::from_utf16(&proc_info.sz_exe_file)?
        .trim_end_matches("\0")
        .to_owned();

    let mut procs_info: Vec<crate::types::ProcInfo> = Vec::new();

    procs_info.push(crate::types::ProcInfo {
        name: proc_name,
        id: proc_info.th32_process_id,
    });

    while 0 != crate::ffi::Process32NextW(snapshot_handle, &mut proc_info) {
        proc_name = String::from_utf16(&proc_info.sz_exe_file)?
            .trim_end_matches("\0")
            .to_owned();

        procs_info.push(crate::types::ProcInfo {
            name: proc_name,
            id: proc_info.th32_process_id,
        })
    }

    close_handle(snapshot_handle)?;

    Ok(procs_info)
}

#[doc = "Return value: `ModInfo`"]
pub unsafe fn get_mod_info<S: AsRef<str>>(
    pid: u32,
    mod_name: S,
) -> AnyResult<crate::types::ModInfo> {
    let snapshot_handle: HANDLE = crate::ffi::CreateToolhelp32Snapshot(0x8 | 0x10, pid);

    if snapshot_handle as isize == -1 {
        return Err(::std::io::Error::last_os_error().into());
    }

    let mut mod_info: crate::ffi::ModuleEntry32W =
        ::core::mem::zeroed::<crate::ffi::ModuleEntry32W>();

    mod_info.dw_size = ::core::mem::size_of::<crate::ffi::ModuleEntry32W>() as u32;

    if 0 == crate::ffi::Module32FirstW(snapshot_handle, &mut mod_info) {
        close_handle(snapshot_handle)?;

        return Err(::std::io::Error::last_os_error().into());
    }

    let mut mod_name_ = String::from_utf16(&mod_info.sz_module)?
        .trim_end_matches("\0")
        .to_owned();

    if mod_name_.eq_ignore_ascii_case(mod_name.as_ref()) {
        close_handle(snapshot_handle)?;

        return Ok(crate::types::ModInfo {
            name: mod_name_,
            handle: mod_info.h_module,
            addr: mod_info.mod_base_addr.cast(),
            size: mod_info.mod_base_size,
        });
    }

    while 0 != crate::ffi::Module32NextW(snapshot_handle, &mut mod_info) {
        mod_name_ = String::from_utf16(&mod_info.sz_module)?
            .trim_end_matches("\0")
            .to_owned();

        if mod_name_.eq_ignore_ascii_case(mod_name.as_ref()) {
            close_handle(snapshot_handle)?;

            return Ok(crate::types::ModInfo {
                name: mod_name_,
                handle: mod_info.h_module,
                addr: mod_info.mod_base_addr.cast(),
                size: mod_info.mod_base_size,
            });
        }
    }

    close_handle(snapshot_handle)?;

    Err(format!("{} not found", mod_name.as_ref()).into())
}

#[doc = "Return value: `Vec<ModInfo>`"]
pub unsafe fn get_all_mod_info(pid: u32) -> AnyResult<Vec<crate::types::ModInfo>> {
    let snapshot_handle: HANDLE = crate::ffi::CreateToolhelp32Snapshot(0x8 | 0x10, pid);

    if snapshot_handle as isize == -1 {
        return Err(::std::io::Error::last_os_error().into());
    }

    let mut mod_info: crate::ffi::ModuleEntry32W =
        ::core::mem::zeroed::<crate::ffi::ModuleEntry32W>();

    mod_info.dw_size = ::core::mem::size_of::<crate::ffi::ModuleEntry32W>() as u32;

    if 0 == crate::ffi::Module32FirstW(snapshot_handle, &mut mod_info) {
        close_handle(snapshot_handle)?;

        return Err(::std::io::Error::last_os_error().into());
    }

    let mut mod_name = String::from_utf16(&mod_info.sz_module)?
        .trim_end_matches("\0")
        .to_owned();

    let mut mods_info: Vec<crate::types::ModInfo> = Vec::new();

    mods_info.push(crate::types::ModInfo {
        name: mod_name,
        handle: mod_info.h_module,
        addr: mod_info.mod_base_addr.cast(),
        size: mod_info.mod_base_size,
    });

    while 0 != crate::ffi::Module32NextW(snapshot_handle, &mut mod_info) {
        mod_name = String::from_utf16(&mod_info.sz_module)?
            .trim_end_matches("\0")
            .to_owned();

        mods_info.push(crate::types::ModInfo {
            name: mod_name,
            handle: mod_info.h_module,
            addr: mod_info.mod_base_addr.cast(),
            size: mod_info.mod_base_size,
        })
    }

    close_handle(snapshot_handle)?;

    Ok(mods_info)
}

#[doc = "Return value: `Allocated memory address`"]
pub unsafe fn alloc_mem(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    size: usize,
    mem_alloc: u32,
    mem_protect: u32,
) -> AnyResult<*mut ::core::ffi::c_void> {
    let addr = crate::ffi::VirtualAllocEx(proc_handle, addr, size, mem_alloc, mem_protect);

    if ::core::ptr::null_mut() == addr {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(addr)
}

#[doc = "If the fourth parameter is **RELEASE/0x8000**, the third parameter must be 0"]
pub unsafe fn free_mem(
    proc_handle: HANDLE,
    addr: *mut ::core::ffi::c_void,
    mut size: usize,
    mem_free: u32,
) -> AnyResult<()> {
    if crate::types::mem_free::RELEASE == mem_free {
        size = 0;
    }

    if 0 == crate::ffi::VirtualFreeEx(proc_handle, addr, size, mem_free) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(())
}

#[doc = "Return value: `(BaseAddress, RegionSize, AllocationProtect, Type, State, Protect)`"]
pub unsafe fn query_mem(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
) -> AnyResult<crate::types::MemInfo> {
    let mut mbi: crate::ffi::MemoryBasicInformation =
        ::core::mem::zeroed::<crate::ffi::MemoryBasicInformation>();

    if 0 == crate::ffi::VirtualQueryEx(
        proc_handle,
        addr,
        &mut mbi,
        ::core::mem::size_of::<crate::ffi::MemoryBasicInformation>(),
    ) {
        return Err(::std::io::Error::last_os_error().into());
    };

    Ok(crate::types::MemInfo {
        protect: mbi.protect,
        state: mbi.state,
        region_size: mbi.region_size,
    })
}

pub unsafe fn protect_mem(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    size: usize,
    mem_protect: u32,
) -> AnyResult<u32> {
    let mut prev_prot: u32 = Default::default();

    if 0 == crate::ffi::VirtualProtectEx(proc_handle, addr, size, mem_protect, &mut prev_prot) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(prev_prot)
}

#[doc = "Remote DLL Injection"]
pub unsafe fn inject_dll<S: AsRef<str>>(proc_handle: HANDLE, dll_path: S) -> AnyResult<()> {
    let dll_path_buf = format!("{}\0", dll_path.as_ref())
        .to_string()
        .encode_utf16()
        .collect::<Vec<u16>>();

    let len = dll_path_buf.len() * ::core::mem::size_of::<u16>();

    let proc_name = b"LoadLibraryW\0";

    let procedure = crate::ffi::GetProcAddress(
        crate::ffi::GetModuleHandleW(
            String::from("kernel32.dll\0")
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr(),
        ),
        proc_name.as_ptr(),
    );

    if 0 == procedure as isize {
        return Err(::std::io::Error::last_os_error().into());
    }

    let addr = crate::external::alloc_mem(
        proc_handle,
        ::core::ptr::null_mut(),
        len * ::core::mem::size_of::<u16>(),
        crate::types::mem_alloc::COMMIT | crate::types::mem_alloc::RESERVE,
        crate::types::mem_protect::READ_WRITE,
    )?;

    crate::write_mem(proc_handle, addr, &dll_path_buf)?;

    let thread_handle = crate::ffi::CreateRemoteThread(
        proc_handle,
        ::core::ptr::null_mut(),
        0,
        procedure,
        addr,
        0,
        ::core::ptr::null_mut(),
    );

    if 0 == thread_handle as isize {
        let err = ::std::io::Error::last_os_error();

        free_mem(proc_handle, addr, 0, crate::types::mem_free::RELEASE)?;

        return Err(err.into());
    }

    crate::ffi::WaitForSingleObject(thread_handle, 0xFFFFFFF);

    let mut code = 0;

    if 0 == crate::ffi::GetExitCodeThread(thread_handle, &mut code) {
        return Err(::std::io::Error::last_os_error().into());
    }

    if 0 == code {
        return Err(::std::io::ErrorKind::InvalidInput.to_string().into());
    }

    free_mem(proc_handle, addr, 0, crate::types::mem_free::RELEASE)?;

    close_handle(thread_handle)?;

    Ok(())
}

#[doc = "Remote DLL Ejection"]
pub unsafe fn eject_dll(
    proc_handle: HANDLE,
    mod_handle: HANDLE,
    should_exit_thread: bool,
) -> AnyResult<()> {
    let c_str: &'static str;

    if should_exit_thread {
        c_str = "FreeLibraryAndExitThread\0";
    } else {
        c_str = "FreeLibrary\0";
    }

    let procedure = crate::ffi::GetProcAddress(
        crate::ffi::GetModuleHandleW(
            String::from("kernel32.dll\0")
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr(),
        ),
        c_str.as_ptr(),
    );

    let thread_handle = crate::ffi::CreateRemoteThread(
        proc_handle,
        ::core::ptr::null_mut(),
        0,
        procedure,
        mod_handle as *const ::core::ffi::c_void,
        0,
        ::core::ptr::null_mut(),
    );

    if 0 == thread_handle as isize {
        return Err(::std::io::Error::last_os_error().into());
    }

    crate::ffi::WaitForSingleObject(thread_handle, 0xFFFFFFFF);

    if !should_exit_thread {
        let mut code = 0;

        if 0 == crate::ffi::GetExitCodeThread(thread_handle, &mut code) {
            return Err(::std::io::Error::last_os_error().into());
        }

        if 0 == code {
            return Err(::std::io::ErrorKind::InvalidInput.to_string().into());
        }
    }

    close_handle(thread_handle)?;

    Ok(())
}

#[doc = "Return value: `Final pointer`"]
pub unsafe fn read_multi_pointer(
    proc_handle: HANDLE,
    mut base_addr: *const ::core::ffi::c_void,
    byte_offsets: &[isize],
) -> AnyResult<*const ::core::ffi::c_void> {
    {
        let mut mbi = query_mem(proc_handle, base_addr)?;

        if mbi.state != crate::types::mem_alloc::COMMIT {
            return Err("The mem is not commit".into());
        }

        protect_mem(
            proc_handle,
            base_addr,
            0x1000,
            mbi.protect | crate::types::mem_protect::READ_WRITE,
        )?;

        base_addr = base_addr.read() as isize as *const ::core::ffi::c_void;

        protect_mem(proc_handle, base_addr, 0x1000, mbi.protect)?;

        for byte_offset in byte_offsets {
            base_addr = base_addr.byte_offset(*byte_offset);

            mbi = query_mem(proc_handle, base_addr)?;

            if mbi.state != crate::types::mem_alloc::COMMIT {
                return Err("The mem is not commit".into());
            }

            protect_mem(
                proc_handle,
                base_addr,
                0x1000,
                mbi.protect | crate::types::mem_protect::READ_WRITE,
            )?;

            base_addr = base_addr.read() as isize as *const ::core::ffi::c_void;

            protect_mem(proc_handle, base_addr, 0x1000, mbi.protect)?;
        }

        Ok(base_addr)
    }
}

#[doc = "Return value: `Exec/Read/Write?`"]
pub unsafe fn check_mem_protect(
    proc_handle: HANDLE,
    addr: *const ::core::ffi::c_void,
    mem_query_protect: crate::types::MemQueryProtect,
) -> AnyResult<bool> {
    let mbi = query_mem(proc_handle, addr)?;

    let is_commit = mbi.state == crate::types::mem_alloc::COMMIT;

    if !is_commit {
        return Err("The mem is not commit".into());
    }

    let is_protect_able: bool;

    match mem_query_protect {
        crate::types::MemQueryProtect::READ => {
            is_protect_able = mbi.protect
                & (crate::types::mem_protect::READONLY
                    | crate::types::mem_protect::READ_WRITE
                    | crate::types::mem_protect::WRITECOPY
                    | crate::types::mem_protect::EXECUTE_READ
                    | crate::types::mem_protect::EXECUTE_READ_WRITE
                    | crate::types::mem_protect::EXECUTE_WRITECOPY)
                != 0
        }
        crate::types::MemQueryProtect::WRITE => {
            is_protect_able = mbi.protect
                & (crate::types::mem_protect::READ_WRITE
                    | crate::types::mem_protect::EXECUTE_READ_WRITE)
                != 0
        }
        crate::types::MemQueryProtect::EXECUTE => {
            is_protect_able = mbi.protect
                & (crate::types::mem_protect::EXECUTE
                    | crate::types::mem_protect::EXECUTE_READ
                    | crate::types::mem_protect::EXECUTE_READ_WRITE
                    | crate::types::mem_protect::EXECUTE_WRITECOPY)
                != 0
        }
    };

    Ok(is_protect_able && is_commit)
}
