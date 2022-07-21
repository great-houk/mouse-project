use hidapi::HidApi;

#[cfg(test)]
mod tests {

    #[test]
    fn test_hello() {
        // let api = hidapi::HidApi::new().unwrap();
        // // Find mouse
        // let mut mouse = None;
        // for device in api.device_list() {
        //     if device.vendor_id() == VID
        //         && device.product_id() == PID
        //         && device.usage_page() == USAGE_PAGE
        //     {
        //         mouse = Some(device);
        //         break;
        //     }
        // }
        // let mouse = mouse.expect("Couldn't Connect to Mouse");

        // // Connect to device using its VID and PID
        // let device = api.open_path(mouse.path()).unwrap();

        // // Write data to device
        // let buf = [0x03, 0x00, 0x00, 0x00, 0x00, 0x00];
        // device.send_feature_report(&buf).unwrap();

        // // Read data from device
        // let mut buf = [0u8; 6];
        // let res = device.read(&mut buf[..]).unwrap();
        // println!("Read: {:?}", &buf[..res]);
        // for &byte in &buf[..res] {
        //     print!("{:?}", char::from_u32(byte as u32));
        // }
    }
}
