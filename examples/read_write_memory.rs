fn main() {
    let read_module_data = true;

    #[cfg(any(
        all(target_arch = "arm", target_pointer_width = "32"),
        target_arch = "x86"
    ))]
    {
        for p in vcheat::get_all_processes_info().unwrap() {
            if let Ok(is_wow64) = vcheat::is_wow64_process(p.process_id) {
                if is_wow64 {
                    read_write_memory(&p.process_name, &p.process_name, read_module_data);
                    break;
                }
            }
        }
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    {
        read_write_memory("explorer.Exe", "ExPlorEr.eXe", read_module_data);
    }
}

fn read_write_memory<S: AsRef<str>>(process_name: S, module_name: S, read_module_data: bool) {
    for p in vcheat::get_all_processes_info().unwrap() {
        if p.process_name.to_lowercase() == process_name.as_ref().to_lowercase() {
            for m in vcheat::get_all_process_modules_info(p.process_id, read_module_data).unwrap() {
                if m.module_name.to_lowercase() == module_name.as_ref().to_lowercase() {
                    let read_data =
                        vcheat::read_process_memory(p.process_id, m.module_address.cast(), 8)
                            .unwrap();
                    println!(
                        "[{}] The data that was read: {:02X?}",
                        process_name.as_ref(),
                        read_data
                    );
                    let num_bytes_writte = vcheat::write_process_memory(
                        p.process_id,
                        m.module_address.cast(),
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
