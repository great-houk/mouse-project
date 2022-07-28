use crate::{regs::RegisterSize, Pmw3389Driver, Pmw3389Error, Pmw3389Register};

pub struct Pmw3389<DRIVER: Pmw3389Driver> {
    driver: DRIVER,
    is_setup: bool,
}

impl<DRIVER: Pmw3389Driver> Pmw3389<DRIVER> {
    pub fn init(driver: DRIVER) -> Result<Self, Pmw3389Error> {
        /*
            Power on steps
            1. Apply power
            2. Drive NCS high, and then low to reset SPI bus
            3. Write 0x5A to PowerUpRegister (or toggle nreset pin)
            4. Wait for at least 50ms
            5. Read from registers 0x02, 0x03, 0x04, 0x05, and 0x06
               one time regardless of motion pin state
            6. Perform SROM download
            7. Write to register 0x3D with value 0x80
            8. Read from register 0x3D at 1ms polling interval until
               value 0xC0 is obtained or read up to 55ms. The read interval
               must be 1ms with a tolerance of +/- 1% (I don't fully think that's true)
            9. Write to register 0x3D with a value of 0x00
            10. Load configuration for other registers
        */
        let mut this = Self {
            driver,
            is_setup: false,
        };
        // Step 1: Already Done
        // Step 2:
        this.driver.set_cs(false);
        this.driver.set_cs(true);
        this.driver.set_cs(false);
        // Step 3:
        this.write_reg(Register::PowerUpReset, 0x5A, false)?;
        // Step 4:
        this.driver.delay_ms(50)?;
        // Step 5:
        this.read_reg(0x02, false)?;
        this.read_reg(0x03, false)?;
        this.read_reg(0x04, false)?;
        this.read_reg(0x05, false)?;
        this.read_reg(0x06, false)?;
        // Step 6:
        this.srom_download()?;
        // Step 7:
        this.write_reg(0x3D, 0x80, false)?;
        // Step 8:
        for _ in 0..55 {
            if this.read_reg(0x3D, false)? == 0xC0 {
                break;
            }
            this.driver.delay_ms(1)?;
        }
        // Step 9:
        this.write_reg(0x3D, 0x00, false)?;
        // Step 10: Nothing yet
        Ok(this)
    }

    pub fn get_frame(&mut self, buf: &mut [u8]) -> Result<(), Pmw3389Error> {
        /*
        1. The chip should be powered up and reset correctly (SROM download should be part of this powered up and reset
            sequence - Refer to 6.1 Power-Up).
        2. Wait for 250ms.
        3. Write 0x00 to register 0x10
        4. Write 0x83 to Frame_Capture register.
        5. Write 0xC5 to Frame_Capture register.
        6. Wait for 20ms.
        7. Continue burst read from RawData_Burst register until all 1296 RawDatas are transferred.
        8. Continue step 3 to 7 to capture another frame.
        9. Write 0x20 to register 0x10.
        */
        // Steps 1-2 should be done already
        // Step 3.
        self.write_reg(Register::Config2, 0x00, true)?;
        // Step 4.
        self.write_reg(Register::FrameCapture, 0x83, true)?;
        // Step 5.
        self.write_reg(Register::FrameCapture, 0xC5, true)?;
        // Step 6.
        self.driver.delay_ms(5)?;
        // Step 7.
        self.driver
            .spi_write(&[Register::RawDataBurst as u8], false)?;
        self.driver.delay_us(35)?;
        for i in 0..36 * 36 {
            self.driver.spi_read(&mut buf[i..=i], false)?;
            self.driver.delay_us(20)?;
        }
        self.driver.set_cs(false);
        // Step 8 and on, we don't do, since we don't know how many we're gonna read
        self.driver.delay_us(100)?;
        Ok(())
    }

    pub fn reset_sensor(&mut self) -> Result<(), Pmw3389Error> {
        self.write_reg(Register::Config2, 0x20, true)?;
        self.write_reg(Register::PowerUpReset, 0x5A, true)?;
        self.srom_download()
    }

    pub fn read_motion_regs(&self) -> Result<MotionReport, Pmw3389Error> {
        if !self.is_setup {
            return Err(Pmw3389Error::NotInitialized);
        }
        /*
            Steps to read from motion reg
            1. Write any value to motion register
            2. Lower NCS
            3. Send motion_burst address
            4. Wait Tsrad_motbr (35us)
            5. Start reading continuously for 12 bytes
        */
        // Step 1:
        self.write_reg(Register::Motion, 0xFF, true)?;
        // Step 2: Done
        // Step 3:
        self.driver
            .spi_write(&[Register::MotionBurst as u8], false)?;
        // Step 4:
        self.driver.delay_us(35)?;
        // Step 5:
        let mut buf = [0; 12];
        self.driver.spi_read(&mut buf, true)?;
        Ok(buf.into())
    }

    pub fn read(&self, register: Pmw3389Register) -> Result<u16, Pmw3389Error> {
        if !self.is_setup {
            return Err(Pmw3389Error::NotInitialized);
        }
        // Read either 1 or two registers, depending on if the value is 16 or 8 bit
        let data = match register.to_register_read()? {
            RegisterSize::Single(reg) => {
                // Read register
                let val = self.read_reg(reg, true)?;
                // Return data
                val as u16
            }
            RegisterSize::DoubleHighFirst(regl, regh) => {
                // Read upper register
                let upper = self.read_reg(regh, false)?;
                // Read lower
                let lower = self.read_reg(regl, true)?;
                // Return data
                u16::from_le_bytes([lower, upper])
            }
            RegisterSize::DoubleLowFirst(regl, regh) => {
                // Read lower
                let lower = self.read_reg(regl, true)?;
                // Read upper register
                let upper = self.read_reg(regh, false)?;
                // Return data
                u16::from_le_bytes([lower, upper])
            }
        };
        // Return read data
        Ok(data)
    }

    pub fn write(&self, value: u16, register: Pmw3389Register) -> Result<(), Pmw3389Error> {
        if !self.is_setup {
            return Err(Pmw3389Error::NotInitialized);
        }
        // Write either 1 or two registers, depending on if the value is 16 or 8 bit
        match register.to_register_write()? {
            RegisterSize::Single(reg) => {
                // Write register
                self.write_reg(reg, value as u8, true)?;
            }
            RegisterSize::DoubleHighFirst(regl, regh) => {
                // Split val
                let [vall, valh] = value.to_le_bytes();
                // Write upper register
                self.write_reg(regh, valh, false)?;
                // Write lower
                self.write_reg(regl, vall, true)?;
            }
            RegisterSize::DoubleLowFirst(regl, regh) => {
                // Split val
                let [vall, valh] = value.to_le_bytes();
                // Write lower
                self.write_reg(regl, vall, true)?;
                // Write upper register
                self.write_reg(regh, valh, false)?;
            }
        }
        Ok(())
    }

    fn read_reg(&self, register: impl Into<u8>, end: bool) -> Result<u8, Pmw3389Error> {
        // Send adress of the register, with MSBit = 0 to indicate it's a read
        self.driver.spi_write(&[register.into() & 0x7f], false)?;
        self.driver.delay_us(160)?; // tSRAD

        // Read data
        let mut data = [0];
        self.driver.spi_read(&mut data, false)?;

        self.driver.delay_us(1)?; // tSCLK-NCS for read operation is 120ns
        self.driver.set_cs(end);
        self.driver.delay_us(19)?; //  tSRR/tsRW (=20us) minus tSCLK-NCS

        Ok(data[0])
    }

    fn write_reg(&self, register: impl Into<u8>, data: u8, end: bool) -> Result<(), Pmw3389Error> {
        //send adress of the register, with MSBit = 1 to indicate it's a write
        self.driver.spi_write(&[register.into() | 0x80], false)?;
        //sent data
        self.driver.spi_write(&[data], false)?;

        self.driver.delay_us(35)?; // tSCLK-NCS for write operation
        self.driver.set_cs(end);
        self.driver.delay_us(145)?; // tSWW/tSWR (=180us) minus tSCLK-NCS
        Ok(())
    }

    fn srom_download(&mut self) -> Result<(), Pmw3389Error> {
        /*
            SROM Download Steps
            1. Perform Steps 1 through 5 of the power up
            2. Write 0x00 to register 0x10 (Config2)
            3. Write 0x1D to SROMEnable Register to initialize
            4. Wait for 10ms
            5. Write 0x18 to SROMEnable register again to start download
            6. Write SROM file into SROMLoadBurst register, use burst mode
            7. Read the SromId register to verify the ID before any other
               register reads or writes
        */
        // Step 1: Assumed done
        // Step 2:
        self.write_reg(Register::Config2, 0x20, false)?;
        // Step 3:
        self.write_reg(Register::SromEnable, 0x1D, false)?;
        // Step 4:
        self.driver.delay_ms(10)?;
        // Step 5:
        self.write_reg(Register::SromEnable, 0x18, false)?;
        // Step 6:
        // Send adress of the register, with MSBit = 0 to indicate it's a read
        self.driver
            .spi_write(&[0x80 | Register::SromLoadBurst as u8], false)?;
        self.driver.delay_us(15)?; // >= 15us Delay
        const BYTES: &[u8; 4094] = include_bytes!("../rom.hex");
        for &byte in BYTES {
            // Write all bytes, with 15us delay inbetween
            self.driver.spi_write(&[byte], false)?;
            self.driver.delay_us(15)?;
        }
        self.driver.set_cs(false);
        // Delay recomended amount (200us) - last read delay (15us) = 185us
        self.driver.delay_us(180)?;
        // Step 7:
        let id = self.read_reg(Register::SromId, false)?;
        if id == 0x00 {
            return Err(Pmw3389Error::FailedUpload);
        }
        // Make this setup
        self.is_setup = true;
        Ok(())
    }
}

/*
0x00 ProductID R
0x01 RevisionID R
0x02 Motion RW
0x03 DeltaXL R
0x04 DeltaXH R
0x05 DeltaYL R
0x06 DeltaYH R
0x07 SQUAL R
0x08 RawDataSum R
0x09 MaximumRawData R
0x0A MinimumRawData R
0x0B ShutterLower R
0x0C ShutterUpper R
0x0D RippleControl RW
0x0E ResolutionL RW
0x0F ResolutionH RW
0x10 Config2 RW
0x11 AngleTune RW
0x12 FrameCapture RW
0x13 SROMEnable W
0x14 RunDownshift RW
0x15 Rest1RateLower RW
0x16 Rest1RateUpper RW
0x17 Rest1Downshift RW
0x18 Rest2RateLower RW
0x19 Rest2RateUpper RW
0x1A Rest2Downshift RW
0x1B Rest3RateLower RW
0x1C Rest3RateUpper RW
0x24 Observation RW
0x25 DataOutLower R
0x26 DataOutUpper R
0x2A SROMID R
0x2B MinSQRun RW
0x2C RawDataThreshold RW
0x2D Control2 RW
0x2E Config5L RW
0x2F Config5H RW
0X3A PowerUpReset W
0x3B Shutdown W
0x3F InverseProductID R
0x41 LiftCutoffCal3 RW
0x42 AngleSnap RW
0x4A LiftCutoffCal1 RW
0x50 MotionBurst RW
0x62 SROMLoadBurst W
0x63 LiftConfig RW
0x64 RawDataBurst R
0x65 LiftCutoffCal2 R
0x71 LiftCutoffCalTimeout RW
0x72 LiftCutoffCalMinLength RW
0x73 PWMPeriodCnt RW
0x74 PWMWidthCnt RW
*/

#[allow(unused)]
pub(crate) enum Register {
    ProductId = 0x00,
    RevisionId = 0x01,
    Motion = 0x02,
    DeltaXL = 0x03,
    DeltaXH = 0x04,
    DeltaYL = 0x05,
    DeltaYH = 0x06,
    Squal = 0x07,
    RawDataSum = 0x08,
    MaximumRawData = 0x09,
    MinimumRawData = 0x0A,
    ShutterLower = 0x0B,
    ShutterUpper = 0x0C,
    RippleControl = 0x0D,
    ResolutionL = 0x0E,
    ResolutionH = 0x0F,
    Config2 = 0x10,
    AngleTune = 0x11,
    FrameCapture = 0x12,
    SromEnable = 0x13,
    RunDownshift = 0x14,
    Rest1RateLower = 0x15,
    Rest1RateUpper = 0x16,
    Rest1Downshift = 0x17,
    Rest2RateLower = 0x18,
    Rest2RateUpper = 0x19,
    Rest2Downshift = 0x1A,
    Rest3RateLower = 0x1B,
    Rest3RateUpper = 0x1C,
    Observation = 0x24,
    DataOutLower = 0x25,
    DataOutUpper = 0x26,
    SromId = 0x2A,
    MinSQRun = 0x2B,
    RawDataThreshold = 0x2C,
    Control2 = 0x2D,
    Config5L = 0x2E,
    Config5H = 0x2F,
    PowerUpReset = 0x3A,
    Shutdown = 0x3B,
    InverseProductID = 0x3F,
    LiftCutoffCal3 = 0x41,
    AngleSnap = 0x42,
    LiftCutoffCal1 = 0x4A,
    MotionBurst = 0x50,
    SromLoadBurst = 0x62,
    LiftConfig = 0x63,
    RawDataBurst = 0x64,
    LiftCutoffCal2 = 0x65,
    LiftCutoffCalTimeout = 0x71,
    LiftCutoffCalMinLength = 0x72,
    PWMPeriodCnt = 0x73,
    PWMWidthCnt = 0x74,
}

impl Into<u8> for Register {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct MotionReport {
    motion: Motion,
    srom_is_running: bool,
    delta_x: i16,
    delta_y: i16,
    squal: u8,
    rawdata_sum: u8,
    max_rawdata: u8,
    min_rawdata: u8,
    shutter: u16,
}

impl MotionReport {
    pub fn motion(&self) -> Motion {
        self.motion
    }

    pub fn srom_is_running(&self) -> bool {
        self.srom_is_running
    }

    pub fn delta_x(&self) -> i16 {
        self.delta_x
    }

    pub fn delta_y(&self) -> i16 {
        self.delta_y
    }

    pub fn squal(&self) -> u8 {
        self.squal
    }

    pub fn rawdata_sum(&self) -> u8 {
        self.rawdata_sum
    }

    pub fn max_rawdata(&self) -> u8 {
        self.max_rawdata
    }

    pub fn min_rawdata(&self) -> u8 {
        self.min_rawdata
    }

    pub fn shutter(&self) -> u16 {
        self.shutter
    }
}

impl From<[u8; 12]> for MotionReport {
    fn from(bytes: [u8; 12]) -> Self {
        MotionReport {
            motion: Motion { val: bytes[0] },
            srom_is_running: bytes[1] & 0b0100_0000 != 0,
            delta_x: i16::from_le_bytes([bytes[2], bytes[3]]),
            delta_y: i16::from_le_bytes([bytes[4], bytes[5]]),
            squal: bytes[6],
            rawdata_sum: bytes[7],
            max_rawdata: bytes[8],
            min_rawdata: bytes[9],
            shutter: u16::from_le_bytes([bytes[10], bytes[11]]),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Motion {
    val: u8,
}

impl Motion {
    // Whether or not the data going to be read from RawData is from the first pixel
    pub fn pix_first(&self) -> bool {
        self.val & 0b0001_0000 != 0
    }
    // Whether or not the mouse has moved
    pub fn motion(&self) -> bool {
        self.val & 0b1000_0000 != 0
    }
    // Whether or not the mouse is lifted
    pub fn lifted(&self) -> bool {
        self.val & 0b0000_1000 != 0
    }
    // Whether or not the data going to be read from RawData is from the first frame
    pub fn frame_first(&self) -> bool {
        self.val & 0b0000_0001 != 0
    }
}
