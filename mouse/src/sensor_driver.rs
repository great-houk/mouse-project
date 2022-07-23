use max32625::{CLKMAN, IOMAN, SPIM1};
use max32625_gpio::{PinNum, PinOutMode, PinVoltageSupply, Port, GPIO};
use max32625_spi::{Polarity, SpiClockScale, SpiMode, Spim, SS};
use max32625_timer_basic::Timer;
use pmw3389_driver::{Pmw3389Driver as Driver, Pmw3389Error};

#[derive(Debug)]
pub struct Pmw3389Driver<TIMER: Timer> {
    spi: Spim<SPIM1>,
    timer: TIMER,
}

impl<TIMER: Timer> Pmw3389Driver<TIMER> {
    pub fn new(spi: SPIM1, timer: TIMER, gpio: &mut GPIO, ioman: &IOMAN, clkman: &CLKMAN) -> Self {
        let mut sensor_rstn = gpio.take(Port::Port1, PinNum::Pin5).unwrap();
        sensor_rstn.set_supply(PinVoltageSupply::VDDIOH);
        {
            let mut sensor_clock = gpio.take(Port::Port1, PinNum::Pin0).unwrap();
            sensor_clock.set_supply(PinVoltageSupply::VDDIOH);
            gpio.put_back(sensor_clock);

            let mut sensor_mosi = gpio.take(Port::Port1, PinNum::Pin1).unwrap();
            sensor_mosi.set_supply(PinVoltageSupply::VDDIOH);
            gpio.put_back(sensor_mosi);

            let mut sensor_cs = gpio.take(Port::Port1, PinNum::Pin3).unwrap();
            sensor_cs.set_supply(PinVoltageSupply::VDDIOH);
            gpio.put_back(sensor_cs);
        }

        sensor_rstn.set_output(PinOutMode::OpenDriveHigh).unwrap();

        let mut spi = Spim::new(spi)
            .init(gpio, ioman, SpiClockScale::Div32, clkman)
            .unwrap();
        spi.add_slave(
            gpio,
            ioman,
            SS::Slave0,
            Polarity::ActiveLow,
            Polarity::ActiveLow,
        )
        .unwrap();
        spi.get_slave_mut(SS::Slave0)
            .unwrap()
            .set_spi_mode(SpiMode::Mode3);

        Self { spi, timer }
    }
}

impl<TIMER: Timer> Driver for Pmw3389Driver<TIMER> {
    fn set_cs(&self, asserted: bool) {
        unsafe {
            self.spi
                .get_slave(SS::Slave0)
                .unwrap()
                .set_ss(&self.spi, asserted)
        }
    }

    fn spi_read(&self, buf: &mut [u8], end: bool) -> Result<(), pmw3389_driver::Pmw3389Error> {
        unsafe {
            let mut trans = self.spi.unbounded_transaction(SS::Slave0).unwrap();
            trans.set_end(end);
            trans.rx(buf);
        };
        Ok(())
    }

    fn spi_write(&self, data: &[u8], end: bool) -> Result<(), pmw3389_driver::Pmw3389Error> {
        unsafe {
            let mut trans = self.spi.unbounded_transaction(SS::Slave0).unwrap();
            trans.set_end(end);
            trans.tx(data);
        };
        Ok(())
    }

    fn delay_us(&self, us: u32) -> Result<(), pmw3389_driver::Pmw3389Error> {
        self.timer
            .delay_us(us)
            .map_err(|_| Pmw3389Error::TimerError("Invalid Delay"))
    }

    fn delay_ms(&self, ms: u32) -> Result<(), pmw3389_driver::Pmw3389Error> {
        self.timer
            .delay_ms(ms)
            .map_err(|_| Pmw3389Error::TimerError("Invalid Delay"))
    }
}
