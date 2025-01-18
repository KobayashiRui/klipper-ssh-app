use rusb::{Device, DeviceDescriptor, GlobalContext, UsbContext};

#[tauri::command]
pub fn get_dfu_devices() {
    list_stm_dfu_devices();
}

fn list_stm_dfu_devices() -> rusb::Result<()> {
    println!("DFUモードデバイスをスキャン中");

    // 全USBデバイスを取得
    let devices = rusb::devices()?;
    for device in devices.iter() {
        let device_desc = device.device_descriptor()?;
        // STM32 DFUモードのVIDとPIDを確認
        if device_desc.vendor_id() == 0x0483 && device_desc.product_id() == 0xDF11 {
            println!(
                "STM32 DFUモードのデバイスを検出: Vendor ID: {:04x}, Product ID: {:04x}",
                device_desc.vendor_id(),
                device_desc.product_id()
            );

            // 製品名も取得
            if let Ok(product_name) = get_product_string(&device, &device_desc) {
                println!("製品名: {}", product_name);
            }
        }
    }
    Ok(())
}

fn get_product_string(
    device: &Device<rusb::GlobalContext>,
    device_desc: &DeviceDescriptor,
) -> rusb::Result<String> {
    let handle = device.open()?; // デバイスハンドルを開く

    if let Some(product_index) = device_desc.product_string_index() {
        let product_name = handle.read_string_descriptor_ascii(product_index)?;
        Ok(product_name)
    } else {
        Ok("製品名なし".to_string())
    }
}
