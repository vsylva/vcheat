fn main() {
    let dmi_info = vcheat::system::get_dmi_info().unwrap();

    println!(
        "bios_embedded_controller_firmware_version: {}",
        dmi_info.bios_embedded_controller_firmware_version
    );

    println!("bios_release_date: {}", dmi_info.bios_release_date);

    println!("bios_vendor: {}", dmi_info.bios_vendor);

    println!("bios_version: {}", dmi_info.bios_version);

    println!("system_family: {}", dmi_info.system_family);

    let guid = dmi_info.system_guid;

    println!("system_guid_vec: {:X?}", guid.0);

    println!("system_guid_string: {}", guid.1);

    let uuid = dmi_info.system_uuid;

    println!("system_uuid_vec: {:X?}", uuid.0);

    println!("system_uuid_string: {}", uuid.1);

    println!("system_manufacturer: {}", dmi_info.system_manufacturer);

    println!("system_product: {}", dmi_info.system_product);

    println!("system_serial_number: {}", dmi_info.system_serial_number);

    println!("system_sku_number: {}", dmi_info.system_sku_number);

    println!("system_version: {}", dmi_info.system_version);
}
