use vcheat::{module, process};

fn main() {
    let dll_name = r"test.dll";

    let process_name = "test.exe";

    let process_info = process::get_processes_info()
        .unwrap()
        .into_iter()
        .find(|process_info| {
            process_info.to_owned().name.to_lowercase() == process_name.to_lowercase()
        })
        .unwrap();

    let process_handle = process::open_process(process_info.id).unwrap();

    module::inject_dll(process_handle, dll_name).unwrap();

    ::std::thread::sleep(::std::time::Duration::from_secs(5));

    let module_info = module::get_modules_info(process_info.id)
        .unwrap()
        .into_iter()
        .find(|module_info| module_info.to_owned().name.to_lowercase() == dll_name.to_lowercase())
        .unwrap();

    module::eject_dll(process_handle, module_info.handle).unwrap();

    process::close_handle(process_handle).unwrap();
}
