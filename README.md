[![Crates.io Version](https://img.shields.io/crates/v/vcheat?style=for-the-badge)](https://crates.io/crates/vcheat)
[![Static Badge](https://img.shields.io/badge/Github-vcheat-green?style=for-the-badge)](https://github.com/vSylva/vcheat/)

Hacking Library

```rust
// tests/external.rs

#[test]
fn get_pid() {
    unsafe {
        vcheat::external::get_pid("explorer.exe").unwrap();
    }
}

#[test]
fn get_all_proc_info() {
    unsafe {
        vcheat::external::get_all_proc_info().unwrap();
    }
}

#[test]
fn get_mod_info() {
    unsafe {
        let proc_id = vcheat::external::get_pid("explorer.exe").unwrap();

        vcheat::external::get_mod_info(proc_id, "explorer.exe").unwrap();
    }
}

#[test]
fn get_all_mod_info() {
    unsafe {
        let proc_id = vcheat::external::get_pid("explorer.exe").unwrap();

        vcheat::external::get_all_mod_info(proc_id).unwrap();
    }
}

#[test]
fn read_write_mem() {
    unsafe {
        let proc_id = vcheat::external::get_pid("explorer.exe").unwrap();

        let (mod_handle, mod_addr, mod_size) =
            vcheat::external::get_mod_info(proc_id, "explorer.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(proc_id).unwrap();

        vcheat::external::protect_mem(
            proc_handle,
            mod_handle as *const ::core::ffi::c_void,
            mod_size as usize,
            vcheat::page_prot_ty::EXECUTE_READ_WRITE,
        )
        .unwrap();

        let mod_data = vcheat::read_mem(
            proc_handle,
            mod_handle as *const ::core::ffi::c_void,
            mod_size as usize,
        )
        .unwrap();

        let bnw = vcheat::write_mem(proc_handle, mod_addr.cast(), &mod_data).unwrap();

        assert_eq!(mod_size as usize, bnw);

        vcheat::close_handle(proc_handle).unwrap();
    }
}

#[test]
fn read_write_mem_t() {
    unsafe {
        let proc_id = vcheat::external::get_pid("explorer.exe").unwrap();

        let (mod_handle, _mod_addr, mod_size) =
            vcheat::external::get_mod_info(proc_id, "explorer.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(proc_id).unwrap();

        vcheat::external::protect_mem(
            proc_handle,
            mod_handle as *const ::core::ffi::c_void,
            mod_size as usize,
            vcheat::page_prot_ty::EXECUTE_READ_WRITE,
        )
        .unwrap();

        struct Test {
            _reserved0: u8,
            _reserved1: i32,
            _reserved2: [u64; 8],
        }

        let mut buf = ::core::mem::zeroed::<Test>();

        vcheat::read_mem_t(
            proc_handle,
            mod_handle as *const ::core::ffi::c_void,
            &mut buf,
            ::core::mem::size_of::<Test>(),
        )
        .unwrap();

        vcheat::write_mem_t(
            proc_handle,
            mod_handle as *const ::core::ffi::c_void,
            &buf,
            ::core::mem::size_of::<Test>(),
        )
        .unwrap();

        vcheat::close_handle(proc_handle).unwrap();
    }
}

#[test]
fn inject_dll() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        vcheat::external::inject_dll(proc_handle, r"test.dll").unwrap();

        vcheat::close_handle(proc_handle).unwrap();
    }
}

#[test]
fn eject_dll() {
    unsafe {
        let pid = vcheat::external::get_pid("test.exe").unwrap();

        let proc_handle = vcheat::external::open_proc(pid).unwrap();

        let (mod_handle, _, _) = vcheat::external::get_mod_info(pid, "test.dll").unwrap();

        vcheat::external::eject_dll(proc_handle, mod_handle, false).unwrap();

        vcheat::close_handle(proc_handle).unwrap();
    }
}
```
