fn main() {
    #[cfg(target_arch = "x86")]
    {
        for process_info in vcheat::get_all_processes_info().unwrap() {
            if let Ok(is_wow64) = vcheat::is_wow64_process(process_info.process_id) {
                if is_wow64 {
                    read_write_memory(&process_info.process_name, &process_info.process_name);
                    break;
                }
            }
        }
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    {
        read_write_memory("explorer.Exe", "ExPlorEr.eXe");
    }
}

fn read_write_memory<S: AsRef<str>>(process_name: S, module_name: S) {
    for process_info in vcheat::get_all_processes_info().unwrap() {
        if process_info.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            for module_info in
                vcheat::get_all_process_modules_info(process_info.process_id).unwrap()
            {
                if module_info.module_name.to_lowercase() == module_name.as_ref().to_lowercase() {
                    let read_data = vcheat::read_process_memory(
                        process_info.process_id,
                        module_info.module_base_address.cast(),
                        8,
                    )
                    .unwrap();
                    println!(
                        "[{}] The data that was read: {:02X?}",
                        process_name.as_ref(),
                        read_data
                    );
                    let num_bytes_writte = vcheat::write_process_memory(
                        process_info.process_id,
                        module_info.module_base_address.cast(),
                        &read_data,
                    )
                    .unwrap();
                    println!(
                        "[{}] Number of bytes written: {}",
                        process_name.as_ref(),
                        num_bytes_writte
                    );
                }
            }
        }
    }
}
