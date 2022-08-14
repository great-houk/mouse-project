use mouse_commands::{Command, CommandError, DataType, Response};
use pmw3389_driver::Pmw3389Register;
use serialport::{Error, SerialPort};
use std::io::ErrorKind;
use std::time::Duration;

pub struct Mouse {
    port: Box<dyn SerialPort>,
}

impl Mouse {
    pub fn connect(name: String) -> Result<Mouse, Error> {
        let mut port = None;
        for _ in 0..100 {
            match serialport::new(name.clone(), 9600).open() {
                Ok(raw) => {
                    port = Some(raw);
                    break;
                }
                Err(_) => {}
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let port = port.unwrap();
        Ok(Mouse { port })
    }

    pub fn set_dpi(&mut self, dpi: u32) {
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

    pub fn get_settings(&mut self) -> MouseSettings {
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

    pub fn save_settings(&mut self) -> Result<(), ()> {
        // Send request
        self.write(Command::SaveSettings);
        // Get response
        match self.read() {
            Ok(Response::Ok) => Ok(()),
            _ => return Err(()),
        }
    }

    pub fn lorem_ipsum(&mut self) -> String {
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

    pub fn set_cpi_keys(&mut self, mods: u8, keys: [u8; 6]) -> Result<(), ()> {
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

    pub fn set_lift_keys(&mut self, mods: u8, keys: [u8; 6]) -> Result<(), ()> {
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

    pub fn say_hi(&mut self) -> Result<String, ()> {
        // Send command
        self.write(Command::SayHi);
        // Read hi
        if let Ok(Response::String(hi)) = self.read() {
            Ok(String::from_utf8(hi.to_vec()).unwrap())
        } else {
            Err(())
        }
    }

    pub fn get_angle_snap(&mut self) -> Result<bool, ()> {
        // Send command
        self.write(Command::GetSensorReg(Pmw3389Register::AngleSnap));
        if let Ok(Response::Data(DataType::U16, data)) = self.read() {
            let value = data[0] != 0;
            Ok(value)
        } else {
            Err(())
        }
    }

    pub fn set_angle_snap(&mut self, enabled: bool) -> Result<(), ()> {
        // Send command
        let val;
        if enabled {
            val = 0b1000_0000;
        } else {
            val = 0;
        }
        self.write(Command::SetSensorReg(val, Pmw3389Register::AngleSnap));
        if let Ok(Response::Ok) = self.read() {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn set_polling_rate(&mut self, rate: u8) {
        // Send command
        self.write(Command::SetPollingRate(rate))
        // We should technically recieve an ok here, but since it disconnects first,
        // there's no other choice besides assuming it worked.
    }

    pub fn start_frame_read(&mut self, frames: u16) -> Result<(), ()> {
        // Send command
        self.write(Command::StreamSensorImages(frames));

        if self.read() != Ok(Response::Ok) {
            panic!("Failed Read");
        }

        Ok(())
    }

    pub fn read_frame(&mut self) -> Result<Vec<u8>, ()> {
        todo!();
        let mut buf = [0; 64];
        let mut ind = None;
        let mut ret = Vec::with_capacity(21);

        while ind == None || ind < Some(20) {
            // self.image.read(&mut buf[..]).unwrap();
            // println!("{buf:?}");
            ind = Some(buf[1] as usize);
            while ret.len() < ind.unwrap() * 62 {
                ret.push(0);
            }
            ret.extend_from_slice(&buf[2..]);
        }
        ret.resize(36 * 36, 0);
        Ok(ret)
    }

    pub fn end_frame_read(&mut self) -> Result<(), ()> {
        // Send command
        self.write(Command::ResetSensor);

        if self.read() != Ok(Response::Ok) {
            panic!("Failed Read");
        }

        Ok(())
    }
}

mod hardware {
    use super::*;
    impl Mouse {
        pub fn write(&mut self, command: Command) {
            // Write data to device
            let mut buf = [0; 5];
            let (command, args) = command.get_command();
            buf[0] = command;
            buf[1..].copy_from_slice(&args);
            while let Err(ErrorKind::TimedOut) = self.port.write(&buf).map_err(|err| err.kind()) {}
        }

        pub fn read(&mut self) -> Result<Response, CommandError> {
            let raw = self.read_raw();
            Response::match_response(raw[0], [raw[1], raw[2], raw[3], raw[3]])
        }

        pub fn read_raw(&mut self) -> Vec<u8> {
            let mut buf = Vec::with_capacity(5);
            while self.port.bytes_to_read().unwrap() == 0 {
                print!(".")
            }
            while let Err(ErrorKind::TimedOut) = self.port.read(&mut buf).map_err(|e| e.kind()) {}
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
