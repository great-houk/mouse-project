#[derive(Debug, PartialEq, PartialOrd)]
pub enum Pmw3389Error {
    /// String contains the reason
    SpiError(&'static str),
    /// String contains the reason
    TimerError(&'static str),
    FailedUpload,
    CantRead,
    CantWrite,
    BadInput,
    BufOverflow,
    NotInitialized,
    CalTimeout,
}

pub trait Pmw3389Driver {
    /// Sets the CS pin for the 3389 to either asserted (low) or not (high)
    fn set_cs(&self, asserted: bool);
    /// Reads the number of bytes necessary to fill buf from the SPI bus.
    /// This function should automatically set CS, and only unset it if end is true.
    fn spi_read(&self, buf: &mut [u8], end: bool) -> Result<(), Pmw3389Error>;
    /// Writes the number of bytes in data to the SPI bus.
    /// This function should automatically set CS, and only unset it if end is true.
    fn spi_write(&self, data: &[u8], end: bool) -> Result<(), Pmw3389Error>;
    /// Delays us microseconds
    fn delay_us(&self, us: u32) -> Result<(), Pmw3389Error>;
    /// Delays ms milliseconds
    fn delay_ms(&self, ms: u32) -> Result<(), Pmw3389Error>;
}
