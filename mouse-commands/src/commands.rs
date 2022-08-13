use pmw3389_driver::Pmw3389Register;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CommandError {
    InvalidCommandError = 0,
    InvalidCommand = 1,
    InvalidArgs = 2,
    InvalidResponse = 3,
    SensorErr = 4,
    FlashErr = 5,
    Busy = 6,
}

impl CommandError {
    pub fn from_bytes(bytes: &[u8; 4]) -> Result<Self, CommandError> {
        match (bytes[0], &bytes[1..]) {
            (0, _) => Ok(Self::InvalidCommandError),
            (1, _) => Ok(Self::InvalidCommand),
            (2, _) => Ok(Self::InvalidArgs),
            (3, _) => Ok(Self::InvalidResponse),
            (4, _) => Ok(Self::SensorErr),
            (5, _) => Ok(Self::FlashErr),
            (6, _) => Ok(Self::Busy),
            _ => Err(Self::InvalidCommandError),
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            // Everything so far uses this pattern
            _ => [*self as u8, 0, 0, 0],
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Command {
    SayHi,
    ReportScrollState(u32 /* Iterations */),
    Stop,
    LoremIpsum,
    Err(CommandError),
    GetSettings,
    SetDPI(u32 /* DPI */),
    SaveSettings,
    Cpi1(u8 /* Mods */, [u8; 2] /* First 2 of keys array */),
    Cpi2([u8; 4] /* Last 4 of keys array */),
    Lift1(u8 /* Mods */, [u8; 2] /* First 2 of keys array */),
    Lift2([u8; 4] /* Last 4 of keys array */),
    SetSensorReg(u16 /* Value to set to */, Pmw3389Register),
    GetSensorReg(Pmw3389Register),
    SetPollingRate(u8 /* Number of ms between poll */),
    StreamSensorImages(u16 /* Number of Frames */),
    ResetSensor,
    // Not to be sent by host, ever
    Loop(u16 /* Index */, (u8, [u8; 4])),
}

impl Command {
    pub fn get_command(&self) -> (u8, [u8; 4]) {
        match self {
            // Sendable Commands
            Self::SayHi => (0, [0; 4]),
            Self::ReportScrollState(iters) => (1, iters.to_le_bytes()),
            Self::Stop => (2, [0; 4]),
            Self::LoremIpsum => (3, [0; 4]),
            Self::Err(err) => (4, err.to_bytes()),
            Self::GetSettings => (5, [0; 4]),
            Self::SetDPI(dpi) => (6, dpi.to_le_bytes()),
            Self::SaveSettings => (7, [0; 4]),
            Self::Cpi1(mods, keys) => (8, [*mods, keys[0], keys[1], 0]),
            Self::Cpi2(keys) => (9, *keys),
            Self::Lift1(mods, keys) => (10, [*mods, keys[0], keys[1], 0]),
            Self::Lift2(keys) => (11, *keys),
            Self::SetSensorReg(val, reg) => {
                let [val1, val2] = val.to_le_bytes();
                (12, [val1, val2, reg.to_u8(), 0])
            }
            Self::GetSensorReg(reg) => (13, [reg.to_u8(), 0, 0, 0]),
            Self::SetPollingRate(rate) => (14, [*rate, 0, 0, 0]),
            Self::StreamSensorImages(frames) => {
                let [val0, val1] = frames.to_le_bytes();
                (15, [val0, val1, 0, 0])
            }
            Self::ResetSensor => (16, [0; 4]),
            // Info/State Commands
            Self::Loop(_, _) => Command::Err(CommandError::InvalidCommand).get_command(),
        }
    }

    pub fn match_command(command: u8, args: &[u8; 4]) -> Result<Command, CommandError> {
        match (command, args) {
            // Say Hi Command
            (0, args) => {
                if u32::from_le_bytes(*args) % 2 == 0 {
                    Ok(Command::SayHi)
                } else {
                    Err(CommandError::InvalidArgs)
                }
            }
            // Report ScrollState Command
            (1, iters) => Ok(Command::ReportScrollState(u32::from_le_bytes(*iters))),
            // Stop Command
            (2, _) => Ok(Command::Stop),
            // Lorem Ipsum
            (3, _) => Ok(Command::LoremIpsum),
            // Err
            (4, err) => Ok(Command::Err(CommandError::from_bytes(err)?)),
            // Get DPI
            (5, _) => Ok(Command::GetSettings),
            // Set DPI
            (6, dpi) => Ok(Command::SetDPI(u32::from_le_bytes(*dpi))),
            // Save settings
            (7, _) => Ok(Command::SaveSettings),
            // Cpi 1
            (8, [mods, keys1, keys2, ..]) => Ok(Command::Cpi1(*mods, [*keys1, *keys2])),
            // Cpi 2
            (9, keys) => Ok(Command::Cpi2(*keys)),
            // Lift 1
            (10, [mods, keys1, keys2, ..]) => Ok(Command::Lift1(*mods, [*keys1, *keys2])),
            // Lift 2
            (11, keys) => Ok(Command::Lift2(*keys)),
            // Set Sensor Reg
            (12, [val1, val2, reg, ..]) => Ok(Command::SetSensorReg(
                u16::from_le_bytes([*val1, *val2]),
                Pmw3389Register::from_u8(*reg).map_err(|_| CommandError::InvalidArgs)?,
            )),
            // Get Sensor Reg
            (13, [reg, ..]) => Ok(Command::GetSensorReg(
                Pmw3389Register::from_u8(*reg).map_err(|_| CommandError::InvalidArgs)?,
            )),
            // Set Polling Rate
            (14, [rate, ..]) => Ok(Command::SetPollingRate(*rate)),
            // Stream sensor image
            (15, [val0, val1, ..]) => Ok(Command::StreamSensorImages(u16::from_le_bytes([
                *val0, *val1,
            ]))),
            // Reset Sensor
            (16, _) => Ok(Command::ResetSensor),
            // Nothing else matched, so it's an invalid command
            _ => Err(CommandError::InvalidCommand),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Response {
    Err(CommandError),
    Ok,
    String([u8; 4] /* Unicode Chars */),
    ScrollState(u8 /* State */),
    DataArray(u16 /* Length */, DataType),
    Data(DataType, [u8; 3]),
    // Things not meant to be matched
    // Raw Data
    RawData([u8; 5]),
    // Image endpoint
    ImageData(&'static [u8]),
}

impl Response {
    pub fn get_response(&self) -> &'static [u8] {
        static mut BUF: [u8; 5] = [0; 5];
        match self {
            Response::RawData(data) => unsafe {
                BUF[..data.len()].clone_from_slice(data);
                &BUF
            },
            Response::Err(err) => {
                let [err0, err1, err2, err3] = err.to_bytes();
                let ret = [0, err0, err1, err2, err3];
                unsafe {
                    BUF[..ret.len()].clone_from_slice(&ret);
                    &BUF
                }
            }
            Response::String(chars) => {
                let [ch0, ch1, ch2, ch3] = *chars;
                let ret = [1, ch0, ch1, ch2, ch3];
                unsafe {
                    BUF[..ret.len()].clone_from_slice(&ret);
                    &BUF
                }
            }
            Response::ScrollState(state) => {
                let ret = [2, *state, 0, 0, 0];
                unsafe {
                    BUF[..ret.len()].clone_from_slice(&ret);
                    &BUF
                }
            }
            Response::Ok => {
                let ret = [3, 0, 0, 0, 0];
                unsafe {
                    BUF[..ret.len()].clone_from_slice(&ret);
                    &BUF
                }
            }
            Response::DataArray(len, data) => {
                let [len0, len1] = len.to_le_bytes();
                let ret = [4, len0, len1, *data as u8, 0];
                unsafe {
                    BUF[..ret.len()].clone_from_slice(&ret);
                    &BUF
                }
            }
            Response::Data(ty, data) => {
                let ret = [5, *ty as u8, data[0], data[1], data[2]];
                unsafe {
                    BUF[..ret.len()].clone_from_slice(&ret);
                    &BUF
                }
            }
            Response::ImageData(data) => data,
        }
    }

    pub fn match_response(response: u8, data: [u8; 4]) -> Result<Response, CommandError> {
        match (response, data) {
            // Err Response
            (0, data) => {
                let command_err = CommandError::from_bytes(&data)?;
                Ok(Response::Err(command_err))
            }
            // String
            (1, chars) => Ok(Response::String(chars)),
            // Scroll State
            (2, [state, ..]) => Ok(Response::ScrollState(state)),
            // Ok
            (3, _) => Ok(Response::Ok),
            // Data Array
            (4, [len0, len1, data, ..]) => Ok(Response::DataArray(
                u16::from_le_bytes([len0, len1]),
                DataType::from(data)?,
            )),
            // Data
            (5, [ty, data @ ..]) => Ok(Response::Data(DataType::from(ty)?, data)),
            // Nothing/Raw Data
            _ => Err(CommandError::InvalidResponse),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum DataType {
    String = 0,
    RGB = 1,
    Settings = 2,
    Bool = 3,
    U8 = 4,
    U16 = 5,
}

impl DataType {
    pub fn from(val: u8) -> Result<Self, CommandError> {
        match val {
            0 => Ok(Self::String),
            1 => Ok(Self::RGB),
            2 => Ok(Self::Settings),
            3 => Ok(Self::Bool),
            4 => Ok(Self::U8),
            5 => Ok(Self::U16),
            _ => Err(CommandError::InvalidArgs),
        }
    }
}
