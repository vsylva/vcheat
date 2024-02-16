use vcheat::{memory, module, process};

fn main() {
    let process_name = "ExplorEr.Exe";

    let module_name = "ExplorEr.Exe";

    let size = 16;

    let process_info = process::get_processes_info()
        .unwrap()
        .into_iter()
        .find(|process_info| {
            process_info.to_owned().name.to_lowercase() == process_name.to_lowercase()
        })
        .unwrap();

    let module_info = module::get_modules_info(process_info.id)
        .unwrap()
        .into_iter()
        .find(|module_info| {
            module_info.to_owned().name.to_lowercase() == module_name.to_lowercase()
        })
        .unwrap();

    let process_handle = process::open_process(process_info.id).unwrap();

    let module_data =
        memory::read_process_memory(process_handle, module_info.base_address.cast(), size).unwrap();

    let num_bytes_written = memory::write_process_memory(
        process_handle,
        module_info.base_address.cast(),
        &module_data,
    )
    .unwrap();

    assert_eq!(num_bytes_written, size);

    let module_data_usize: &[usize] = unsafe {
        ::core::slice::from_raw_parts(
            module_data.as_ptr().cast(),
            module_data.len() / ::std::mem::size_of::<usize>(),
        )
    };

    let num_bytes_written = memory::write_process_memory(
        process_handle,
        module_info.base_address.cast(),
        &module_data_usize,
    )
    .unwrap();

    assert_eq!(num_bytes_written, size);

    let module_data_ =
        memory::read_process_memory(process_handle, module_info.base_address.cast(), size).unwrap();

    assert_eq!(module_data, module_data_);

    vcheat::process::close_handle(process_handle).unwrap();
}
