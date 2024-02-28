use crate::{HANDLE, HMODULE};

#[doc = "Return value: `Handle`"]
pub unsafe fn get_proc_handle() -> HANDLE {
    crate::ffi::GetCurrentProcess()
}

#[doc = r#"Return value: `Handle`

If the parameter is **empty**, the function returns information about the file used to create the calling process (.exe file)"#]
pub unsafe fn get_mod_handle<S: AsRef<str>>(name: S) -> Result<HMODULE, ::std::io::Error> {
    let ptr;

    let buf;

    if name.as_ref().is_empty() {
        ptr = ::core::ptr::null();
    } else {
        buf = crate::common::rs_to_cwsb(name.as_ref());
        ptr = buf.as_ptr();
    }

    let mod_handle = crate::ffi::GetModuleHandleW(ptr);

    if 0 == mod_handle {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(mod_handle)
}

#[doc = r"Return value: `(Base Address, Size)`"]
pub unsafe fn get_mod_info(
    mod_handle: HMODULE,
) -> Result<(*mut ::core::ffi::c_void, u32), ::std::io::Error> {
    let mut mod_info = ::core::mem::zeroed::<crate::ffi::MODULEINFO>();

    if 0 == crate::ffi::GetModuleInformation(
        crate::ffi::GetCurrentProcess(),
        mod_handle,
        &mut mod_info,
        ::core::mem::size_of::<crate::ffi::MODULEINFO>() as u32,
    ) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok((mod_info.lp_base_of_dll, mod_info.size_of_image))
}

#[doc = "Return value: `Allocated memory address`"]
pub unsafe fn alloc_mem(
    addr: *const ::core::ffi::c_void,
    size: usize,
    mem_alloc_ty: u32,
    page_prot_ty: u32,
) -> Result<*mut ::core::ffi::c_void, ::std::io::Error> {
    let addr = crate::ffi::VirtualAlloc(addr, size, mem_alloc_ty, page_prot_ty);

    if ::core::ptr::null_mut() == addr {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(addr)
}

#[doc = "If the third parameter is RELEASE, the second parameter must be 0"]
pub unsafe fn free_mem(
    addr: *mut ::core::ffi::c_void,
    mut size: usize,
    mem_free_ty: u32,
) -> Result<(), ::std::io::Error> {
    if crate::mem_free_ty::RELEASE == mem_free_ty {
        size = 0;
    }

    if 0 == crate::ffi::VirtualFree(addr, size, mem_free_ty) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(())
}

#[doc = "Return value: `(BaseAddress, RegionSize, AllocationProtect, Type, State, Protect)`"]
pub unsafe fn query_mem(
    addr: *mut ::core::ffi::c_void,
) -> Result<(*mut ::core::ffi::c_void, usize, u32, u32, u32, u32), ::std::io::Error> {
    let mut mbi: crate::ffi::MemoryBasicInformation =
        ::core::mem::zeroed::<crate::ffi::MemoryBasicInformation>();

    if 0 == crate::ffi::VirtualQuery(
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
    addr: *const ::core::ffi::c_void,
    size: usize,
    page_prot_ty: u32,
) -> Result<u32, ::std::io::Error> {
    let mut prev_prot: u32 = Default::default();

    if 0 == crate::ffi::VirtualProtect(addr, size, page_prot_ty, &mut prev_prot) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(prev_prot)
}

#[doc = "Return value: `Module handle`"]
pub unsafe fn load_dll<S: AsRef<str>>(name: S) -> Result<HMODULE, ::std::io::Error> {
    let buf = crate::common::rs_to_cwsb(name.as_ref());

    let handle = crate::ffi::LoadLibraryW(buf.as_ptr());

    if 0 == handle {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(handle)
}

pub unsafe fn free_dll(mod_handle: HMODULE) -> Result<(), ::std::io::Error> {
    if 0 == crate::ffi::FreeLibrary(mod_handle) {
        return Err(::std::io::Error::last_os_error());
    }

    Ok(())
}

pub unsafe fn free_dll_exit_thread(mod_handle: HMODULE, exit_code: u32) {
    crate::ffi::FreeLibraryAndExitThread(mod_handle, exit_code);
}
