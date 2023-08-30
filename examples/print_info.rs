fn main() {
    print_dmi_info();

    #[cfg(any(
        all(target_arch = "arm", target_pointer_width = "32"),
        target_arch = "x86"
    ))]
    {
        for process_info in vcheat::get_all_processes_info().unwrap() {
            if let Ok(is_wow64) = vcheat::is_wow64_process(process_info.process_id) {
                if is_wow64 {
                    print_process_info(&process_info.process_name);
                    print_module_info(&process_info.process_name, &process_info.process_name);
                    break;
                }
            }
        }
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    {
        print_process_info("Explorer.exe");
        print_module_info("Explorer.exe", "ntdll.DLL");
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
    for process_info in vcheat::get_all_processes_info().unwrap() {
        if process_info.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            println!("process_name: {}", process_info.process_name);
            println!("process_id: {}", process_info.process_id);
            println!(
                "process_base_priority_class: {}",
                process_info.process_base_priority_class
            );
            println!(
                "process_thread_count: {}",
                process_info.process_thread_count
            );
        }
    }
}

fn print_module_info<S: AsRef<str>>(process_name: S, module_name: S) {
    for process_info in vcheat::get_all_processes_info().unwrap() {
        if process_info.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            for module_info in
                vcheat::get_all_process_modules_info(process_info.process_id).unwrap()
            {
                if module_info.module_name.to_lowercase() == module_name.as_ref().to_lowercase() {
                    println!("process id: {}", module_info.process_id);
                    println!("module name: {}", module_info.module_name);
                    println!("module handle: {:X?}", module_info.module_handle as usize);
                    println!("module address: {:X?}", module_info.module_address as usize);
                    println!("module path: {}", module_info.module_path);
                }
            }
        }
    }
}

fn nt_print_process_info() {
    for system_process_info in vcheat::nt_get_all_processes_info().unwrap() {
        if system_process_info.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
            println!("nt_process_name: {}", system_process_info.process_name);
            println!("nt_process_id: {}", system_process_info.process_id);
            println!(
                "nt_process_session_id: {}",
                system_process_info.process_session_id
            );
            println!(
                "nt_process_base_priority_class: {}",
                system_process_info.process_base_priority_class
            );
            println!(
                "nt_process_handle_count: {}",
                system_process_info.process_handle_count
            );
            println!(
                "nt_process_pagefile_usage: {:X}",
                system_process_info.process_pagefile_usage
            );
            println!(
                "nt_process_thread_count: {}",
                system_process_info.process_thread_count
            );
        }
    }
}
