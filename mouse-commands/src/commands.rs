#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CommandError {
    InvalidCommandError = 0,
    InvalidCommand = 1,
    InvalidArgs = 2,
    InvalidResponse = 3,
    SensorErr = 4,
    FlashErr = 5,
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
            _ => Err(Self::InvalidCommandError),
        }
    }

    pub fn to_bytes(self) -> [u8; 4] {
        match self {
            // Everything so far uses this pattern
            _ => [self as u8, 0, 0, 0],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
    // Not to be sent by host, ever
    Loop(u16 /* Index */, (u8, [u8; 4])),
}

impl Command {
    pub fn get_command(&self) -> (u8, [u8; 4]) {
        match self {
            // Sendable Commands
            Command::SayHi => (0, [0; 4]),
            Command::ReportScrollState(iters) => (1, iters.to_le_bytes()),
            Command::Stop => (2, [0; 4]),
            Command::LoremIpsum => (3, [0; 4]),
            Command::Err(err) => (4, err.to_bytes()),
            Command::GetSettings => (5, [0; 4]),
            Command::SetDPI(dpi) => (6, dpi.to_le_bytes()),
            Command::SaveSettings => (7, [0; 4]),
            Command::Cpi1(mods, keys) => (8, [*mods, keys[0], keys[1], 0]),
            Command::Cpi2(keys) => (9, *keys),
            Command::Lift1(mods, keys) => (10, [*mods, keys[0], keys[1], 0]),
            Command::Lift2(keys) => (11, *keys),
            // Info/State Commands
            Command::Loop(_, _) => Command::Err(CommandError::InvalidCommand).get_command(),
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
            // Nothing else matched, so it's an invalid command
            _ => Err(CommandError::InvalidCommand),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Response {
    Err(CommandError),
    Ok,
    String([u8; 4] /* 4 Unicode Chars */),
    ScrollState(u8 /* State */),
    DataArray(u16 /* Length */, DataType),
    RawData([u8; 5]),
}

impl Response {
    pub fn get_response(self) -> (u8, [u8; 4]) {
        match self {
            Response::RawData([first, rest @ ..]) => (first, rest),
            Response::Err(err) => (0, err.to_bytes()),
            Response::String(chars) => (1, chars),
            Response::ScrollState(state) => (2, [state, 0, 0, 0]),
            Response::Ok => (3, [0, 0, 0, 0]),
            Response::DataArray(len, data) => {
                let [len0, len1] = len.to_le_bytes();
                (4, [len0, len1, data as u8, 0])
            }
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
}

impl DataType {
    pub fn from(val: u8) -> Result<Self, CommandError> {
        match val {
            0 => Ok(Self::String),
            1 => Ok(Self::RGB),
            2 => Ok(Self::Settings),
            _ => Err(CommandError::InvalidArgs),
        }
    }
}
