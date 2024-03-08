use crate::{HANDLE, HMODULE};

#[doc = "Return value: `Handle`"]
pub unsafe fn get_proc_handle() -> HANDLE {
    crate::ffi::GetCurrentProcess()
}

// #[doc = r#"Return value: `Handle`

// If the parameter is **empty**, the function returns information about the file used to create the calling process (.exe file)

// The function does not increase the module reference count"#]
// unsafe fn get_mod_handle<S: AsRef<str>>(mod_name: S) -> Result<HMODULE, ::std::io::Error> {
//     let mod_name_ptr;

//     let mod_name_buf;

//     if mod_name.as_ref().is_empty() {
//         mod_name_ptr = ::core::ptr::null();
//     } else {
//         mod_name_buf = crate::common::rs_to_cwsb(mod_name.as_ref());
//         mod_name_ptr = mod_name_buf.as_ptr();
//     }

//     let mod_handle = crate::ffi::GetModuleHandleW(mod_name_ptr);

//     if 0 == mod_handle {
//         return Err(::std::io::Error::last_os_error());
//     }

//     Ok(mod_handle)
// }

#[doc = "Return value: `ModInfo`"]
pub unsafe fn get_mod_info<S: AsRef<str>>(
    mod_name: S,
) -> Result<crate::types::ModInfo, ::std::io::Error> {
    let mod_name_ptr;

    let mod_name_buf;

    if mod_name.as_ref().is_empty() {
        mod_name_ptr = ::core::ptr::null();
    } else {
        mod_name_buf = crate::common::rs_to_cwsb(mod_name.as_ref());
        mod_name_ptr = mod_name_buf.as_ptr();
    }

    let mod_handle = crate::ffi::GetModuleHandleW(mod_name_ptr);

    if 0 == mod_handle {
        return Err(::std::io::Error::last_os_error());
    }

    // let mod_handle = get_mod_handle(mod_name.as_ref())?;

    let mut mod_info = ::core::mem::zeroed::<crate::ffi::MODULEINFO>();

    if 0 == crate::ffi::GetModuleInformation(
        crate::ffi::GetCurrentProcess(),
        mod_handle,
        &mut mod_info,
        ::core::mem::size_of::<crate::ffi::MODULEINFO>() as u32,
    ) {
        return Err(::std::io::Error::last_os_error());
    }

    // 7FFF = 32,767
    let mut mod_name_buf = [0u16; 0x7FFF];

    if 0 == crate::ffi::GetModuleFileNameW(mod_handle, mod_name_buf.as_mut_ptr(), 0x7FFF) {
        return Err(::std::io::Error::last_os_error());
    }

    let mod_name = String::from_utf16(&mod_name_buf)
        .map_err(|err| ::std::io::Error::other(err))?
        .trim_end_matches("\0")
        .to_owned();

    let mod_path = ::std::path::PathBuf::from(&mod_name);
    let mod_filename = mod_path
        .file_name()
        .ok_or(::std::io::Error::other("file_name()"))?
        .to_str()
        .ok_or(::std::io::Error::other("file_name()to_str()"))?
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
) -> Result<*mut ::core::ffi::c_void, ::std::io::Error> {
    let addr = crate::ffi::VirtualAlloc(addr, size, mem_alloc, mem_protect);

    if ::core::ptr::null_mut() == addr {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(addr)
}

#[doc = "If the third parameter is **RELEASE/0x8000**, the second parameter must be 0"]
pub unsafe fn free_mem(
    addr: *mut ::core::ffi::c_void,
    mut size: usize,
    mem_free: u32,
) -> Result<(), ::std::io::Error> {
    if crate::types::mem_free::RELEASE == mem_free {
        size = 0;
    }

    if 0 == crate::ffi::VirtualFree(addr, size, mem_free) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(())
}

#[doc = "Return value: `(BaseAddress, RegionSize, AllocationProtect, Type, State, Protect)`"]
pub unsafe fn query_mem(
    addr: *const ::core::ffi::c_void,
) -> Result<crate::types::MemInfo, ::std::io::Error> {
    let mut mbi: crate::ffi::MemoryBasicInformation =
        ::core::mem::zeroed::<crate::ffi::MemoryBasicInformation>();

    if 0 == crate::ffi::VirtualQuery(
        addr,
        &mut mbi,
        ::core::mem::size_of::<crate::ffi::MemoryBasicInformation>(),
    ) {
        return Err(::std::io::Error::last_os_error());
    };

    Ok(crate::types::MemInfo {
        protect: mbi.protect,
        state: mbi.state,
        region_size: mbi.region_size,
    })
}

#[doc = "Return value: `Previous access protection`"]
pub unsafe fn protect_mem(
    addr: *const ::core::ffi::c_void,
    size: usize,
    mem_protect: u32,
) -> Result<u32, ::std::io::Error> {
    let mut prev_protect: u32 = 0;

    if 0 == crate::ffi::VirtualProtect(addr, size, mem_protect, &mut prev_protect) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(prev_protect)
}

#[doc = "Return value: `Handle`"]
pub unsafe fn load_dll<S: AsRef<str>>(name: S) -> Result<HMODULE, ::std::io::Error> {
    let mod_name_buf = crate::common::rs_to_cwsb(name.as_ref());

    let mod_handle = crate::ffi::LoadLibraryW(mod_name_buf.as_ptr());

    if 0 == mod_handle {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(mod_handle)
}

#[doc = r#"Module reference count decrement

Calling the function from `DllMain` is not safe"#]
pub unsafe fn free_dll(mod_handle: HMODULE) -> Result<(), ::std::io::Error> {
    if 0 == crate::ffi::FreeLibrary(mod_handle) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(())
}

#[doc = "The function allows threads that are executing within a DLL to safely free the DLL in which they are executing and terminate themselves"]
pub unsafe fn free_dll_exit_thread(mod_handle: HMODULE, exit_code: u32) {
    crate::ffi::FreeLibraryAndExitThread(mod_handle, exit_code);
}

#[doc = "Return value: `Final pointer`"]
pub unsafe fn read_multi_pointer(
    mut base_addr: *const ::core::ffi::c_void,
    offsets: &[isize],
) -> Result<*const ::core::ffi::c_void, std::io::Error> {
    {
        let mut mbi = query_mem(base_addr)?;

        let mut is_mem_readable = mbi.state == crate::types::mem_alloc::COMMIT
            && mbi.protect & crate::types::mem_protect::READONLY
                | crate::types::mem_protect::READ_WRITE
                | crate::types::mem_protect::EXECUTE_READ
                | crate::types::mem_protect::EXECUTE_READ_WRITE
                != 0;

        if !is_mem_readable {
            protect_mem(
                base_addr,
                0x1000,
                mbi.protect | crate::types::mem_protect::READ_WRITE,
            )?;
        }

        base_addr = base_addr.read() as isize as *const ::core::ffi::c_void;

        if !is_mem_readable {
            protect_mem(base_addr, 0x1000, mbi.protect)?;
        }

        for offset in offsets {
            base_addr = base_addr.offset(*offset);

            mbi = query_mem(base_addr)?;

            is_mem_readable = mbi.state == crate::types::mem_alloc::COMMIT
                && mbi.protect & crate::types::mem_protect::READONLY
                    | crate::types::mem_protect::READ_WRITE
                    | crate::types::mem_protect::EXECUTE_READ
                    | crate::types::mem_protect::EXECUTE_READ_WRITE
                    != 0;

            if !is_mem_readable {
                protect_mem(
                    base_addr,
                    0x1000,
                    mbi.protect | crate::types::mem_protect::READ_WRITE,
                )?;
            }

            base_addr = base_addr.read() as isize as *const ::core::ffi::c_void;

            if !is_mem_readable {
                protect_mem(base_addr, 0x1000, mbi.protect)?;
            }
        }

        Ok(base_addr)
    }
}
