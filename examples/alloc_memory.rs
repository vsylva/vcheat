fn main() {
    let size = 1024;

    let standard_allocated_address = vcheat::standard_alloc(size).unwrap();

    let standard_query_info =
        vcheat::virtual_query(std::process::id(), standard_allocated_address.cast()).unwrap();

    println!(
        "standard_alloc allocated_address: {:p}\tread/write",
        standard_allocated_address
    );

    println!("query info: {:X}", standard_query_info.memory_page_protect);

    assert_eq!(
        standard_query_info.memory_page_protect,
        vcheat::PageProtectType::ReadWrite
    );

    let virtual_allocated_address = vcheat::virtual_alloc(
        core::ptr::null_mut(),
        size,
        vcheat::MemAllocationType::Reserve | vcheat::MemAllocationType::Commit,
        vcheat::PageProtectType::ExecuteRead,
    )
    .unwrap();

    let query_info =
        vcheat::virtual_query(std::process::id(), virtual_allocated_address.cast()).unwrap();

    println!(
        "virtual_alloc allocated_address: {:p}\texecute/read",
        virtual_allocated_address
    );

    println!("query info: {:X}", query_info.memory_page_protect);

    assert_eq!(
        query_info.memory_page_protect,
        vcheat::PageProtectType::ExecuteRead
    );

    vcheat::standard_free(standard_allocated_address, size).unwrap();

    vcheat::virtual_free(virtual_allocated_address, 0, vcheat::MemFreeType::Release).unwrap();
}
