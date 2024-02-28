#[test]
fn get_mod_info() {
    unsafe {
        let mod_handle = vcheat::internal::get_mod_handle("").unwrap();
        vcheat::internal::get_mod_info(mod_handle).unwrap();
    }
}

#[test]
fn read_write_mem() {
    unsafe {
        let mod_handle = vcheat::internal::get_mod_handle("ntdll.dll").unwrap();

        let proc_handle = vcheat::internal::get_proc_handle();

        let (mod_addr, mod_size) = vcheat::internal::get_mod_info(mod_handle).unwrap();

        let data = vcheat::read_mem(proc_handle, mod_addr, mod_size as usize).unwrap();

        vcheat::internal::protect_mem(
            mod_addr,
            mod_size as usize,
            vcheat::page_prot_ty::EXECUTE_READ_WRITE,
        )
        .unwrap();

        vcheat::write_mem(proc_handle, mod_addr, &data).unwrap();
    }
}

#[test]
fn alloc_query_mem() {
    unsafe {
        let buffer = vcheat::internal::alloc_mem(
            ::core::ptr::null_mut(),
            0x100,
            vcheat::mem_alloc_ty::RESERVE | vcheat::mem_alloc_ty::COMMIT,
            vcheat::page_prot_ty::READ_WRITE,
        )
        .unwrap();

        #[allow(unused_variables)]
        let (base_address, region_size, allocation_protectct, type_, state, protect) =
            vcheat::internal::query_mem(buffer).unwrap();

        vcheat::internal::free_mem(buffer, 0, vcheat::mem_free_ty::RELEASE).unwrap();
    }
}
