fn main() {
    let read_module_data = true;
    for p in vcheat::get_all_processes_info().unwrap() {
        if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
            for m in vcheat::get_all_process_modules_info(p.process_id, read_module_data).unwrap() {
                if m.module_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
                    let process_handle = vcheat::get_process_handle(p.process_id).unwrap();
                    let read_data =
                        vcheat::read_memory(process_handle, m.module_address.cast(), 16).unwrap();
                    println!("The data that was read: {:02X?}", read_data);
                    let num_bytes_writte =
                        vcheat::write_memory(process_handle, m.module_address.cast(), &read_data)
                            .unwrap();
                    println!("Number of bytes written: {}", num_bytes_writte);
                }
            }
        }
    }
}
