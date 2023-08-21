fn main() {
    unsafe {
        let dmi_info = vcheat::get_dmi_info().unwrap();
        println!(
            "bios_embedded_controller_firmware_version: {}",
            dmi_info.bios_embedded_controller_firmware_version.unwrap()
        );
        println!("bios_release_date: {}", dmi_info.bios_release_date.unwrap());
        println!("bios_vendor: {}", dmi_info.bios_vendor.unwrap());
        println!("bios_version: {}", dmi_info.bios_version.unwrap());

        println!("system_family: {}", dmi_info.system_family.unwrap());
        let result = dmi_info.system_guid.unwrap();
        println!("system_guid_vec: {:X?}", result.0);
        println!("system_guid_string: {}", result.1);
        let result = dmi_info.system_uuid.unwrap();
        println!("system_uuid_vec: {:X?}", result.0);
        println!("system_uuid_string: {}", result.1);
        println!(
            "system_manufacturer: {}",
            dmi_info.system_manufacturer.unwrap()
        );
        println!("system_product: {}", dmi_info.system_product.unwrap());
        println!(
            "system_serial_number: {}",
            dmi_info.system_serial_number.unwrap()
        );
        println!("system_sku_number: {}", dmi_info.system_sku_number.unwrap());
        println!("system_version: {}", dmi_info.system_version.unwrap());
    }
}
