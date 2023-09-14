use vcheat::{memory, process, types::*};

fn main() {
    let process_handle = process::open_process(std::process::id()).unwrap();

    let size = 1024;

    '_rust_alloc: {
        let standard_allocated_address = memory::standard_alloc(size).unwrap();

        let standard_query_info =
            memory::virtual_query(process_handle, standard_allocated_address.cast()).unwrap();

        assert_eq!(standard_query_info.page_protect, PageProtect::ReadWrite);

        memory::standard_free(standard_allocated_address, size).unwrap();
    }

    '_win_alloc: {
        let virtual_allocated_address = memory::virtual_alloc(
            core::ptr::null_mut(),
            size,
            MemAllocation::Reserve | MemAllocation::Commit,
            PageProtect::ExecuteRead.into(),
        )
        .unwrap();

        let query_info =
            memory::virtual_query(process_handle, virtual_allocated_address.cast()).unwrap();

        assert_eq!(query_info.page_protect, PageProtect::ExecuteRead);

        memory::virtual_free(virtual_allocated_address, 0, MemFree::Release.into()).unwrap();
    }

    process::close_handle(process_handle).unwrap();
}
