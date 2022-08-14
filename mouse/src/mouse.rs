use crate::{
    flash::{Flash, FlashData, FlashError},
    mouse_report::{MouseReport, Reports},
    sensor_driver::Pmw3389Driver,
    static_borrow::StaticBorrow,
    usb::{reset_usb, POLLING_RATE},
};
use core::cell::RefCell;
use cortex_m::interrupt::{self as interruptm, Mutex};
use max32625::{ADC, CLKMAN, FLC, NVIC, TMR1};
use max32625_adc::{Adc, AdcChannel, AdcSettings};
use max32625_gpio::{GpioError, GpioReg, Pin, PinInMode, PinNum, PinOutMode, Port, GPIO};
use max32625_timer_basic::{Timer, TimerError};
use mouse_commands::{Command, CommandError, DataType, Response};
use mouse_error::*;
use mouse_settings::*;
use pmw3389_driver::{Pmw3389, Pmw3389Error, Pmw3389Register};

pub static MOUSE: Mutex<RefCell<Option<Mouse>>> = Mutex::new(RefCell::new(None));
static IMAGE: StaticBorrow<[u8; 36 * 36]> = StaticBorrow::new();

const LOREM_IPSUM: &str = "\
Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi 
ut aliquip ex ea commodo consequat. Duis aute irure dolor in 
reprehenderit in voluptate velit esse cillum dolore eu fugiat 
nulla pariatur. Excepteur sint occaecat cupidatat non proident, 
sunt in culpa qui officia deserunt mollit anim id est laborumaa";

const LIFTED_TIMER_MAX: u16 = 100 * 4; // 100ms

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum LiftedStatus {
    Down,
    Lifted,
    SendKeys,
    KeysSent,
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
    lifted_status: LiftedStatus,
    lifted_timer: u16,
    sensor: Pmw3389<Pmw3389Driver<TMR1>>,
    response: Option<Response>,
    command: Option<Command>,
    scroll_state: u8,
    scroll: i8,
    settings: MouseSettings,
    flash: Flash<MouseSettings>,
    adc: Adc,
    bat_voltage: f32,
}

impl Mouse {
    pub fn new(
        nvic: &mut NVIC,
        gpio: &mut GPIO,
        clkman: &CLKMAN,
        sensor: Pmw3389<Pmw3389Driver<TMR1>>,
        timer: impl Timer,
        adc: ADC,
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
        sensor.write(settings.dpi, Pmw3389Register::Resolution)?;

        // Setup ADC
        let adc = Adc::init(adc, clkman);

        // Instantiate Image
        IMAGE.set([0; 36 * 36]);

        // Set static mouse var
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
                lifted_status: LiftedStatus::Down,
                lifted_timer: 0,
                sensor,
                response: None,
                command: None,
                scroll_state: 0,
                scroll: 0,
                settings,
                flash,
                adc,
                bat_voltage: 0.0,
            });
        });

        Ok(())
    }

    pub fn get_mouse_hid<'a>(&mut self, buf: &'a mut [u8]) -> &'a [u8] {
        let mut report = MouseReport {
            buttons: 0,
            x: 0,
            y: 0,
            wheel: 0,
            keys: [0; 6],
            modifier: 0,
        };
        // Get which report to send
        let report_type = self.get_report_type();
        // Set fields in report based on type
        match report_type {
            Reports::CpiReport => {
                self.cpi_pressed = !self.cpi_pressed;
                // Put keys in if CPI is pressed
                if self.cpi_pressed {
                    report.keys = self.settings.cpi_keys;
                    report.modifier = self.settings.cpi_mods;
                }
            }
            Reports::LiftReport => {
                // Put keys in if this is the first lifted report
                if self.lifted_status == LiftedStatus::SendKeys {
                    self.lifted_status = LiftedStatus::KeysSent;
                    report.keys = self.settings.lift_keys;
                    report.modifier = self.settings.lift_mods;
                }
                // If lifted sent keys last time, all we need to do
                // is update the status to not send anymore lift packets.
                else if self.lifted_status == LiftedStatus::KeysSent {
                    self.lifted_status = LiftedStatus::Down;
                }
            }
            Reports::MouseReport => {
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
                report.buttons = buttons;
                // Get Mouse Movement if IRQ is set
                if self.sensor_irq.read() {
                    if let Ok(motion) = self.sensor.read_motion_regs() {
                        report.x = motion.delta_x();
                        report.y = motion.delta_y();
                        if motion.motion().lifted() {
                            self.lifted_status = LiftedStatus::Lifted;
                        } else {
                            self.lifted_status = LiftedStatus::Down;
                        }
                    }
                }
                // Get wheel movement
                report.wheel = self.scroll;
                self.scroll = 0;
            }
        };
        // Return data to write
        report.get_hid(report_type, buf)
    }

    /// SAFTEY: Can only be called if the static array returned previously has no references.
    /// This is because internally a static buffer is used to hand out data, and any data previously handed
    /// out will be overwritten on the next call. This is UB if the previous call's array still has references.
    pub unsafe fn get_report(&mut self) -> Option<&'static [u8]> {
        // Check if there is some command we still need to process
        self.process_pending_commands();
        // Send response
        self.response.map(|response| response.get_response())
    }

    pub fn interpret_mouse_report(&mut self, [command, args @ ..]: &[u8; 5]) {
        let command = Command::match_command(*command, args);
        // Start or stop command loops for longer than one response commands
        self.command = Some(match command {
            Ok(command) => command,
            Err(err) => Command::Err(err),
        })
    }

    pub fn update_battery_level(&mut self) {
        const BATTERY_SETTINGS: AdcSettings = AdcSettings::new(AdcChannel::AIN0HV);
        // Get voltage level
        let voltage = self.adc.get_value(BATTERY_SETTINGS);
        // Process it
        /*
        Channels 4 - 5: (AIN0 / 5), (AIN1 / 5):
            AdcData[9 : 0] = round { (AIN / (5×VREF)) × MAX }
            VREF = 1.2
            MAX = 0x3FF
        */
        let voltage = (voltage as f32 / 0x3FF as f32) * 5. * 1.2;
        self.bat_voltage = voltage;
    }

    fn process_pending_commands(&mut self) {
        // Get Response
        let response = self.get_response();
        // Update response
        self.response = response;
        // Dispose/Update command
        if let Some(ref command) = self.command {
            match command {
                Command::LoremIpsum
                | Command::ReportScrollState(_)
                | Command::GetSettings
                | Command::StreamSensorImages(_) => {
                    self.command = Some(Command::Loop(0, command.get_command()))
                }
                Command::Loop(ind, com) => self.command = Some(Command::Loop(ind + 1, *com)),
                _ => self.command = None,
            }
        }
    }

    fn get_report_type(&self) -> Reports {
        // Priorities:
        // 1. Send keyboard reports, lets say CPI first
        // 2. Send other keyboard report, lift
        // 3. Send normal mouse reports
        if self.cpi.read() != self.cpi_pressed {
            Reports::CpiReport
        } else if self.lifted_status == LiftedStatus::SendKeys
            || self.lifted_status == LiftedStatus::KeysSent
        {
            Reports::LiftReport
        } else {
            Reports::MouseReport
        }
    }

    fn get_response(&mut self) -> Option<Response> {
        if let Some(ref mut command) = self.command {
            let response = match command {
                Command::SayHi => Response::String(['H' as u8, 'i' as u8, '\r' as u8, '\n' as u8]),
                Command::Err(err) => Response::Err(*err),
                Command::Stop => Response::Ok,
                Command::ReportScrollState(_) => Response::ScrollState(self.scroll_state),
                Command::LoremIpsum => {
                    Response::DataArray(LOREM_IPSUM.len() as u16, DataType::String)
                }
                Command::GetSettings => Response::DataArray(4, DataType::Settings),
                Command::SetDPI(dpi) => {
                    if *dpi < 16_000 && *dpi > 0 {
                        self.settings.dpi = (*dpi / 50) as u16;
                        match self
                            .sensor
                            .write(self.settings.dpi, Pmw3389Register::Resolution)
                        {
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
                Command::SetSensorReg(val, reg) => match self.sensor.write(*val, *reg) {
                    Ok(_) => Response::Ok,
                    Err(_) => Response::Err(CommandError::SensorErr),
                },
                Command::GetSensorReg(reg) => match self.sensor.read(*reg) {
                    Ok(value) => {
                        let [val1, val2] = value.to_le_bytes();
                        Response::Data(DataType::U16, [val1, val2, 0])
                    }
                    Err(_) => Response::Err(CommandError::SensorErr),
                },
                Command::SetPollingRate(rate) => {
                    unsafe { POLLING_RATE = *rate };
                    reset_usb();
                    Response::Ok
                }
                Command::StreamSensorImages(frames) => {
                    if *frames > 0 {
                        *frames = *frames + 1;
                    }
                    Response::Ok
                }
                Command::ResetSensor => {
                    // Reset sensor
                    if let Err(_) = self.sensor.reset_sensor() {
                        Response::Err(CommandError::SensorErr)
                    } else if let Err(_) = self
                        .sensor
                        .write(self.settings.dpi, Pmw3389Register::Resolution)
                    {
                        Response::Err(CommandError::SensorErr)
                    } else {
                        Response::Ok
                    }
                }
                // Loop
                Command::Loop(ind_mut, (inner, args)) => {
                    // Get inner command
                    let inner_command = Command::match_command(*inner, args);
                    if let Err(err) = inner_command {
                        Response::Err(err)
                    } else {
                        // Use inner command to get response based on ind
                        let inner_command = inner_command.unwrap();
                        let ind = *ind_mut as usize;
                        // Iterate
                        let response = match inner_command {
                            // Lorem Ipsum Getter
                            Command::LoremIpsum => {
                                let ind = ind * 5;
                                let len = LOREM_IPSUM.len();
                                if ind < len {
                                    let mut buf = [0; 5];
                                    let too = usize::min(ind + 5, len);
                                    buf[..too - ind]
                                        .copy_from_slice(&LOREM_IPSUM.as_bytes()[ind..too]);
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
                            // Settings Getter
                            Command::GetSettings => {
                                if ind < 4 {
                                    let dpi = self.settings.dpi.to_le_bytes();
                                    let ckeys = self.settings.cpi_keys;
                                    let cpi_mods = self.settings.cpi_mods;
                                    let lkeys = self.settings.lift_keys;
                                    let lift_mods = self.settings.lift_mods;
                                    let bvolt = self.bat_voltage.to_le_bytes();
                                    Response::RawData(match ind {
                                        0 => [dpi[0], dpi[1], ckeys[0], ckeys[1], ckeys[2]],
                                        1 => [ckeys[3], ckeys[4], ckeys[5], cpi_mods, lkeys[0]],
                                        2 => [lkeys[1], lkeys[2], lkeys[3], lkeys[4], lkeys[5]],
                                        3 => [lift_mods, bvolt[0], bvolt[1], bvolt[2], bvolt[3]],
                                        _ => unreachable!(),
                                    })
                                } else {
                                    Response::Ok
                                }
                            }
                            // Sensor image sender
                            Command::StreamSensorImages(frames) => {
                                // If frames equals 1, then we quit the loop
                                if frames != 1 {
                                    // If frames equals 0, then we repeat infinitely
                                    if frames != 0 {
                                        [args[0], args[1]] = (frames - 1).to_le_bytes();
                                    }
                                    // Get new image
                                    if let Err(_) =
                                        self.sensor.get_frame(unsafe { IMAGE.borrow_mut() })
                                    {
                                        return Some(Response::Err(CommandError::SensorErr));
                                    }
                                    // Send image
                                    Response::ImageData(&IMAGE.borrow()[..])
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
            };
            Some(response)
        } else {
            None
        }
    }
}

mod mouse_error {
    use super::*;
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
}

mod mouse_settings {
    use super::*;
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
            ret[0] =
                u32::from_le_bytes([split_dpi[0], split_dpi[1], self.cpi_mods, self.lift_mods]);
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
    let mut scroll = 0;
    // Make sure there was a scroll
    if state != last_state {
        // Check change
        let changed = state ^ last_state;
        // Do logic
        if a == b {
            if changed == 0b10 {
                scroll = 1;
            } else {
                scroll = -1;
            }
        }
    }
    // Update Mouse
    interruptm::free(|cs| {
        if let Some(mouse) = &mut *MOUSE.borrow(cs).borrow_mut() {
            // Update scroll
            mouse.scroll += scroll;
            mouse.scroll_state = state;
            // Update lifted
            if mouse.lifted_status == LiftedStatus::Lifted {
                mouse.lifted_timer += 1;
            } else if mouse.lifted_timer > 0 && mouse.lifted_timer < LIFTED_TIMER_MAX {
                mouse.lifted_status = LiftedStatus::SendKeys;
                mouse.lifted_timer = 0;
            } else {
                mouse.lifted_timer = 0;
            }
        }
    });
    // Continue loop
    true
}
