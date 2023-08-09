fn main() {
    scan();
}

fn scan() {
    let process_info_array = vcheat::get_all_processes_info().unwrap();

    for p in process_info_array {
        if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
            let modules_info = vcheat::get_all_process_modules_info(p.process_id, true).unwrap();
            for m in modules_info {
                if m.module_name.to_lowercase() == "NTDLL.DLL".to_lowercase() {
                    let addres_array = vcheat::aob_scan_single_threaded(
                        "5C ? 6D ??",
                        &m.module_data.unwrap(),
                        false,
                    )
                    .unwrap();
                    println!("{:X?}", addres_array);
                    break;
                }
            }
        }
    }
}

// fn nt() {
//     let process_info_array = vcheat::nt_get_all_processes_info().unwrap();

//     for p in process_info_array {
//         if p.process_name.to_lowercase() == "Explorer.EXE".to_lowercase() {
//             let modules_info = vcheat::get_all_process_modules_info(p.process_id, true).unwrap();
//             for m in modules_info {
//                 if m.module_name.to_lowercase() == "NTDLL.DLL".to_lowercase() {
//                     let addres_array = vcheat::aob_scan_single_threaded(
//                         "5C ? 6D ??",
//                         &m.module_data.unwrap(),
//                         true,
//                     )
//                     .unwrap();
//                     println!("{:X?}", addres_array);
//                     break;
//                 }
//             }
//         }
//     }
// }
