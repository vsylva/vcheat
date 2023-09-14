use vcheat::{module, process};

fn main() {
    info("Explorer.exe");

    println!("If a crash occurs below instead of an unwrap, it indicates an error in the nt_get_all_processes_info function");

    nt_info("Explorer.exe");
}

fn info(process_name: &str) {
    for process_info in process::get_processes_info().unwrap() {
        if process_info.name.to_lowercase() == process_name.to_lowercase() {
            for module_info in module::get_modules_info(process_info.id).unwrap() {
                if module_info.name.to_lowercase() == process_name.to_lowercase() {
                    println!("process name: {}", process_info.name);
                    println!("process id: {}", process_info.id);
                    println!("process id from module info: {}", module_info.process_id);
                    println!("module name: {}", module_info.name);
                    println!("module handle: {:p}", module_info.handle);
                    println!("module address: {:p}", module_info.base_address);
                    println!("module path: {}", module_info.path);
                }
            }
        }
    }
}

fn nt_info(process_name: &str) {
    for system_process_info in process::nt_get_processes_info().unwrap() {
        if system_process_info.name.to_lowercase() == process_name.to_lowercase() {
            println!("nt proces name: {}", system_process_info.name);
            println!("nt proces id: {}", system_process_info.id);
            println!("nt proces session id: {}", system_process_info.session_id);
            println!(
                "nt proces pagefile usage: {:X}",
                system_process_info.pagefile_usage
            );
        }
    }
}
