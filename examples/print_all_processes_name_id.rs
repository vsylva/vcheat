fn main() {
    print_process_info();
    nt_print_process_info();
}

fn print_process_info() {
    let process_info_array = vcheat::get_all_processes_info().unwrap();

    for p in process_info_array {
        println!(
            "process_name: {}\tprocess_id: {}",
            p.process_name, p.process_id
        );
    }
}

fn nt_print_process_info() {
    let process_info_array = vcheat::nt_get_all_processes_info().unwrap();

    for p in process_info_array {
        println!(
            "nt_process_name: {}\tnt_process_id: {}",
            p.process_name, p.process_id
        );
    }
}
