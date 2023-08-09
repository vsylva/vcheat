use crate::*;

pub fn get_all_processes_info() -> Result<Vec<VProcessInfo>> {
    let mut process_info_array = Vec::<VProcessInfo>::new();
    unsafe {
        for p in process::get_all_process_info()? {
            process_info_array.push(VProcessInfo {
                process_id: p.th32_process_id,
                process_thread_count: p.cnt_threads,
                process_parent_process_id: p.th32_parent_process_id,
                process_base_priority_class: p.pc_pri_class_base,
                process_name: core::ffi::CStr::from_ptr(
                    String::from_utf16_lossy(p.sz_exe_file.as_ref())
                        .as_ptr()
                        .cast(),
                )
                .to_string_lossy()
                .to_string(),
            });
        }
    }
    Ok(process_info_array)
}

// pub fn nt_get_all_processes_info() -> Result<Vec<VNTProcessInfo>> {
//     let mut process_info_array = Vec::<VNTProcessInfo>::new();
//     unsafe {
//         for p in process::nt_get_all_process_info()? {
//             process_info_array.push(VNTProcessInfo {
//                 process_thread_count: p.number_of_threads,
//                 process_name: String::from_utf16_lossy(std::slice::from_raw_parts(
//                     p.image_name.buffer,
//                     (p.image_name.length) as usize / 2,
//                 )),
//                 process_base_priority_class: p.base_priority,
//                 process_id: p.unique_process_id as u32,
//                 process_handle_count: p.handle_count,
//                 process_session_id: p.session_id,
//                 process_peak_virtual_size: p.peak_virtual_size,
//                 process_virtual_size: p.virtual_size,
//                 process_peak_working_set_size: p.peak_working_set_size,
//                 process_working_set_size: p.working_set_size,
//                 process_quota_paged_pool_usage: p.quota_paged_pool_usage,
//                 process_quota_non_paged_pool_usage: p.quota_non_paged_pool_usage,
//                 process_pagefile_usage: p.pagefile_usage,
//                 process_peak_pagefile_usage: p.peak_pagefile_usage,
//                 process_private_page_count: p.private_page_count,
//             });
//         }
//     }
//     Ok(process_info_array)
// }

pub fn get_process_handle(process_id: u32) -> Result<*mut core::ffi::c_void> {
    unsafe {
        let process_handle = OpenProcess(0x000F0000 | 0x00100000 | 0xFFFF, 0, process_id);

        if process_handle.is_null() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("OpenProcess failed with return value: {process_handle:X?}"),
            )));
        }

        Ok(process_handle)
    }
}
