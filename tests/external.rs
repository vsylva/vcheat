#[test]
fn get_all_proc_info() {
    unsafe {
        for pi in vcheat::external::get_all_proc_info().unwrap() {
            println!("{:#?}", pi);
        }
    }
}

#[test]
fn get_mod_info() {
    unsafe {
        let pid = vcheat::external::get_pid("explorer.exe").unwrap();

        let mi = vcheat::external::get_mod_info(pid, "explorer.exe").unwrap();

        println!("{:#?}", mi);

        let mis = vcheat::external::get_all_mod_info(pid).unwrap();

        for mi in mis {
            println!("{:#?}", mi);
        }
    }
}

#[test]
fn read_write_mem() {
    unsafe {
        let pid = vcheat::external::get_pid("explorer.exe").unwrap();

        let mi = vcheat::external::get_mod_info(pid, "explorer.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        vcheat::external::protect_mem(
            proc_handle,
            mi.addr,
            mi.size as usize,
            vcheat::types::mem_protect::EXECUTE_READ_WRITE,
        )
        .unwrap();

        let mod_data = vcheat::read_mem(proc_handle, mi.addr, mi.size as usize).unwrap();

        let bytes_num_written = vcheat::write_mem(proc_handle, mi.addr, &mod_data).unwrap();

        let mut mod_data1 = vec![0u8; mi.size as usize];

        let bytes_num_written1 = vcheat::read_mem_t(
            proc_handle,
            mi.addr,
            mod_data1.as_mut_ptr(),
            mi.size as usize,
        )
        .unwrap();

        assert_eq!(bytes_num_written, bytes_num_written1);
        assert_eq!(mod_data, mod_data1);

        vcheat::external::close_handle(proc_handle).unwrap();
    }
}

#[test]
fn _alloc_free_mem() {
    unsafe {
        let pid = vcheat::external::get_pid("explorer.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        let alloc = vcheat::external::alloc_mem(
            proc_handle,
            ::core::ptr::null(),
            0x1000,
            vcheat::types::mem_alloc::COMMIT,
            vcheat::types::mem_protect::READ_WRITE,
        )
        .unwrap();

        vcheat::write_mem(proc_handle, alloc, &[0xD2_u8, 0x04]).unwrap();

        let buf = vcheat::read_mem(proc_handle, alloc, 4).unwrap();

        vcheat::external::free_mem(proc_handle, alloc, 0, vcheat::types::mem_free::RELEASE)
            .unwrap();

        let bytes = [buf[0], buf[1], buf[2], buf[3]];

        let num = i32::from_le_bytes(bytes);

        vcheat::external::close_handle(proc_handle).unwrap();

        assert_eq!(num, 1234);
    }
}

fn _inject_dll() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        vcheat::external::inject_dll(proc_handle, "test.dll").unwrap();

        vcheat::external::close_handle(proc_handle).unwrap();
    }
}

fn _eject_dll() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        let mi = vcheat::external::get_mod_info(pid, "test.dll").unwrap();

        vcheat::external::eject_dll(proc_handle, mi.handle, false).unwrap();

        vcheat::external::close_handle(proc_handle).unwrap();
    }
}

fn _read_multi_pointer() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();
        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        let _final_ptr = vcheat::external::read_multi_pointer(
            proc_handle,
            0x123456 as *const ::core::ffi::c_void,
            &[0xAB, 0xCD, 0x10, 0x20],
        )
        .unwrap();
    }
}
