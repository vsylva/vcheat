use crate::*;

pub fn get_logical_cpu_count() -> usize {
    unsafe {
        let system_info = &mut core::mem::zeroed() as *mut SystemInfo;

        GetSystemInfo(system_info);

        system_info.read().dw_number_of_processors as usize
    }
}

/// Some of the code in the function is based on Neacsu's C++ code from
/// <https://blog.csdn.net/youyudexiaowangzi/article/details/122308734>
/// <https://www.codeproject.com/Tips/5263343/How-to-Get-the-BIOS-UUID>
pub fn get_dmi_info() -> Result<DmiInfo> {
    unsafe {
        let signature = *b"RSMB";
        let signature: u32 = ((signature[0] as u32) << 24)
            | ((signature[1] as u32) << 16)
            | ((signature[2] as u32) << 8)
            | (signature[3] as u32);

        let mut return_value = GetSystemFirmwareTable(signature, 0, core::ptr::null_mut(), 0);

        if return_value == 0 {
            return Err(format!(
                "GetSystemFirmwareTable failed with return value: {return_value:X}"
            ));
        }

        let mut buffer = vec![0u8; return_value as usize];

        return_value = GetSystemFirmwareTable(signature, 0, buffer.as_mut_ptr(), return_value);

        if return_value == 0 {
            return Err(format!(
                "GetSystemFirmwareTable failed with return value: {return_value:X}"
            ));
        }

        let get_string_by_dmi = |dm_header: *const DmiHeader, mut value: u8| -> Result<String> {
            let get_c_str_len = |cstr: *const i8| -> isize {
                let mut len = 0;
                while cstr.offset(len).read() != 0 {
                    len += 1;
                }
                len
            };

            let base = dm_header as *const i8;

            if value == 0 {
                return Err("Invalid".to_string());
            }

            let mut base = base.add(dm_header.read().length as usize);

            while value > 1 && base.read() != 0 {
                base = base.add(get_c_str_len(base) as usize);
                base = base.add(1);
                value -= 1;
            }

            if base.read() == 0 {
                return Err("Bad index".to_string());
            }

            let len = get_c_str_len(base);

            let bp_vec = std::slice::from_raw_parts(base.cast::<u8>(), len as usize);

            Ok(String::from_utf8_lossy(bp_vec).to_string())
        };

        let smb = RawSMBIOSData {
            used20_calling_method: buffer[0],
            smbiosmajor_version: buffer[1],
            smbiosminor_version: buffer[2],
            dmi_revision: buffer[3],
            length: u32::from_ne_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
            smbiostable_data: buffer[8..].to_vec(),
        };

        let mut dmi_info = DmiInfo {
            bios_version: None,
            bios_release_date: None,
            bios_vendor: None,
            bios_embedded_controller_firmware_version: None,
            system_manufacturer: None,
            system_product: None,
            system_version: None,
            system_serial_number: None,
            system_uuid: None,
            system_guid: None,
            system_sku_number: None,
            system_family: None,
        };

        let mut uuid = vec![0u8; 16];

        let mut data = smb.smbiostable_data.as_ptr();

        let mut once_flag = false;

        while (data as usize) < smb.smbiostable_data.as_ptr() as usize + smb.length as usize {
            let mut next: *const u8;
            let h: *const DmiHeader = data.cast();

            if h.read().length < 4 {
                break;
            }

            if h.read().ctype == 0 && once_flag == false {
                if let Ok(bios_vendor) = get_string_by_dmi(h, data.offset(0x4).read()) {
                    dmi_info.bios_vendor = Some(bios_vendor);
                }

                if let Ok(bios_version) = get_string_by_dmi(h, data.offset(0x5).read()) {
                    dmi_info.bios_version = Some(bios_version);
                }

                if let Ok(bios_release_date) = get_string_by_dmi(h, data.offset(0x8).read()) {
                    dmi_info.bios_release_date = Some(bios_release_date);
                }

                if data.offset(0x16).read() != 0xFF && data.offset(0x17).read() != 0xFF {
                    dmi_info.bios_embedded_controller_firmware_version = Some(format!(
                        "{}.{}",
                        data.offset(0x16).read(),
                        data.offset(0x17).read()
                    ));
                }

                once_flag = true;
            }

            if h.read().ctype == 0x01 && h.read().length >= 0x19 {
                if let Ok(manufacturer) = get_string_by_dmi(h, data.offset(0x4).read()) {
                    dmi_info.system_manufacturer = Some(manufacturer);
                }

                if let Ok(product) = get_string_by_dmi(h, data.offset(0x5).read()) {
                    dmi_info.system_product = Some(product);
                }

                if let Ok(version) = get_string_by_dmi(h, data.offset(0x6).read()) {
                    dmi_info.system_version = Some(version);
                }

                if let Ok(serial_number) = get_string_by_dmi(h, data.offset(0x7).read()) {
                    dmi_info.system_serial_number = Some(serial_number);
                }

                if let Ok(sku_number) = get_string_by_dmi(h, data.offset(0x19).read()) {
                    dmi_info.system_sku_number = Some(sku_number);
                }

                if let Ok(family) = get_string_by_dmi(h, data.offset(0x1A).read()) {
                    dmi_info.system_family = Some(family);
                }

                data = data.add(0x8);

                let mut all_zero = true;

                let mut all_one = true;

                let mut i = 0;
                while i < 16 && (all_zero || all_one) {
                    if data.offset(i).read() != 0x00 {
                        all_zero = false;
                    }
                    if data.offset(i).read() != 0xFF {
                        all_one = false;
                    }
                    i += 1;
                }

                if !all_zero && !all_one {
                    for i in 0..4 {
                        uuid[i] = data.offset(i as isize).read();
                    }

                    uuid[5] = data.offset(5).read();
                    uuid[4] = data.offset(4).read();
                    uuid[7] = data.offset(7).read();
                    uuid[6] = data.offset(6).read();

                    for j in 8..16 {
                        uuid[j] = data.offset(j as isize).read();
                    }

                    let mut uuid_string = String::new();
                    for i in 0..16 {
                        uuid_string.push_str(format!("{:02X}", uuid[i]).as_str());
                        if (i + 1) % 4 == 0 && i != 15 {
                            uuid_string.push('-');
                        }
                    }

                    let mut guid = uuid.clone();

                    for (i, j) in (0..4).zip((0..4).rev()) {
                        guid[i] = uuid[j];
                    }

                    guid[4] = uuid[5];
                    guid[5] = uuid[4];
                    guid[6] = uuid[7];
                    guid[7] = uuid[6];

                    dmi_info.system_uuid = Some((uuid, uuid_string));

                    let mut guid_string = String::new();
                    for i in 0..16 {
                        guid_string.push_str(format!("{:02X}", guid[i]).as_str());
                        if i == 3 {
                            guid_string.push('-');
                        }
                        if i % 2 == 1 && i < 10 && i > 4 {
                            guid_string.push('-');
                        }
                    }
                    dmi_info.system_guid = Some((guid, guid_string));
                }
                break;
            }

            next = data.add(h.read().length as usize);

            while (next as usize) < smb.smbiostable_data.as_ptr() as usize + smb.length as usize
                && (next.offset(0).read() != 0 || next.offset(1).read() != 0)
            {
                next = next.add(1);
            }

            next = next.add(2);

            data = next;
        }
        Ok(dmi_info)
    }
}
