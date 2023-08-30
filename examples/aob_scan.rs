fn main() {
    let module_path = r"C:\Windows\System32\Windows.UI.Xaml.dll";
    let max_thread_count = vcheat::get_logical_cpu_count();
    let return_on_first = false;

    file_scan_single_threaded(module_path, return_on_first);
    file_scan_multi_threaded(module_path, return_on_first, max_thread_count);

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    {
        let process_name = "Explorer.exe";
        let module_name = "Explorer.exe";
        process_scan_single_threaded(process_name, module_name, return_on_first);
        process_scan_multi_threaded(process_name, module_name, return_on_first, max_thread_count);
    }

    #[cfg(any(
        all(target_arch = "arm", target_pointer_width = "32"),
        target_arch = "x86"
    ))]
    {
        for process_info in vcheat::get_all_processes_info().unwrap() {
            if let Ok(is_wow64) = vcheat::is_wow64_process(process_info.process_id) {
                if is_wow64 {
                    process_scan_single_threaded(
                        &process_info.process_name,
                        &process_info.process_name,
                        return_on_first,
                    );
                    process_scan_multi_threaded(
                        &process_info.process_name,
                        &process_info.process_name,
                        return_on_first,
                        max_thread_count,
                    );

                    break;
                }
            }
        }
    }
}

fn process_scan_single_threaded<S: AsRef<str>>(
    process_name: S,
    module_name: S,
    return_on_first: bool,
) {
    let now = std::time::Instant::now();

    let process_info_array = vcheat::get_all_processes_info().unwrap();

    for process_info in process_info_array {
        if process_info.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            let modules_info =
                vcheat::get_all_process_modules_info(process_info.process_id).unwrap();
            for module_info in modules_info {
                if module_info.module_name.to_lowercase() == module_name.as_ref().to_lowercase() {
                    let module_data = vcheat::read_process_memory(
                        process_info.process_id,
                        module_info.module_address.cast(),
                        module_info.module_size as usize,
                    )
                    .unwrap();
                    let offset_array = vcheat::aob_scan_single_threaded(
                        "5C ? 6D ??",
                        module_data.as_ref(),
                        return_on_first,
                    )
                    .unwrap();
                    println!(
                        "[{}] Offsets found by a single thread: {:X?}",
                        process_name.as_ref(),
                        {
                            if offset_array.len() <= 4 {
                                &offset_array
                            } else {
                                &offset_array[0..=4]
                            }
                        }
                    );
                    println!(
                        "[{}] Elapsed time of a single thread: {} millis",
                        process_name.as_ref(),
                        now.elapsed().as_millis()
                    );
                    break;
                }
            }
        }
    }
}

fn process_scan_multi_threaded<S: AsRef<str>>(
    process_name: S,
    module_name: S,
    return_on_first: bool,
    thread_count: usize,
) {
    let now = std::time::Instant::now();

    let process_info_array = vcheat::get_all_processes_info().unwrap();

    for process_info in process_info_array {
        if process_info.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            let modules_info =
                vcheat::get_all_process_modules_info(process_info.process_id).unwrap();
            for module_info in modules_info {
                if module_info.module_name.to_lowercase() == module_name.as_ref().to_lowercase() {
                    let module_data = vcheat::read_process_memory(
                        process_info.process_id,
                        module_info.module_address.cast(),
                        module_info.module_size as usize,
                    )
                    .unwrap();
                    let offset_array = vcheat::aob_scan_multi_threaded(
                        "5C ? 6D ??",
                        &module_data,
                        return_on_first,
                        thread_count,
                    )
                    .unwrap();

                    println!(
                        "[{}] Offsets found by a multi thread: {:X?}",
                        process_name.as_ref(),
                        if offset_array.len() <= 4 {
                            &offset_array
                        } else {
                            &offset_array[0..=4]
                        }
                    );
                    println!(
                        "[{}] Elapsed time of a multi thread: {} millis",
                        process_name.as_ref(),
                        now.elapsed().as_millis()
                    );
                    break;
                }
            }
        }
    }
}

fn file_scan_single_threaded<P: AsRef<std::path::Path>>(path: P, return_on_first: bool) {
    let file_data = std::fs::read(path.as_ref()).unwrap();

    let pattern = "5C ? 6D ??";

    let now = std::time::Instant::now();

    let offset_array =
        vcheat::aob_scan_single_threaded(pattern, &file_data, return_on_first).unwrap();

    println!(
        "[{}] Elapsed time of a single thread: {} millis",
        path.as_ref().display(),
        now.elapsed().as_millis()
    );

    println!("Length of the found offsets: {}", offset_array.len());
}

fn file_scan_multi_threaded<P: AsRef<std::path::Path>>(
    path: P,
    return_on_first: bool,
    thread_count: usize,
) {
    let file_data = std::fs::read(path.as_ref()).unwrap();

    let pattern = "5C ? 6D ??";

    let now = std::time::Instant::now();

    let multi_array =
        vcheat::aob_scan_multi_threaded(pattern, &file_data, return_on_first, thread_count)
            .unwrap();

    println!(
        "[{}] Elapsed time of a multi thread: {} millis",
        path.as_ref().display(),
        now.elapsed().as_millis()
    );

    println!("Length of the found address: {}", multi_array.len());
}
