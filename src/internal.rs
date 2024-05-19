use crate::{AnyResult, HANDLE};

#[doc = "Return value: `Handle`"]
pub unsafe fn get_proc_handle() -> HANDLE {
    crate::ffi::GetCurrentProcess()
}

#[doc = r#"Return value: `ModInfo`

If the parameter is an empty string `""`, retrieve the main module"#]
pub unsafe fn get_mod_info<S: AsRef<str>>(mod_name: S) -> AnyResult<crate::types::ModInfo> {
    let mod_handle;

    if mod_name.as_ref().is_empty() {
        mod_handle = crate::ffi::GetModuleHandleW(::core::ptr::null())
    } else {
        let mod_name_buf = format!("{}\0", mod_name.as_ref())
            .to_string()
            .encode_utf16()
            .collect::<Vec<u16>>();

        mod_handle = crate::ffi::GetModuleHandleW(mod_name_buf.as_ptr())
    };

    if 0 == mod_handle as isize {
        return Err(::std::io::Error::last_os_error().into());
    }

    let mut mod_info = ::core::mem::zeroed::<crate::ffi::ModuleInfo>();

    if 0 == crate::ffi::GetModuleInformation(
        crate::ffi::GetCurrentProcess(),
        mod_handle,
        &mut mod_info,
        ::core::mem::size_of::<crate::ffi::ModuleInfo>() as u32,
    ) {
        return Err(::std::io::Error::last_os_error().into());
    }

    // 7FFF = 32,767
    let mut mod_name_buf = [0u16; 260];

    if 0 == crate::ffi::GetModuleFileNameW(mod_handle, mod_name_buf.as_mut_ptr(), 260) {
        return Err(::std::io::Error::last_os_error().into());
    }

    let mod_name = String::from_utf16(&mod_name_buf)?
        .trim_end_matches("\0")
        .to_owned();

    let mod_path = ::std::path::PathBuf::from(&mod_name);
    let mod_filename = mod_path
        .file_name()
        .ok_or("file_name()")?
        .to_str()
        .ok_or("to_str()")?
        .to_owned();

    Ok(crate::types::ModInfo {
        name: mod_filename,
        handle: mod_handle,
        addr: mod_info.lp_base_of_dll,
        size: mod_info.size_of_image,
    })
}

#[doc = "Return value: `Allocated memory address`"]
pub unsafe fn alloc_mem(
    addr: *const ::core::ffi::c_void,
    size: usize,
    mem_alloc: u32,
    mem_protect: u32,
) -> AnyResult<*mut ::core::ffi::c_void> {
    let addr = crate::ffi::VirtualAlloc(addr, size, mem_alloc, mem_protect);

    if ::core::ptr::null_mut() == addr {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(addr)
}

#[doc = "If the third parameter is **RELEASE/0x8000**, the second parameter must be 0"]
pub unsafe fn free_mem(
    addr: *mut ::core::ffi::c_void,
    mut size: usize,
    mem_free: u32,
) -> AnyResult<()> {
    if crate::types::mem_free::RELEASE == mem_free {
        size = 0;
    }

    if 0 == crate::ffi::VirtualFree(addr, size, mem_free) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(())
}

#[doc = "Return value: `(BaseAddress, RegionSize, AllocationProtect, Type, State, Protect)`"]
pub unsafe fn query_mem(addr: *const ::core::ffi::c_void) -> AnyResult<crate::types::MemInfo> {
    let mut mbi: crate::ffi::MemoryBasicInformation =
        ::core::mem::zeroed::<crate::ffi::MemoryBasicInformation>();

    if 0 == crate::ffi::VirtualQuery(
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
    addr: *const ::core::ffi::c_void,
    size: usize,
    mem_protect: u32,
) -> AnyResult<u32> {
    let mut prev_protect: u32 = 0;

    if 0 == crate::ffi::VirtualProtect(addr, size, mem_protect, &mut prev_protect) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(prev_protect)
}

#[doc = "Return value: `Handle`"]
pub unsafe fn load_dll<S: AsRef<str>>(dll_name: S) -> AnyResult<HANDLE> {
    let dll_name_buf = format!("{}\0", dll_name.as_ref())
        .to_string()
        .encode_utf16()
        .collect::<Vec<u16>>();

    let mod_handle = crate::ffi::LoadLibraryW(dll_name_buf.as_ptr());

    if 0 == mod_handle as isize {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(mod_handle)
}

#[doc = r#"Module reference count decrement

Calling the function from `DllMain` is not safe"#]
pub unsafe fn free_dll(mod_handle: HANDLE) -> AnyResult<()> {
    if 0 == crate::ffi::FreeLibrary(mod_handle) {
        return Err(::std::io::Error::last_os_error().into());
    }

    Ok(())
}

#[doc = "The function allows threads that are executing within a DLL to safely free the DLL in which they are executing and terminate themselves"]
pub unsafe fn free_dll_exit_thread(mod_handle: HANDLE, exit_code: u32) {
    crate::ffi::FreeLibraryAndExitThread(mod_handle, exit_code);
}

#[doc = "Return value: `Final pointer`"]
pub unsafe fn read_multi_pointer(
    mut base_addr: *const ::core::ffi::c_void,
    byte_offsets: &[isize],
) -> AnyResult<*const ::core::ffi::c_void> {
    {
        let mut mbi = query_mem(base_addr)?;

        if mbi.state != crate::types::mem_alloc::COMMIT {
            return Err("The mem is not commit".into());
        }

        protect_mem(
            base_addr,
            0x1000,
            mbi.protect | crate::types::mem_protect::READ_WRITE,
        )?;

        base_addr = base_addr.read() as isize as *const ::core::ffi::c_void;

        protect_mem(base_addr, 0x1000, mbi.protect)?;

        for byte_offset in byte_offsets {
            base_addr = base_addr.byte_offset(*byte_offset);

            mbi = query_mem(base_addr)?;

            if mbi.state != crate::types::mem_alloc::COMMIT {
                return Err("The mem is not commit".into());
            }

            protect_mem(
                base_addr,
                0x1000,
                mbi.protect | crate::types::mem_protect::READ_WRITE,
            )?;

            base_addr = base_addr.read() as isize as *const ::core::ffi::c_void;

            protect_mem(base_addr, 0x1000, mbi.protect)?;
        }

        Ok(base_addr)
    }
}

#[doc = "Return value: `Exec/Read/Write?`"]
pub unsafe fn check_mem_protect(
    addr: *const ::core::ffi::c_void,
    mem_query_protect: crate::types::MemQueryProtect,
) -> AnyResult<bool> {
    let mbi = query_mem(addr)?;

    let is_commit = mbi.state == crate::types::mem_alloc::COMMIT;

    if !is_commit {
        return Err("The mem is not commit".into());
    }

    let protect: bool;

    match mem_query_protect {
        crate::types::MemQueryProtect::READ => {
            protect = mbi.protect
                & (crate::types::mem_protect::READONLY
                    | crate::types::mem_protect::READ_WRITE
                    | crate::types::mem_protect::WRITECOPY
                    | crate::types::mem_protect::EXECUTE_READ
                    | crate::types::mem_protect::EXECUTE_READ_WRITE
                    | crate::types::mem_protect::EXECUTE_WRITECOPY)
                != 0
        }
        crate::types::MemQueryProtect::WRITE => {
            protect = mbi.protect
                & (crate::types::mem_protect::READ_WRITE
                    | crate::types::mem_protect::EXECUTE_READ_WRITE)
                != 0
        }
        crate::types::MemQueryProtect::EXECUTE => {
            protect = mbi.protect
                & (crate::types::mem_protect::EXECUTE
                    | crate::types::mem_protect::EXECUTE_READ
                    | crate::types::mem_protect::EXECUTE_READ_WRITE
                    | crate::types::mem_protect::EXECUTE_WRITECOPY)
                != 0
        }
    };

    Ok(protect && is_commit)
}
