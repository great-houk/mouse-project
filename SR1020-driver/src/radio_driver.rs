use bare_metal::CriticalSection;

pub trait RadioDriver: Sized {
    type Error;

    /// SPI transfer in full duplex mode (both recieving and sending), block until finished.
    fn spi_duplex_blocking<const SIZE: usize>(
        &mut self,
        data: &[u8; SIZE],
        buffer: &mut [u8; SIZE],
    ) -> Result<(), Self::Error>;

    /// Sets the reset pin either high or low
    fn set_reset_pin(&mut self, high: bool) -> Result<(), Self::Error>;

    /// Sets the shutdown pin either high or low
    fn set_shutdown_pin(&mut self, high: bool) -> Result<(), Self::Error>;

    /// Reads the state of the IRQ pin of the chip
    fn read_irq(&mut self) -> Result<bool, Self::Error>;

    /// Delays at least ms milliseconds
    fn delay_ms(&mut self, ms: u32) -> Result<(), Self::Error>;

    /// Triggers and goes into the interrupt
    fn context_switch(&mut self) -> Result<(), Self::Error>;

    /// Enable/Disable SR1020 Interrupt
    fn enable_interrupt(&mut self, enabled: bool) -> Result<(), Self::Error>;

    /// Interrupt free section
    fn critical_section<R>(f: impl FnOnce(&CriticalSection) -> R) -> R;
}
