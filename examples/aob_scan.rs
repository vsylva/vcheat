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
        for p in vcheat::get_all_processes_info().unwrap() {
            if let Ok(is_wow64) = vcheat::is_wow64_process(p.process_id) {
                if is_wow64 {
                    process_scan_single_threaded(&p.process_name, &p.process_name, return_on_first);
                    process_scan_multi_threaded(
                        &p.process_name,
                        &p.process_name,
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

    for p in process_info_array {
        if p.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            let modules_info = vcheat::get_all_process_modules_info(p.process_id, true).unwrap();
            for m in modules_info {
                if m.module_name.to_lowercase() == module_name.as_ref().to_lowercase() {
                    let addres_array = vcheat::aob_scan_single_threaded(
                        "5C ? 6D ??",
                        m.module_data.unwrap().as_deref().unwrap(),
                        return_on_first,
                    )
                    .unwrap();
                    println!(
                        "[{}] Address found by a single thread: {:X?}",
                        process_name.as_ref(),
                        {
                            if addres_array.len() <= 4 {
                                &addres_array
                            } else {
                                &addres_array[0..=4]
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

    for p in process_info_array {
        if p.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            let modules_info = vcheat::get_all_process_modules_info(p.process_id, true).unwrap();
            for m in modules_info {
                if m.module_name.to_lowercase() == module_name.as_ref().to_lowercase() {
                    let addres_array = vcheat::aob_scan_multi_threaded(
                        "5C ? 6D ??",
                        m.module_data.unwrap().as_deref().unwrap(),
                        return_on_first,
                        thread_count,
                    )
                    .unwrap();

                    println!(
                        "[{}] Address found by a multi thread: {:X?}",
                        process_name.as_ref(),
                        if addres_array.len() <= 4 {
                            &addres_array
                        } else {
                            &addres_array[0..=4]
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
    let data = std::fs::read(path.as_ref()).unwrap();

    let pattern = "5C ? 6D ??";

    let now = std::time::Instant::now();

    let single_array = vcheat::aob_scan_single_threaded(pattern, &data, return_on_first).unwrap();

    println!(
        "[{}] Elapsed time of a single thread: {} millis",
        path.as_ref().display(),
        now.elapsed().as_millis()
    );

    println!("Length of the found address: {}", single_array.len());
}

fn file_scan_multi_threaded<P: AsRef<std::path::Path>>(
    path: P,
    return_on_first: bool,
    thread_count: usize,
) {
    let data = std::fs::read(path.as_ref()).unwrap();

    let pattern = "5C ? 6D ??";

    let now = std::time::Instant::now();

    let multi_array =
        vcheat::aob_scan_multi_threaded(pattern, &data, return_on_first, thread_count).unwrap();

    println!(
        "[{}] Elapsed time of a multi thread: {} millis",
        path.as_ref().display(),
        now.elapsed().as_millis()
    );

    println!("Length of the found address: {}", multi_array.len());
}
