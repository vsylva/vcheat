[![Crates.io Version](https://img.shields.io/crates/v/vcheat?style=for-the-badge)](https://crates.io/crates/vcheat)
[![Static Badge](https://img.shields.io/badge/Github-vcheat-green?style=for-the-badge)](https://github.com/vSylva/vcheat/)

Hacking Library

```rust
// tests/external.rs

#[test]
fn get_all_proc_info() {
    unsafe {
        for pi in vcheat::external::get_all_proc_info().unwrap() {
            pi.id;
            pi.name;
        }
    }
}

#[test]
fn get_mod_info() {
    unsafe {
        let pid = vcheat::external::get_pid("explorer.exe").unwrap();

        let mi = vcheat::external::get_mod_info(pid, "explorer.exe").unwrap();

        mi.name;
        mi.handle;
        mi.addr;
        mi.size;

        let mis = vcheat::external::get_all_mod_info(pid).unwrap();

        for mi in mis {
            mi.name;
            mi.handle;
            mi.addr;
            mi.size;
        }
    }
}

#[test]
fn read_write_mem() {
    unsafe {
        let pid = vcheat::external::get_pid("explorer.exe").unwrap();

        let mi = vcheat::external::get_mod_info(pid, "explorer.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        let prev_protect = vcheat::external::protect_mem(
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

        vcheat::external::protect_mem(proc_handle, mi.addr, mi.size as usize, prev_protect)
            .unwrap();

        assert_eq!(bytes_num_written, bytes_num_written1);
        assert_eq!(mod_data, mod_data1);

        vcheat::close_handle(proc_handle).unwrap();
    }
}

#[allow(unused)]
// #[test]
fn inject_dll() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        vcheat::external::inject_dll(proc_handle, "test.dll").unwrap();

        vcheat::close_handle(proc_handle).unwrap();
    }
}

#[allow(unused)]
// #[test]
fn eject_dll() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        let mi = vcheat::external::get_mod_info(pid, "test.dll").unwrap();

        vcheat::external::eject_dll(proc_handle, mi.handle, false).unwrap();

        vcheat::close_handle(proc_handle).unwrap();
    }
}

#[test]
fn alloc_free_mem() {
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

        vcheat::close_handle(proc_handle).unwrap();

        assert_eq!(num, 1234);
    }
}

#[allow(unused)]
// #[test]
fn read_multi_pointer() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();
        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        // A: *base_addr
        // B: (*A).add(0xAB)
        // C: (*B).add(0xCD)
        // ......
        let final_ptr = vcheat::external::read_multi_pointer(
            proc_handle,
            0x123456 as *const ::core::ffi::c_void,
            &[0xAB, 0xCD, 0x10, 0x20],
        )
        .unwrap();
    }
}
```
