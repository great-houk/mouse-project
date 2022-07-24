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

    pub fn get_settings(&self) -> MouseSettings {
        // Send request
        self.write(Command::GetSettings);
        // Get Response
        let val = self.read();
        if val != Ok(Response::DataArray(4, DataType::Settings)) {
            panic!("Failed to get settings");
        }
        // Get data
        let mut data = Vec::with_capacity(20);
        for _ in 0..4 {
            let raw = self.read_raw();
            for byte in raw {
                data.push(byte);
            }
        }
        // Get OK
        if self.read() != Ok(Response::Ok) {
            panic!("Didn't recieve Ok!");
        }
        // Get settings
        let mut buf = [0; 20];
        buf.copy_from_slice(&data);
        MouseSettings::from_bytes(&buf)
    }

    pub fn save_settings(&self) -> Result<(), ()> {
        // Send request
        self.write(Command::SaveSettings);
        // Get response
        match self.read() {
            Ok(Response::Ok) => Ok(()),
            _ => return Err(()),
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

    pub fn set_cpi_keys(&self, mods: u8, keys: [u8; 6]) -> Result<(), ()> {
        // Send command 1
        self.write(Command::Cpi1(mods, [keys[0], keys[1]]));
        // Get response
        match self.read() {
            Ok(Response::Ok) => {}
            _ => return Err(()),
        }
        // Send command 2
        self.write(Command::Cpi2([keys[2], keys[3], keys[4], keys[5]]));
        // Get response
        match self.read() {
            Ok(Response::Ok) => Ok(()),
            _ => return Err(()),
        }
    }

    pub fn set_lift_keys(&self, mods: u8, keys: [u8; 6]) -> Result<(), ()> {
        // Send command 1
        self.write(Command::Lift1(mods, [keys[0], keys[1]]));
        // Get response
        match self.read() {
            Ok(Response::Ok) => {}
            _ => return Err(()),
        }
        // Send command 2
        self.write(Command::Lift2([keys[2], keys[3], keys[4], keys[5]]));
        // Get response
        match self.read() {
            Ok(Response::Ok) => Ok(()),
            _ => return Err(()),
        }
    }

    pub fn say_hi(&self) -> Result<String, ()> {
        // Send command
        self.write(Command::SayHi);
        // Read hi
        if let Ok(Response::String(hi)) = self.read() {
            Ok(String::from_utf8(hi.to_vec()).unwrap())
        } else {
            Err(())
        }
    }
}

mod hardware {
    use super::*;
    impl Mouse {
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
}

pub struct MouseSettings {
    dpi: u16,
    cpi_keys: [u8; 6],
    cpi_mods: u8,
    lift_keys: [u8; 6],
    lift_mods: u8,
    bat_volt: f32,
}

impl MouseSettings {
    pub fn from_bytes(bytes: &[u8; 20]) -> Self {
        let dpi = u16::from_le_bytes([bytes[0], bytes[1]]);
        let mut cpi_keys = [0; 6];
        cpi_keys.copy_from_slice(&bytes[2..8]);
        let cpi_mods = bytes[8];
        let mut lift_keys = [0; 6];
        lift_keys.copy_from_slice(&bytes[9..15]);
        let lift_mods = bytes[15];
        let bat_volt = f32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        Self {
            dpi,
            cpi_keys,
            cpi_mods,
            lift_keys,
            lift_mods,
            bat_volt,
        }
    }

    pub fn dpi(&self) -> u16 {
        self.dpi * 50
    }

    pub fn cpi_keys(&self) -> [u8; 6] {
        self.cpi_keys
    }

    pub fn cpi_mods(&self) -> u8 {
        self.cpi_mods
    }

    pub fn lift_keys(&self) -> [u8; 6] {
        self.lift_keys
    }

    pub fn lift_mods(&self) -> u8 {
        self.lift_mods
    }

    pub fn bat_volt(&self) -> f32 {
        self.bat_volt
    }
}
