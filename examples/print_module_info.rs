fn main() {
    let process_info_array = vcheat::get_all_processes_info().unwrap();
    let read_module_data = true;
    for p in process_info_array {
        if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
            let modules_info =
                vcheat::get_all_process_modules_info(p.process_id, read_module_data).unwrap();
            for m in modules_info {
                if m.module_name.to_lowercase() == "SHELL32.dll".to_lowercase() {
                    println!("process id: {}", m.process_id);
                    println!("module name: {}", m.module_name);
                    println!("module handle: {:?}", m.module_handle);
                    println!("module address: {:?}", m.module_address);
                    println!("module path: {}", m.module_path);
                    if read_module_data {
                        println!(
                            "module size(module_data.len()): {:X}",
                            m.module_data.unwrap().len()
                        );
                    } else {
                        println!("module size: {:X?}", m.module_size);
                    }
                }
            }
        }
    }
}
