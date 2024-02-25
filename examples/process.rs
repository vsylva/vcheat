fn main() {
    let proc_a = vcheat::process::get_process_info("explorer.exe").unwrap();
    let proc_b = vcheat::process::get_processes_info()
        .unwrap()
        .iter()
        .find(|x| x.name == "explorer.exe")
        .unwrap()
        .to_owned();
    assert_eq!(proc_a, proc_b);
}
