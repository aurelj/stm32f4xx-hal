use core::ops::Deref;

use crate::stm32::i2c1;

use crate::stm32::{I2C1, I2C2, RCC};

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

use crate::gpio::gpioa::{PA9, PA10, PA11, PA12};
use crate::gpio::gpiob::{PB6, PB7, PB8, PB9, PB10, PB11, PB13, PB14};
use crate::gpio::gpiof::{PF0, PF1};
use crate::gpio::{Alternate, AF1, AF4, AF5};

use crate::rcc::Clocks;
use crate::time::{KiloHertz, U32Ext};

/// I2C abstraction
pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

pub trait Pins<I2c> {}
pub trait PinScl<I2c> {}
pub trait PinSda<I2c> {}

impl<I2c, SCL, SDA> Pins<I2c> for (SCL, SDA)
where
    SCL: PinScl<I2c>,
    SDA: PinSda<I2c>,
{}

impl PinScl<I2C1> for PA9<Alternate<AF4>> {}
impl PinSda<I2C1> for PA10<Alternate<AF4>> {}
impl PinScl<I2C1> for PA11<Alternate<AF5>> {}
impl PinSda<I2C1> for PA12<Alternate<AF5>> {}
impl PinScl<I2C1> for PB6<Alternate<AF1>> {}
impl PinSda<I2C1> for PB7<Alternate<AF1>> {}
impl PinScl<I2C1> for PB8<Alternate<AF1>> {}
impl PinSda<I2C1> for PB9<Alternate<AF1>> {}
impl PinScl<I2C1> for PB10<Alternate<AF1>> {}
impl PinSda<I2C1> for PB11<Alternate<AF1>> {}
impl PinScl<I2C1> for PB13<Alternate<AF5>> {}
impl PinSda<I2C1> for PB14<Alternate<AF5>> {}
impl PinScl<I2C1> for PF1<Alternate<AF1>> {}
impl PinSda<I2C1> for PF0<Alternate<AF1>> {}

impl PinScl<I2C2> for PA11<Alternate<AF5>> {}
impl PinSda<I2C2> for PA12<Alternate<AF5>> {}
impl PinScl<I2C2> for PB10<Alternate<AF1>> {}
impl PinSda<I2C2> for PB11<Alternate<AF1>> {}
impl PinScl<I2C2> for PB13<Alternate<AF5>> {}
impl PinSda<I2C2> for PB14<Alternate<AF5>> {}

#[derive(Debug)]
pub enum Error {
    OVERRUN,
    NACK,
}

impl<PINS> I2c<I2C1, PINS> {
    pub fn i2c1(i2c: I2C1, pins: PINS, speed: KiloHertz, _clocks: Clocks) -> Self
    where
        PINS: Pins<I2C1>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for I2C1
        rcc.apb1enr.modify(|_, w| w.i2c1en().set_bit());

        // Reset I2C1
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c1rst().clear_bit());

        I2c { i2c, pins }.i2c_init(speed)
    }
}

impl<PINS> I2c<I2C2, PINS> {
    pub fn i2c2(i2c: I2C2, pins: PINS, speed: KiloHertz, _clocks: Clocks) -> Self
    where
        PINS: Pins<I2C2>,
    {
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };

        // Enable clock for I2C2
        rcc.apb1enr.modify(|_, w| w.i2c2en().set_bit());

        // Reset I2C2
        rcc.apb1rstr.modify(|_, w| w.i2c2rst().set_bit());
        rcc.apb1rstr.modify(|_, w| w.i2c2rst().clear_bit());

        I2c { i2c, pins }.i2c_init(speed)
    }
}

impl<I2C, PINS> I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    fn i2c_init(self, speed: KiloHertz) -> Self {
        use core::cmp;

        // Make sure the I2C unit is disabled so we can configure it
        self.i2c.cr1.modify(|_, w| w.pe().clear_bit());

        // Calculate settings for I2C speed modes
        let presc;
        let scldel;
        let sdadel;
        let sclh;
        let scll;

        // We're using HSI here which runs at a fixed 8MHz
        const FREQ: u32 = 8_000_000;

        // Normal I2C speeds use a different scaling than fast mode below
        if speed <= 100_u32.khz() {
            presc = 1;
            scll = cmp::max((((FREQ >> presc) >> 1) / speed.0) - 1, 255) as u8;
            sclh = scll - 4;
            sdadel = 2;
            scldel = 4;
        } else {
            presc = 0;
            scll = cmp::max((((FREQ >> presc) >> 1) / speed.0) - 1, 255) as u8;
            sclh = scll - 6;
            sdadel = 1;
            scldel = 3;
        }

        // Enable I2C signal generator, and configure I2C for 400KHz full speed
        self.i2c.timingr.write(|w| {
            w.presc()
                .bits(presc)
                .scldel()
                .bits(scldel)
                .sdadel()
                .bits(sdadel)
                .sclh()
                .bits(sclh)
                .scll()
                .bits(scll)
        });

        // Enable the I2C processing
        self.i2c.cr1.modify(|_, w| w.pe().set_bit());

        self
    }

    pub fn release(self) -> (I2C, PINS) {
        (self.i2c, self.pins)
    }
}

trait I2cCommon {
    fn send_byte(&self, byte: u8) -> Result<(), Error>;

    fn recv_byte(&self) -> Result<u8, Error>;
}

impl<I2C, PINS> I2cCommon for I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        // Wait until we're ready for sending
        while self.i2c.isr.read().txis().bit_is_clear() {}

        // Push out a byte of data
        self.i2c.txdr.write(|w| unsafe { w.bits(u32::from(byte)) });

        // Wait until byte is transferred
        while {
            let isr = self.i2c.isr.read();

            // If we received a NACK, then this is an error
            if isr.nackf().bit_is_set() {
                self.i2c.icr.write(|w| w.stopcf().set_bit().nackcf().set_bit());
                return Err(Error::NACK);
            }

            isr.tc().bit_is_clear()
        } {}

        Ok(())
    }

    fn recv_byte(&self) -> Result<u8, Error> {
        while self.i2c.isr.read().rxne().bit_is_clear() {}
        let value = self.i2c.rxdr.read().bits() as u8;
        Ok(value)
    }
}

impl<I2C, PINS> WriteRead for I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.write(addr, bytes)?;
        self.read(addr, buffer)?;

        Ok(())
    }
}

impl<I2C, PINS> Write for I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        // Set up current address, we're trying a "read" command and not going to set anything
        // and make sure we end a non-NACKed read (i.e. if we found a device) properly
        self.i2c.cr2.modify(|_, w| { w
            .sadd()
            .bits(u16::from(addr) << 1)
            .nbytes()
            .bits(bytes.len() as u8)
            .rd_wrn()
            .clear_bit()
            .autoend()
            .set_bit()
        });

        // Send a START condition
        self.i2c.cr2.modify(|_, w| w.start().set_bit());

        // Send bytes
        for c in bytes {
            self.send_byte(*c)?;
        }

        // Clear flags if they somehow ended up set
        self.i2c.icr.write(|w| w.stopcf().set_bit().nackcf().set_bit());

        // Fallthrough is success
        Ok(())
    }
}

impl<I2C, PINS> Read for I2c<I2C, PINS>
where
    I2C: Deref<Target = i2c1::RegisterBlock>,
{
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Set up current address, we're trying a "read" command and not going to set anything
        // and make sure we end a non-NACKed read (i.e. if we found a device) properly
        self.i2c.cr2.modify(|_, w| { w
            .sadd()
            .bits(u16::from(addr) << 1)
            .nbytes()
            .bits(buffer.len() as u8)
            .rd_wrn()
            .set_bit()
        });

        // Send a START condition
        self.i2c.cr2.modify(|_, w| w.start().set_bit());

        // Send the autoend after setting the start to get a restart
        self.i2c.cr2.modify(|_, w| w.autoend().set_bit());

        // Receive bytes into buffer
        for c in buffer {
            *c = self.recv_byte()?;
        }

        // Clear flags if they somehow ended up set
        self.i2c.icr.write(|w| w.stopcf().set_bit().nackcf().set_bit());

        // Fallthrough is success
        Ok(())
    }
}
