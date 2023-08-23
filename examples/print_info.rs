fn main() {
    print_dmi_info();

    #[cfg(any(
        all(target_arch = "arm", target_pointer_width = "32"),
        target_arch = "x86"
    ))]
    {
        for p in vcheat::get_all_processes_info().unwrap() {
            if let Ok(is_wow64) = vcheat::is_wow64_process(p.process_id) {
                if is_wow64 {
                    print_process_info(&p.process_name);
                    print_module_info(&p.process_name, &p.process_name, true);
                    enum_data_read_failed_modules(&p.process_name, true);
                    break;
                }
            }
        }
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    {
        print_process_info("Explorer.exe");
        print_module_info("Explorer.exe", "ntdll.DLL", true);
        enum_data_read_failed_modules("Explorer.exe", true);
    }

    println!("If a crash occurs below instead of an unwrap, it indicates an error in the nt_get_all_processes_info function");
    nt_print_process_info();
}

fn print_dmi_info() {
    let dmi_info = vcheat::get_dmi_info().unwrap();
    println!(
        "bios_embedded_controller_firmware_version: {}",
        dmi_info.bios_embedded_controller_firmware_version.unwrap()
    );
    println!("bios_release_date: {}", dmi_info.bios_release_date.unwrap());
    println!("bios_vendor: {}", dmi_info.bios_vendor.unwrap());
    println!("bios_version: {}", dmi_info.bios_version.unwrap());

    println!("system_family: {}", dmi_info.system_family.unwrap());
    let result = dmi_info.system_guid.unwrap();
    println!("system_guid_vec: {:X?}", result.0);
    println!("system_guid_string: {}", result.1);
    let result = dmi_info.system_uuid.unwrap();
    println!("system_uuid_vec: {:X?}", result.0);
    println!("system_uuid_string: {}", result.1);
    println!(
        "system_manufacturer: {}",
        dmi_info.system_manufacturer.unwrap()
    );
    println!("system_product: {}", dmi_info.system_product.unwrap());
    println!(
        "system_serial_number: {}",
        dmi_info.system_serial_number.unwrap()
    );
    println!("system_sku_number: {}", dmi_info.system_sku_number.unwrap());
    println!("system_version: {}", dmi_info.system_version.unwrap());
}

fn print_process_info<S: AsRef<str>>(process_name: S) {
    let process_info_array = vcheat::get_all_processes_info().unwrap();

    for p in process_info_array {
        if p.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            println!("process_name: {}", p.process_name);
            println!("process_id: {}", p.process_id);
            println!(
                "process_base_priority_class: {}",
                p.process_base_priority_class
            );
            println!("process_thread_count: {}", p.process_thread_count);
        }
    }
}

fn print_module_info<S: AsRef<str>>(process_name: S, module_name: S, read_module_data: bool) {
    let process_info_array = vcheat::get_all_processes_info().unwrap();
    for p in process_info_array {
        if p.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            let modules_info =
                vcheat::get_all_process_modules_info(p.process_id, read_module_data).unwrap();
            for m in modules_info {
                if m.module_name.to_lowercase() == module_name.as_ref().to_lowercase() {
                    println!("process id: {}", m.process_id);
                    println!("module name: {}", m.module_name);
                    println!("module handle: {:X?}", m.module_handle as usize);
                    println!("module address: {:X?}", m.module_address as usize);
                    println!("module path: {}", m.module_path);
                    if read_module_data {
                        println!(
                            "module size(module_data.len()): {:X}",
                            m.module_data.unwrap().unwrap().len()
                        );
                    } else {
                        println!("module size: {:X?}", m.module_size);
                    }
                }
            }
        }
    }
}

fn enum_data_read_failed_modules<S: AsRef<str>>(process_name: S, read_module_data: bool) {
    let process_info_array = vcheat::get_all_processes_info().unwrap();
    for p in process_info_array {
        if p.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            let modules_info =
                vcheat::get_all_process_modules_info(p.process_id, read_module_data).unwrap();
            for m in modules_info {
                if let Some(Err(err)) = m.module_data {
                    println!("data read invalid process id: {}", m.process_id);
                    println!("data read invalid module name: {}", m.module_name);
                    println!("data read error message: {}", err);
                }
            }
        }
    }
}

fn nt_print_process_info() {
    let process_info_array = vcheat::nt_get_all_processes_info().unwrap();

    for p in process_info_array {
        if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
            println!("nt_process_name: {}", p.process_name);
            println!("nt_process_id: {}", p.process_id);
            println!("nt_process_session_id: {}", p.process_session_id);
            println!(
                "nt_process_base_priority_class: {}",
                p.process_base_priority_class
            );
            println!("nt_process_handle_count: {}", p.process_handle_count);
            println!("nt_process_pagefile_usage: {:X}", p.process_pagefile_usage);
            println!("nt_process_thread_count: {}", p.process_thread_count);
        }
    }
}
