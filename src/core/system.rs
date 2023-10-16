macro_rules! location {
    () => {
        format!("[{}:{}]", file!(), line!())
    };

    ($val:literal) => {
        format!("[{}:{}]\t\"{}\"", file!(), line!(), $val)
    };

    ($($val:expr),*) => {
        {
            let mut text =  format!("[{}:{}]", file!(), line!());

            text.push('\t');

            text.push('\"');

            $(
                text += &format!("{} = {:?}", stringify!($val), $val);

                text.push('\t');
            )*

            text = text.trim_end().to_string();

            text.push('\"');

            text
        }
    };
}

use crate::{core, ffi};

pub(crate) unsafe fn get_system_info() -> core::SystemInfo {
    let mut system_info: ffi::SystemInfo = ::core::mem::zeroed::<ffi::SystemInfo>();

    ffi::GetSystemInfo(&mut system_info);

    core::SystemInfo {
        processor_architecture: system_info
            .dummy_union
            .dummy_struct
            .w_processor_architecture,
        reserved: system_info.dummy_union.dummy_struct.w_reserved,
        page_size: system_info.dw_page_size,
        minimum_application_address: system_info.lp_minimum_application_address,
        maximum_application_address: system_info.lp_maximum_application_address,
        active_processor_mask: system_info.dw_active_processor_mask,
        number_of_processors: system_info.dw_number_of_processors,
        processor_type: system_info.dw_processor_type,
        allocation_granularity: system_info.dw_allocation_granularity,
        processor_level: system_info.w_processor_level,
        processor_revision: system_info.w_processor_revision,
    }
}

pub(crate) unsafe fn get_dmi_info() -> Result<core::DmiInformation, String> {
    let signature = u32::from_be_bytes(*b"RSMB");

    let mut return_length: u32 =
        ffi::GetSystemFirmwareTable(signature, 0, ::core::ptr::null_mut(), 0);

    let mut buffer: Vec<u8> = vec![0; return_length as usize];

    return_length = ffi::GetSystemFirmwareTable(signature, 0, buffer.as_mut_ptr(), return_length);

    if return_length > return_length {
        return Err(location!());
    }

    let get_string_by_dmi: fn(*const ffi::DmiHeader, u8) -> Result<String, String> =
        |dm_header: *const ffi::DmiHeader, mut index: u8| -> Result<String, String> {
            if index == 0 {
                return Err(location!());
            }

            let mut base_address: *const i8 =
                dm_header.cast::<i8>().add(dm_header.read().length as usize);

            while index > 1 && base_address.read() != 0 {
                let strlen = ::std::ffi::CStr::from_ptr(base_address)
                    .to_str()
                    .map_err(|err| err.to_string())?
                    .len();

                base_address = base_address.add(strlen + 1);

                index -= 1;
            }

            if base_address.read() == 0 {
                return Err(location!());
            }

            let strlen: usize = ::std::ffi::CStr::from_ptr(base_address)
                .to_str()
                .map_err(|err| err.to_string())?
                .len();

            let sm_data: Vec<u8> =
                ::std::slice::from_raw_parts(base_address.cast::<u8>(), strlen + 1).to_vec();

            let sm_cstring: ::std::ffi::CString =
                ::std::ffi::CString::from_vec_with_nul(sm_data).map_err(|e| e.to_string())?;

            let result: String = match sm_cstring.to_str() {
                Ok(ok) => ok.trim_end_matches('\0').to_string(),
                Err(err) => return Err(err.to_string()),
            };

            Ok(result)
        };

    let smb: ffi::RawSMBIOSData = ffi::RawSMBIOSData {
        used20_calling_method: buffer[0],
        smbiosmajor_version: buffer[1],
        smbiosminor_version: buffer[2],
        dmi_revision: buffer[3],
        length: u32::from_ne_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
        smbiostable_data: buffer[8..].to_vec(),
    };

    let mut dmi_info: core::DmiInformation = core::DmiInformation::default();

    let mut uuid: [u8; 16] = [0; 16];

    let mut sm_data: *const u8 = smb.smbiostable_data.as_ptr();

    let mut once_flag: bool = false;

    while sm_data < smb.smbiostable_data.as_ptr().add(smb.length as usize) {
        let dmi_header: *const ffi::DmiHeader = sm_data.cast();

        if dmi_header.read().length < 4 {
            break;
        }

        if dmi_header.read().type_ == 0 && once_flag == false {
            if let Ok(bios_vendor) = get_string_by_dmi(dmi_header, sm_data.offset(0x4).read()) {
                dmi_info.bios_vendor = bios_vendor;
            }

            if let Ok(bios_version) = get_string_by_dmi(dmi_header, sm_data.offset(0x5).read()) {
                dmi_info.bios_version = bios_version;
            }

            if let Ok(bios_release_date) = get_string_by_dmi(dmi_header, sm_data.offset(0x8).read())
            {
                dmi_info.bios_release_date = bios_release_date;
            }

            if sm_data.offset(0x16).read() != 0xFF && sm_data.offset(0x17).read() != 0xFF {
                dmi_info.bios_embedded_controller_firmware_version = format!(
                    "{}.{}",
                    sm_data.offset(0x16).read(),
                    sm_data.offset(0x17).read()
                );
            }

            once_flag = true;
        }

        if dmi_header.read().type_ == 0x01 && dmi_header.read().length >= 0x19 {
            if let Ok(manufacturer) = get_string_by_dmi(dmi_header, sm_data.offset(0x4).read()) {
                dmi_info.system_manufacturer = manufacturer;
            }

            if let Ok(product) = get_string_by_dmi(dmi_header, sm_data.offset(0x5).read()) {
                dmi_info.system_product = product;
            }

            if let Ok(version) = get_string_by_dmi(dmi_header, sm_data.offset(0x6).read()) {
                dmi_info.system_version = version;
            }

            if let Ok(serial_number) = get_string_by_dmi(dmi_header, sm_data.offset(0x7).read()) {
                dmi_info.system_serial_number = serial_number;
            }

            if let Ok(sku_number) = get_string_by_dmi(dmi_header, sm_data.offset(0x19).read()) {
                dmi_info.system_sku_number = sku_number;
            }

            if let Ok(family) = get_string_by_dmi(dmi_header, sm_data.offset(0x1A).read()) {
                dmi_info.system_family = family;
            }

            sm_data = sm_data.add(0x8);

            let mut all_zero: bool = true;

            let mut all_one: bool = true;

            let mut i: isize = 0;

            while i < 16 && (all_zero || all_one) {
                if sm_data.offset(i).read() != 0x00 {
                    all_zero = false;
                }

                if sm_data.offset(i).read() != 0xFF {
                    all_one = false;
                }

                i += 1;
            }

            if !all_zero && !all_one {
                for i in 0..4 {
                    uuid[i] = sm_data.add(i).read();
                }

                uuid[5] = sm_data.offset(5).read();

                uuid[4] = sm_data.offset(4).read();

                uuid[7] = sm_data.offset(7).read();

                uuid[6] = sm_data.offset(6).read();

                for j in 8..16 {
                    uuid[j] = sm_data.add(j).read();
                }

                let mut uuid_string: String = String::new();

                for i in 0..16 {
                    uuid_string.push_str(format!("{:02X}", uuid[i]).as_str());

                    if (i + 1) % 4 == 0 && i != 15 {
                        uuid_string.push('-');
                    }
                }

                let mut guid: [u8; 16] = uuid;

                guid[0] = uuid[3];

                guid[1] = uuid[2];

                guid[2] = uuid[1];

                guid[3] = uuid[0];

                guid[4] = uuid[5];

                guid[5] = uuid[4];

                guid[6] = uuid[7];

                guid[7] = uuid[6];

                dmi_info.system_uuid = (uuid, uuid_string);

                let mut guid_string: String = String::new();

                for i in 0..16 {
                    guid_string.push_str(format!("{:02X}", guid[i]).as_str());

                    if i == 3 {
                        guid_string.push('-');
                    }

                    if i % 2 == 1 && i < 10 && i > 4 {
                        guid_string.push('-');
                    }
                }

                dmi_info.system_guid = (guid, guid_string);
            }

            break;
        }

        let mut next: *const u8 = sm_data.add(dmi_header.read().length as usize);

        while next < smb.smbiostable_data.as_ptr().add(smb.length as usize)
            && (next.offset(0).read() != 0 || next.offset(1).read() != 0)
        {
            next = next.add(1);
        }

        sm_data = next.add(2);
    }

    Ok(dmi_info)
}
