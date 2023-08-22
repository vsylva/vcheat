fn main() {
    nt_scan_single_threaded();
    scan_single_threaded();
    scan_multi_threaded();
}

fn nt_scan_single_threaded() {
    let now = std::time::Instant::now();

    let process_info_array = vcheat::nt_get_all_processes_info().unwrap();
    let return_on_first = false;

    for p in process_info_array {
        if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
            let modules_info = vcheat::get_all_process_modules_info(p.process_id, true).unwrap();
            for m in modules_info {
                if m.module_name.to_lowercase() == "NTDLL.DLL".to_lowercase() {
                    let addres_array = vcheat::aob_scan_single_threaded(
                        "5C ? 6D ??",
                        m.module_data.as_deref().unwrap(),
                        return_on_first,
                    )
                    .unwrap();
                    println!("Address found by a single thread: {:X?}", addres_array);
                    println!(
                        "Elapsed time of a single thread: {} millis",
                        now.elapsed().as_millis()
                    );
                    break;
                }
            }
        }
    }
}

fn scan_single_threaded() {
    let now = std::time::Instant::now();

    let process_info_array = vcheat::get_all_processes_info().unwrap();
    let return_on_first = false;

    for p in process_info_array {
        if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
            let modules_info = vcheat::get_all_process_modules_info(p.process_id, true).unwrap();
            for m in modules_info {
                if m.module_name.to_lowercase() == "NTDLL.DLL".to_lowercase() {
                    let addres_array = vcheat::aob_scan_single_threaded(
                        "5C ? 6D ??",
                        m.module_data.as_deref().unwrap(),
                        return_on_first,
                    )
                    .unwrap();
                    println!("Address found by a single thread: {:X?}", addres_array);
                    println!(
                        "Elapsed time of a single thread: {} millis",
                        now.elapsed().as_millis()
                    );
                    break;
                }
            }
        }
    }
}

fn scan_multi_threaded() {
    let now = std::time::Instant::now();

    let process_info_array = vcheat::get_all_processes_info().unwrap();

    for p in process_info_array {
        if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
            let modules_info = vcheat::get_all_process_modules_info(p.process_id, true).unwrap();
            for m in modules_info {
                if m.module_name.to_lowercase() == "NTDLL.DLL".to_lowercase() {
                    let addres_array = vcheat::aob_scan_multi_threaded(
                        "5C ? 6D ??",
                        m.module_data.as_deref().unwrap(),
                        false,
                        vcheat::get_logical_cpu_count(),
                    )
                    .unwrap();
                    println!("Address found by a multi thread: {:X?}", addres_array);
                    println!(
                        "Elapsed time of a multi thread: {} millis",
                        now.elapsed().as_millis()
                    );
                    break;
                }
            }
        }
    }
}
