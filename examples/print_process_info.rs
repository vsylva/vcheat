fn main() {
    print_process_info();
    nt_print_process_info();
}

fn print_process_info() {
    let process_info_array = vcheat::get_all_processes_info().unwrap();

    for p in process_info_array {
        if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
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
