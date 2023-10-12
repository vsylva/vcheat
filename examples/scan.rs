use vcheat::{memory, module, process, system};

fn main() {
    let module_path = r"C:\Windows\System32\Windows.UI.Xaml.dll";

    let max_thread_count = system::get_system_info().number_of_processors;

    let return_on_first = false;

    '_scan_file: {
        let mut offset_array1 = file_scan_single_threaded(module_path, return_on_first);

        let mut offset_array2 =
            file_scan_multi_threaded(module_path, return_on_first, max_thread_count);

        offset_array1.sort();

        offset_array2.sort();

        assert_eq!(offset_array1, offset_array2);
    }

    '_scan_module: {
        let process_name = "Explorer.exe";

        let module_name = "Explorer.exe";

        let mut offset_array3 =
            process_scan_single_threaded(process_name, module_name, return_on_first);

        let mut offset_array4 = process_scan_multi_threaded(
            process_name,
            module_name,
            return_on_first,
            max_thread_count,
        );

        offset_array3.sort();

        offset_array4.sort();

        assert_eq!(offset_array3, offset_array4);
    }
}

fn process_scan_single_threaded(
    process_name: &'static str,
    module_name: &'static str,
    return_on_first: bool,
) -> Vec<usize> {
    let process_info = process::get_processes_info()
        .unwrap()
        .into_iter()
        .find(|process_info| {
            process_info.to_owned().name.to_lowercase() == process_name.to_lowercase()
        })
        .unwrap();

    let process_info1 = process::get_process_info(process_name).unwrap();

    assert_eq!(process_info, process_info1);

    let module_info = module::get_modules_info(process_info.id)
        .unwrap()
        .into_iter()
        .find(|module_info| {
            module_info.to_owned().name.to_lowercase() == module_name.to_lowercase()
        })
        .unwrap();

    let module_info1 = module::get_module_info(process_info1.id, module_name).unwrap();

    assert_eq!(module_info, module_info1);

    let process_handle = process::open_process(process_info.id).unwrap();

    let module_data = memory::read_process_memory(
        process_handle,
        module_info.base_address.cast(),
        module_info.size as usize,
    )
    .unwrap();

    process::close_handle(process_handle).unwrap();

    memory::aob_scan_single_threaded("5C ? 6D ??", &module_data, return_on_first).unwrap()
}

fn process_scan_multi_threaded(
    process_name: &'static str,
    module_name: &'static str,
    return_on_first: bool,
    thread_count: u32,
) -> Vec<usize> {
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

    let module_data = memory::read_process_memory(
        process_handle,
        module_info.base_address.cast(),
        module_info.size as usize,
    )
    .unwrap();

    process::close_handle(process_handle).unwrap();

    memory::aob_scan_multi_threaded("5C ? 6D ??", &module_data, return_on_first, thread_count)
        .unwrap()
}

fn file_scan_single_threaded(path: &str, return_on_first: bool) -> Vec<usize> {
    let file_data = ::std::fs::read(::std::path::Path::new(path)).unwrap();

    let pattern = "5C ? 6D ??";

    let offset_array =
        memory::aob_scan_single_threaded(pattern, &file_data, return_on_first).unwrap();

    offset_array
}

fn file_scan_multi_threaded(path: &str, return_on_first: bool, thread_count: u32) -> Vec<usize> {
    let file_data = ::std::fs::read(::std::path::Path::new(path)).unwrap();

    let pattern = "5C ? 6D ??";

    let offset_array =
        memory::aob_scan_multi_threaded(pattern, &file_data, return_on_first, thread_count)
            .unwrap();

    offset_array
}
