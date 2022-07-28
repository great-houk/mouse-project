use core::marker::PhantomData;
use max32625_gpio::PinNum;
use max32625_gpio::Port;

use crate::spim::{Spim, SPIM};

#[derive(Debug, Clone, Copy)]
pub enum SpiError {
    SlaveAlreadyExists,
    InvalidSS,
    PinAlreadyTaken(Port, PinNum),
}

#[derive(Debug, Clone, Copy)]
pub enum SpiClockScale {
    Disabled = 0,
    Div1 = 1,
    Div2 = 2,
    Div4 = 3,
    Div8 = 4,
    Div16 = 5,
    Div32 = 6,
    Div64 = 7,
    Div128 = 8,
    Div256 = 9,
}

#[derive(Debug, Clone, Copy)]
pub enum SpiMode {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
    Mode3 = 3,
}

#[derive(Debug, Clone, Copy)]
pub enum SpiPageSize {
    Bytes4 = 0,
    Bytes8 = 1,
    Bytes16 = 2,
    Bytes32 = 3,
}

impl SpiPageSize {
    pub fn as_u16(&self) -> u16 {
        match self {
            SpiPageSize::Bytes4 => 4,
            SpiPageSize::Bytes8 => 8,
            SpiPageSize::Bytes16 => 16,
            SpiPageSize::Bytes32 => 32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpiSizeUnits {
    Bits = 0,
    Bytes = 1,
    Pages = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum SS {
    Slave0 = 0,
    Slave1 = 1,
    Slave2 = 2,
    Slave3 = 3,
    Slave4 = 4,
}

#[derive(Debug, Clone, Copy)]
pub enum Polarity {
    ActiveHigh = 1,
    ActiveLow = 0,
}

#[derive(Debug)]
pub struct SpiSlave<SPI: SPIM> {
    pub(crate) ss: SS,
    pub(crate) three_wire: bool,
    pub(crate) spi_mode: SpiMode,
    pub(crate) page_size: SpiPageSize,
    pub(crate) phantom: PhantomData<SPI>,
}

impl_spims! { SpiSlave<> SPI : mod _spi_slave
    pub fn push_start(&self, spi: &Spim<SPI>) {
        // Push settings
        const SETTINGS_HEADER: u16 = 0b01;
        let value = (SETTINGS_HEADER << 14) |
                    (1 << 12) |
                    (1 << 8) |
                    ((self.page_size as u16) << 6) |
                    ((self.spi_mode as u16) << 4) |
                    ((self.three_wire as u16) << 3) |
                    self.ss as u16;
        spi.push_trans(value);
    }

    pub fn push_end(&self, spi: &Spim<SPI>) {
        // Push settings
        const SETTINGS_HEADER: u16 = 0b01;
        let value = (SETTINGS_HEADER << 14) |
                    (1 << 12) |
                    (1 << 8) |
                    ((self.page_size as u16) << 6) |
                    ((self.spi_mode as u16) << 4) |
                    ((self.three_wire as u16) << 3) |
                    7 as u16;
        spi.push_trans(value);
        // Push None 1 bit, stop SS at the end
        spi.push_trans(0x2010);
    }

    pub unsafe fn set_ss(&self, spi: &Spim<SPI>, asserted: bool) {
        // Push false settings
        const SETTINGS_HEADER: u16 = 0b01;
        let value = (SETTINGS_HEADER << 14) |
                    (1 << 12) |
                    (1 << 8) |
                    ((self.page_size as u16) << 6) |
                    ((self.spi_mode as u16) << 4) |
                    ((self.three_wire as u16) << 3) |
                    7 as u16;
        spi.push_trans(value);
        // Set SS
        if asserted {
            // Push None 1 bit, leave SS at the end
            spi.push_trans(0x0010);
        } else {
            // Push None 1 bit, stop SS at the end
            spi.push_trans(0x2010);
        }
        // Push real settings
        self.push_start(spi);
    }
}

getter_setter_impl! { SpiSlave<SPI: SPIM> :
    pub three_wire, set_three_wire: bool,
    pub spi_mode, set_spi_mode: SpiMode,
    pub page_size, set_page_size: SpiPageSize,
}

#[must_use]
pub struct SpiTransaction<'a, SPI: SPIM> {
    pub(crate) spim: &'a Spim<SPI>,
    pub(crate) slave: &'a SpiSlave<SPI>,
    pub(crate) end: bool,
}

impl<'a, SPI: SPIM> SpiTransaction<'a, SPI> {
    pub(crate) fn gen_header(dir: u16, size_unit: SpiSizeUnits, size: u16, de_ss: bool) -> u16 {
        ((de_ss as u16) << 13) | ((size as u16) << 4) | ((size_unit as u16) << 2) | (dir as u16)
    }

    pub unsafe fn set_end(&mut self, set: bool) {
        self.end = set
    }

    fn should_use_pages(size: u16) -> bool {
        if size > 32 {
            true
        } else {
            false
        }
    }

    fn calc_num_pages(&self, size: u16) -> u16 {
        let num_pages = (size / self.slave.page_size.as_u16()).min(32);
        if num_pages == 32 {
            0
        } else {
            num_pages
        }
    }
}

impl_spims! { SpiTransaction<'a> _SPI : mod spi_trans
    pub fn none(&self, size: u16) -> &Self {
        const NONE_DIR: u16 = 0b00;
        let mut sent = 0;
        // Send pages
        while Self::should_use_pages(size - sent) {
            let pages = self.calc_num_pages(size - sent);
            let bytes = if pages == 0 { 32 } else { pages } * self.slave.page_size.as_u16();
            let end = if (sent + bytes) == size { self.end } else { false };
            let header = Self::gen_header(NONE_DIR, SpiSizeUnits::Pages, pages, end);
            self.spim.push_trans(header);
            sent += bytes;
        }
        // Send bytes
        if size - sent > 0 {
            let mut bytes = size - sent;
            if bytes == 32 {
                bytes = 0;
            }
            let header = Self::gen_header(NONE_DIR, SpiSizeUnits::Bytes, bytes, self.end);
            self.spim.push_trans(header);
        }
        self
    }

    pub fn tx(&self, data: &[u8]) -> &Self {
        const TX_DIR: u16 = 0b01;
        // Send tx header with size of data
        let size = data.len() as u16;
        let mut sent = 0;
        // Send pages
        while Self::should_use_pages(size - sent) {
            let pages = self.calc_num_pages(size - sent);
            let bytes = if pages == 0 { 32 } else { pages } * self.slave.page_size.as_u16();
            // Push header
            let end = if (sent + bytes) == size { self.end } else { false };
            let header = Self::gen_header(TX_DIR, SpiSizeUnits::Pages, pages, end);
            self.spim.push_trans(header);
            // Push data
            let iter = data[sent as usize..(sent + bytes) as usize].iter().cloned();
            for d in Self::iter_u16(iter) {
                self.spim.push_trans(d);
            }
            sent += bytes;
        }
        // Send bytes
        if sent < size {
            let bytes = if (size - sent) == 32 { 0 } else { size - sent };
            // Push header
            let header = Self::gen_header(TX_DIR, SpiSizeUnits::Bytes, bytes, self.end);
            self.spim.push_trans(header);
            // Push data
            let iter = data[sent as usize..(sent + bytes) as usize].iter().cloned();
            for d in Self::iter_u16(iter) {
                self.spim.push_trans(d);
            }
        }
        self
    }

    pub fn rx(&self, buf: &mut [u8]) -> &Self {
        const RX_DIR: u16 = 0b10;
        // Send rx header with size of buf
        let size = buf.len() as u16;
        let mut sent = 0;
        // Send pages
        while Self::should_use_pages(size - sent) {
            let pages = self.calc_num_pages(size - sent);
            let bytes = if pages == 0 { 32 } else { pages } * self.slave.page_size.as_u16();
            // Push header
            let end = if (sent + bytes) == size { self.end } else { false };
            let header = Self::gen_header(RX_DIR, SpiSizeUnits::Pages, pages, end);
            self.spim.push_trans(header);
            // Get data
            for i in sent..sent + bytes {
                // Wait for a byte
                while self.spim.spi_raw.fifo_ctrl.read().rx_fifo_used().bits() == 0 {}
                // Copy byte in
                buf[i as usize] = self.spim.pull_rslts().unwrap();
            }
            sent += bytes;
        }
        // Send bytes
        if sent < size {
            let bytes = if (size - sent) == 32 { 0 } else { size - sent };
            // Push header
            let header = Self::gen_header(RX_DIR, SpiSizeUnits::Bytes, bytes, self.end);
            self.spim.push_trans(header);
            // Get data
            for i in sent..sent + bytes {
               // Wait for a byte
               while self.spim.spi_raw.fifo_ctrl.read().rx_fifo_used().bits() <= 0 {}
               // Copy byte in
               buf[i as usize] = self.spim.pull_rslts().unwrap();
            }
        }
        self
    }

    pub fn both<const SIZE: usize>(&self, data: &[u8; SIZE], buf: &mut [u8; SIZE]) -> &Self {
        const BOTH_DIR: u16 = 0b11;
        // Send both header with size of buf
        let size = SIZE as u16;
        let mut sent = 0;
        // Send pages
        while Self::should_use_pages(size - sent) {
            let pages = self.calc_num_pages(size - sent);
            let bytes = if pages == 0 { 32 } else { pages } * self.slave.page_size.as_u16();
            // Push header
            let end = if (sent + bytes) == size { self.end } else { false };
            let header = Self::gen_header(BOTH_DIR, SpiSizeUnits::Pages, pages, end);
            self.spim.push_trans(header);
            // Push data
            let iter = data[sent as usize..(sent + bytes) as usize].iter().cloned();
            for d in Self::iter_u16(iter) {
                self.spim.push_trans(d);
            }
            // Get data
            for i in sent..sent + bytes {
                // Wait for a byte
                while self.spim.spi_raw.fifo_ctrl.read().rx_fifo_used().bits() <= 0 {}
                // Copy byte in
                buf[i as usize] = self.spim.pull_rslts().unwrap();
            }
            sent += bytes;
        }
        // Send bytes
        if sent < size {
            let bytes = if (size - sent) == 32 { 0 } else { size - sent };
            // Push header
            let header = Self::gen_header(BOTH_DIR, SpiSizeUnits::Bytes, bytes, self.end);
            self.spim.push_trans(header);
            // Push data
            let iter = data[sent as usize..(sent + bytes) as usize].iter().cloned();
            for d in Self::iter_u16(iter) {
                self.spim.push_trans(d);
            }
            // Get data
            for i in sent..sent + bytes {
               // Wait for a byte
               while self.spim.spi_raw.fifo_ctrl.read().rx_fifo_used().bits() == 0 {}
               // Copy byte in
               buf[i as usize] = self.spim.pull_rslts().unwrap();
            }
        }
        self
    }

    fn iter_u16<I: Iterator<Item = u8>>(iter: I) -> impl Iterator<Item = u16> {
        struct DoubleIter<I: Iterator<Item = u8>> { iter: I }

        impl<I: Iterator<Item = u8>> Iterator for DoubleIter<I> {
            type Item = u16;

            fn next(&mut self) -> Option<Self::Item> {
                match self.iter.next() {
                    Some(lsb) => {
                        match self.iter.next() {
                            Some(msb) => Some(((msb as u16) << 8) | lsb as u16),
                            None => Some(lsb as u16),
                        }
                    }
                    None => None,
                }
            }
        }

        DoubleIter { iter }
    }
}
