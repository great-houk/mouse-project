use crate::{
    flash::{Flash, FlashData, FlashError},
    mouse_report::{MouseReport, Reports},
    sensor_driver::{get_x_y, Pmw3389Driver},
};
use core::cell::RefCell;
use cortex_m::interrupt::{self as interruptm, Mutex};
use max32625::{FLC, NVIC, TMR1};
use max32625_gpio::{GpioError, GpioReg, Pin, PinInMode, PinNum, PinOutMode, Port, GPIO};
use max32625_timer_basic::{Timer, TimerError};
use mouse_commands::{Command, CommandError, DataType, Response};
use pmw3389_driver::{Pmw3389, Pmw3389Error};

pub static MOUSE: Mutex<RefCell<Option<Mouse>>> = Mutex::new(RefCell::new(None));

const LOREM_IPSUM: &str = "\
Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi 
ut aliquip ex ea commodo consequat. Duis aute irure dolor in 
reprehenderit in voluptate velit esse cillum dolore eu fugiat 
nulla pariatur. Excepteur sint occaecat cupidatat non proident, 
sunt in culpa qui officia deserunt mollit anim id est laborumaa";

#[derive(Debug, PartialEq, PartialOrd)]
pub enum MouseError {
    Gpio(GpioError),
    Flash(FlashError),
    Timer(TimerError),
    Sensor(Pmw3389Error),
}

impl From<GpioError> for MouseError {
    fn from(err: GpioError) -> Self {
        Self::Gpio(err)
    }
}

impl From<FlashError> for MouseError {
    fn from(err: FlashError) -> Self {
        Self::Flash(err)
    }
}

impl From<TimerError> for MouseError {
    fn from(err: TimerError) -> Self {
        Self::Timer(err)
    }
}

impl From<Pmw3389Error> for MouseError {
    fn from(err: Pmw3389Error) -> Self {
        Self::Sensor(err)
    }
}

pub struct Mouse {
    forward: Pin,    // Port 2, Pin 0
    back: Pin,       // Port 2, Pin 1
    lmb: Pin,        // Port 2, Pin 2
    rmb: Pin,        // Port 2, Pin 3
    mmb: Pin,        // Port 4, Pin 0
    _mode: Pin,      // Port 4, Pin 1
    cpi: Pin,        // Port 4, Pin 2
    sensor_irq: Pin, // Port 1, Pin 4
    cpi_pressed: bool,
    sensor: Pmw3389<Pmw3389Driver<TMR1>>,
    response: Option<Response>,
    command: Option<Command>,
    scroll_state: u8,
    scroll: i8,
    settings: MouseSettings,
    flash: Flash<MouseSettings>,
}

impl Mouse {
    pub fn new(
        nvic: &mut NVIC,
        gpio: &mut GPIO,
        sensor: Pmw3389<Pmw3389Driver<TMR1>>,
        timer: impl Timer,
        flc: FLC,
    ) -> Result<(), MouseError> {
        let pins = [
            (Port::Port2, PinNum::Pin0),
            (Port::Port2, PinNum::Pin1),
            (Port::Port2, PinNum::Pin2),
            (Port::Port2, PinNum::Pin3),
            (Port::Port4, PinNum::Pin0),
            (Port::Port4, PinNum::Pin1),
            (Port::Port4, PinNum::Pin2),
            (Port::Port1, PinNum::Pin4),
            (Port::Port3, PinNum::Pin0),
            (Port::Port3, PinNum::Pin1),
        ];
        let mut pins = pins.map(|(port, pin)| gpio.take(port, pin).unwrap());

        for (i, pin) in pins.iter_mut().enumerate() {
            match i {
                7 => pin.set_output(PinOutMode::HighImpedanceInput)?, // Set IRQ to high impedance
                _ => pin.set_output(PinOutMode::WeakPullup)?, // Set buttons/scroll wheel to pullup
            }
            pin.set_in_mode(PinInMode::Inverted);
        }

        let [forward, back, lmb, rmb, mmb, mode, cpi, sensor_irq, _scroll_a, _scroll_b] = pins;
        // Setup wheel loop
        timer.setup_loop_us(250, quater_ms_loop, nvic)?;

        // Setup flash
        let flash = Flash::<MouseSettings>::init(flc)?;
        // Read settings
        let settings = flash.get_data()?;

        // Update mouse sensor
        sensor.set_dpi(settings.dpi as u32 * 50)?;

        interruptm::free(|cs| {
            *MOUSE.borrow(cs).borrow_mut() = Some(Self {
                forward,
                back,
                lmb,
                rmb,
                mmb,
                _mode: mode,
                cpi,
                sensor_irq,
                cpi_pressed: false,
                sensor,
                response: None,
                command: None,
                scroll_state: 0,
                scroll: 0,
                settings,
                flash,
            });
        });

        Ok(())
    }

    pub fn get_mouse_report<'a>(&mut self, buf: &'a mut [u8]) -> &'a [u8] {
        let mut report = MouseReport {
            buttons: 0,
            x: 0,
            y: 0,
            wheel: 0,
            cpi: [0; 6],
            modifier: 0,
            command: 0,
            args: [0; 4],
            response: 0,
            data: [0; 4],
        };
        // Get which report to send
        let report_type = if let Some(response) = self.response.take() {
            // Send response
            (report.response, report.data) = response.get_response();
            Reports::CommandResponse
        } else if self.cpi.read() != self.cpi_pressed {
            self.cpi_pressed = !self.cpi_pressed;
            // Update struct
            if self.cpi_pressed {
                report.cpi = self.settings.cpi_keys;
                report.modifier = self.settings.cpi_mods;
            }
            // Use second report ID, so the button gets updated
            Reports::KeyboardReport
        } else {
            // Get Buttons
            let mouse_buttons = [
                self.lmb.read(),
                self.rmb.read(),
                self.mmb.read(),
                self.forward.read(),
                self.back.read(),
            ];
            let mut buttons = 0;
            for (i, &button) in mouse_buttons.iter().enumerate() {
                buttons |= (button as u8) << i;
            }
            // Get Mouse Movement
            let (x, y);
            if self.sensor_irq.read() {
                (x, y) = get_x_y(&self.sensor);
            } else {
                (x, y) = (0, 0);
            }
            // Get wheel movement
            let scroll = self.scroll;
            self.scroll = 0;
            // Update struct
            report.buttons = buttons;
            report.x = x;
            report.y = y;
            report.wheel = scroll;
            // Use first report ID, so the mouse gets updated
            Reports::MouseReport
        };
        // Check if there is some command we still need to process
        self.process_command();
        // Return data to write
        report.get_report(report_type, buf)
    }

    pub fn interpret_mouse_report(&mut self, [command, args @ ..]: &[u8; 5]) {
        let command = Command::match_command(*command, args);
        // Start or stop command loops for longer than one response commands
        match command {
            Ok(ref command) => match command {
                Command::LoremIpsum | Command::ReportScrollState(_) => {
                    self.command = Some(Command::Loop(0, command.get_command()))
                }
                _ => {}
            },
            Err(_) => {}
        };
        // Get intial response
        let response = match command {
            Ok(mut command) => self.get_response(&mut command),
            Err(err) => Response::Err(err),
        };
        // Return response
        self.response = Some(response);
    }

    fn process_command(&mut self) {
        // Get Response
        if let Some(mut command) = self.command {
            let response = self.get_response(&mut command);
            // Update response
            self.response = Some(response);
        }
        // Increment Loops
        if let Some(Command::Loop(ind, com)) = self.command {
            self.command = Some(Command::Loop(ind + 1, com))
        } else {
            self.command = None;
        }
    }

    fn get_response(&mut self, command: &mut Command) -> Response {
        match command {
            Command::SayHi => Response::String(['H' as u8, 'i' as u8, '\r' as u8, '\n' as u8]),
            Command::Err(err) => Response::Err(*err),
            Command::Stop => Response::Ok,
            Command::ReportScrollState(_) => Response::ScrollState(self.scroll_state),
            Command::LoremIpsum => Response::DataArray(LOREM_IPSUM.len() as u16, DataType::String),
            Command::GetDPI => match self.sensor.read_dpi() {
                Ok(dpi) => {
                    self.settings.dpi = (dpi / 50) as u16;
                    Response::Dpi(dpi)
                }
                Err(_) => Response::Err(CommandError::SensorErr),
            },
            Command::SetDPI(dpi) => {
                if *dpi < 16_000 && *dpi > 0 {
                    self.settings.dpi = (*dpi / 50) as u16;
                    match self.sensor.set_dpi(*dpi) {
                        Ok(_) => Response::Ok,
                        Err(_) => Response::Err(CommandError::SensorErr),
                    }
                } else {
                    Response::Err(CommandError::InvalidArgs)
                }
            }
            Command::SaveSettings => match self.flash.write_new_data(&self.settings) {
                Ok(_) => Response::Ok,
                Err(_) => Response::Err(CommandError::FlashErr),
            },
            Command::Cpi1(mods, keys) => {
                self.settings.cpi_mods = *mods;
                self.settings.cpi_keys[..2].copy_from_slice(keys);
                Response::Ok
            }
            Command::Cpi2(keys) => {
                self.settings.cpi_keys[2..].copy_from_slice(keys);
                Response::Ok
            }
            Command::Lift1(mods, keys) => {
                self.settings.lift_mods = *mods;
                self.settings.lift_keys[..2].copy_from_slice(keys);
                Response::Ok
            }
            Command::Lift2(keys) => {
                self.settings.lift_keys[2..].copy_from_slice(keys);
                Response::Ok
            }
            // Loop
            Command::Loop(ind, (inner, args)) => {
                // Get inner command
                let inner_command = Command::match_command(*inner, args);
                if let Err(err) = inner_command {
                    Response::Err(err)
                } else {
                    // Use inner command to get response based on ind
                    let inner_command = inner_command.unwrap();
                    let ind = *ind as usize;
                    // Iterate
                    let response = match inner_command {
                        // Lorem Ipsum Getter
                        Command::LoremIpsum => {
                            let ind = ind * 5;
                            let len = LOREM_IPSUM.len();
                            if ind < len {
                                let mut buf = [0; 5];
                                let too = usize::min(ind + 5, len);
                                buf[..too - ind].copy_from_slice(&LOREM_IPSUM.as_bytes()[ind..too]);
                                Response::RawData(buf)
                            } else {
                                Response::Ok
                            }
                        }
                        // Scroll State Getter
                        Command::ReportScrollState(iters) => {
                            if ind < iters as usize {
                                Response::ScrollState(self.scroll_state)
                            } else {
                                Response::Ok
                            }
                        }
                        // Anything else is an error
                        _ => Response::Err(CommandError::InvalidCommand),
                    };
                    // Stop iteration if the response is OK
                    if response == Response::Ok {
                        self.command = None;
                    }
                    response
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct MouseSettings {
    pub dpi: u16,
    pub cpi_mods: u8,
    pub lift_mods: u8,
    pub cpi_keys: [u8; 6],
    pub lift_keys: [u8; 6],
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            dpi: 8_000 / 50,
            cpi_mods: 0,
            lift_mods: 0b101, /* Left CTRL + Left ALT */
            cpi_keys: [0x68 /* F13 */, 0, 0, 0, 0, 0],
            lift_keys: [0x4C /* DEL */, 0, 0, 0, 0, 0],
        }
    }
}

impl FlashData for MouseSettings {
    fn serialize(&self) -> [u32; 4] {
        let mut ret = [0; 4];
        let split_dpi = self.dpi.to_le_bytes();
        ret[0] = u32::from_le_bytes([split_dpi[0], split_dpi[1], self.cpi_mods, self.lift_mods]);
        ret[1] = u32::from_le_bytes([
            self.cpi_keys[0],
            self.cpi_keys[1],
            self.cpi_keys[2],
            self.cpi_keys[3],
        ]);
        ret[2] = u32::from_le_bytes([
            self.cpi_keys[4],
            self.cpi_keys[5],
            self.lift_keys[0],
            self.lift_keys[1],
        ]);
        ret[3] = u32::from_le_bytes([
            self.lift_keys[2],
            self.lift_keys[3],
            self.lift_keys[4],
            self.lift_keys[5],
        ]);
        ret
    }

    fn from_bytes(bytes: &[u32; 4]) -> Self {
        // Turn bytes into a u8 array
        let bytes = {
            let mut temp = [0; 16];
            for i in 0..bytes.len() {
                temp[i * 4..i * 4 + 4].copy_from_slice(&bytes[i].to_le_bytes());
            }
            temp
        };
        // Get CPI keys from array
        let mut cpi = [0; 6];
        cpi.copy_from_slice(&bytes[4..10]);
        // Get lift keys from array
        let mut lift = [0; 6];
        lift.copy_from_slice(&bytes[10..16]);
        // Return self
        Self {
            dpi: u16::from_le_bytes([bytes[0], bytes[1]]),
            cpi_mods: bytes[2],
            lift_mods: bytes[3],
            cpi_keys: cpi,
            lift_keys: lift,
        }
    }
}

fn quater_ms_loop() -> bool {
    const PTR: *const max32625::gpio::RegisterBlock = max32625::GPIO::ptr();
    static mut LAST_STATE: u8 = 0;
    // Get pins
    let a_pin = unsafe { Pin::new(GpioReg::new(PTR), Port::Port3, PinNum::Pin0) };
    let b_pin = unsafe { Pin::new(GpioReg::new(PTR), Port::Port3, PinNum::Pin1) };
    // Get states
    let a = a_pin.read();
    let b = b_pin.read();
    // Get current state
    let state = (a as u8) | ((b as u8) << 1);
    let last_state = unsafe { LAST_STATE };
    unsafe { LAST_STATE = state };
    // Check change
    if state == last_state {
        return true;
    }
    // Get logic vars
    let changed = state ^ last_state;
    let mut scroll = 0;
    // Do logic
    if a == b {
        if changed == 0b10 {
            scroll = 1;
        } else {
            scroll = -1;
        }
    }
    // Update Mouse
    interruptm::free(|cs| {
        if let Some(mouse) = &mut *MOUSE.borrow(cs).borrow_mut() {
            mouse.scroll += scroll;
            mouse.scroll_state = state;
        }
    });
    // Continue loop
    true
}
