use hidapi::HidDevice;
use mouse_commands::{Command, CommandError, DataType, Response};

pub struct Mouse {
    device: HidDevice,
    report_id: u8,
}

impl Mouse {
    pub fn connect(vid: u16, pid: u16, usage_page: u16, report_id: u8) -> Mouse {
        let api = hidapi::HidApi::new().unwrap();
        // Find mouse
        let mut mouse = None;
        for device in api.device_list() {
            if device.vendor_id() == vid
                && device.product_id() == pid
                && device.usage_page() == usage_page
            {
                mouse = Some(device);
                break;
            }
        }
        let mouse = mouse.expect("Couldn't Connect to Mouse, maybe VID/PID/usage_page are wrong?");

        // Connect to device using its VID and PID
        let device = api.open_path(mouse.path()).unwrap();
        device.set_blocking_mode(true).unwrap();
        Mouse { device, report_id }
    }

    pub fn set_dpi(&self, dpi: u32) {
        // Send Request
        self.write(Command::SetDPI(dpi));
        // Get Response
        let val = self.read();
        if let Ok(Response::Ok) = val {
            return;
        } else {
            panic!("Failed to set mouse DPI: {val:?}");
        }
    }

    pub fn read_dpi(&self) -> u32 {
        // Send request
        self.write(Command::GetDPI);
        // Get Response
        let val = self.read();
        if let Ok(Response::Dpi(dpi)) = val {
            return dpi;
        } else {
            panic!("Failed to read DPI: {val:?}");
        }
    }

    pub fn save_settings(&self) -> Result<(), ()> {
        // Send request
        self.write(Command::SaveSettings);
        // Get Response
        match self.read().unwrap() {
            Response::Ok => Ok(()),
            Response::Err(_) => Err(()),
            _ => unreachable!(),
        }
    }

    pub fn lorem_ipsum(&self) -> String {
        // Request Lorem Ipsum
        self.write(Command::LoremIpsum);
        // Read Incoming Data Array
        let size;
        if let Response::DataArray(len, DataType::String) = self.read().unwrap() {
            size = len;
        } else {
            panic!("Wrong Response!");
        }
        // Read raw data
        let mut buffer = Vec::with_capacity(size as usize);
        let mut i = 0;
        'outer: loop {
            // Get data
            let raw_data = self.read_raw();
            // Push it into the buffer
            for byte in raw_data {
                buffer.push(byte);
                i += 1;
                if i >= size {
                    break 'outer;
                }
            }
        }
        // Read OK that comes when finished
        if let Response::Ok = self.read().unwrap() {
            let string = String::from_utf8(buffer).unwrap();
            return string;
        } else {
            panic!("Mouse Failed to OK at end of data!");
        }
    }

    pub fn write(&self, command: Command) {
        // Write data to device
        let mut buf = [self.report_id; 6];
        let (command, args) = command.get_command();
        buf[1] = command;
        buf[2..].copy_from_slice(&args);
        self.device.send_feature_report(&buf).unwrap();
    }

    pub fn read(&self) -> Result<Response, CommandError> {
        let [response, data @ ..] = self.read_raw();
        Response::match_response(response, data)
    }

    pub fn read_raw(&self) -> [u8; 5] {
        let mut buf = [0; 6];
        self.device.read(&mut buf[..]).unwrap();
        let [_, buf @ ..] = buf;
        buf
    }
}
