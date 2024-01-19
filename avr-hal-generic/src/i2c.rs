//! I2C Implementations
//!
//! Check the documentation of [`I2c`] for details.

use embedded_hal::i2c::SevenBitAddress;

use crate::port;
use core::marker::PhantomData;

/// TWI Status Codes
pub mod twi_status {
    // The status codes defined in the C header are meant to be used with the
    // masked status value: (TWSR & TW_STATUS_MASK).  In our case, svd2rust
    // already added code to shift it to just the status value, so all status
    // codes need to be shifted to the right as well.

    /// Start condition transmitted
    pub const TW_START: u8 = 0x08 >> 3;

    /// Repeated start condition transmitted
    pub const TW_REP_START: u8 = 0x10 >> 3;

    // Master Transmitter -----------------------------------------------------
    /// SLA+W transmitted, ACK received
    pub const TW_MT_SLA_ACK: u8 = 0x18 >> 3;

    /// SLA+W transmitted, NACK received
    pub const TW_MT_SLA_NACK: u8 = 0x20 >> 3;

    /// Data transmitted, ACK received
    pub const TW_MT_DATA_ACK: u8 = 0x28 >> 3;

    /// Data transmitted, NACK received
    pub const TW_MT_DATA_NACK: u8 = 0x30 >> 3;

    /// Arbitration lost in SLA+W or data
    pub const TW_MT_ARB_LOST: u8 = 0x38 >> 3;

    // Master Receiver --------------------------------------------------------
    /// Arbitration lost in SLA+R or NACK
    pub const TW_MR_ARB_LOST: u8 = 0x38 >> 3;

    /// SLA+R transmitted, ACK received
    pub const TW_MR_SLA_ACK: u8 = 0x40 >> 3;

    /// SLA+R transmitted, NACK received
    pub const TW_MR_SLA_NACK: u8 = 0x48 >> 3;

    /// Data received, ACK returned
    pub const TW_MR_DATA_ACK: u8 = 0x50 >> 3;

    /// Data received, NACK returned
    pub const TW_MR_DATA_NACK: u8 = 0x58 >> 3;

    // Slave Transmitter ------------------------------------------------------
    /// SLA+R received, ACK returned
    pub const TW_ST_SLA_ACK: u8 = 0xA8 >> 3;

    /// Arbitration lost in SLA+RW, SLA+R received, ACK returned
    pub const TW_ST_ARB_LOST_SLA_ACK: u8 = 0xB0 >> 3;

    /// Data transmitted, ACK received
    pub const TW_ST_DATA_ACK: u8 = 0xB8 >> 3;

    /// Data transmitted, NACK received
    pub const TW_ST_DATA_NACK: u8 = 0xC0 >> 3;

    /// Last data byte transmitted, ACK received
    pub const TW_ST_LAST_DATA: u8 = 0xC8 >> 3;

    // Slave Receiver ---------------------------------------------------------
    /// SLA+W received, ACK returned
    pub const TW_SR_SLA_ACK: u8 = 0x60 >> 3;

    /// Arbitration lost in SLA+RW, SLA+W received, ACK returned
    pub const TW_SR_ARB_LOST_SLA_ACK: u8 = 0x68 >> 3;

    /// General call received, ACK returned
    pub const TW_SR_GCALL_ACK: u8 = 0x70 >> 3;

    /// Arbitration lost in SLA+RW, general call received, ACK returned
    pub const TW_SR_ARB_LOST_GCALL_ACK: u8 = 0x78 >> 3;

    /// Data received, ACK returned
    pub const TW_SR_DATA_ACK: u8 = 0x80 >> 3;

    /// Data received, NACK returned
    pub const TW_SR_DATA_NACK: u8 = 0x88 >> 3;

    /// General call data received, ACK returned
    pub const TW_SR_GCALL_DATA_ACK: u8 = 0x90 >> 3;

    /// General call data received, NACK returned
    pub const TW_SR_GCALL_DATA_NACK: u8 = 0x98 >> 3;

    /// Stop or repeated start condition received while selected
    pub const TW_SR_STOP: u8 = 0xA0 >> 3;

    // Misc -------------------------------------------------------------------
    /// No state information available
    pub const TW_NO_INFO: u8 = 0xF8 >> 3;

    /// Illegal start or stop condition
    pub const TW_BUS_ERROR: u8 = 0x00 >> 3;
}

/// I2C Error
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Error {
    /// Lost arbitration while trying to acquire bus
    ArbitrationLost,
    /// No slave answered for this address or a slave replied NACK
    AddressNack,
    /// Slave replied NACK to sent data
    DataNack,
    /// A bus-error occured
    BusError,
    /// An unknown error occured.  The bus might be in an unknown state.
    Unknown,
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match *self {
            Error::ArbitrationLost => embedded_hal::i2c::ErrorKind::ArbitrationLoss,
            Error::AddressNack => embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Address,
            ),
            Error::DataNack => embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Data,
            ),
            Error::BusError => embedded_hal::i2c::ErrorKind::Bus,
            Error::Unknown => embedded_hal::i2c::ErrorKind::Other,
        }
    }
}

impl<H, I2C: I2cOps<H, SDA, SCL>, SDA, SCL, CLOCK> embedded_hal::i2c::ErrorType
    for I2c<H, I2C, SDA, SCL, CLOCK>
{
    type Error = Error;
}

/// I2C Transfer Direction
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Direction {
    /// Write to a slave (LSB is 0)
    Write,
    /// Read from a slave (LSB is 1)
    Read,
}

/// Internal trait for low-level I2C peripherals.
///
/// This trait defines the common interface for all I2C peripheral variants.  It is used as an
/// intermediate abstraction ontop of which the [`I2c`] API is built.  **Prefer using the
/// [`I2c`] API instead of this trait.**
pub trait I2cOps<H, SDA, SCL> {
    /// Setup the bus for operation at a certain speed.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_setup<CLOCK: crate::clock::Clock>(&mut self, speed: u32);

    /// Start a bus transaction to a certain `address` in either read or write mode.
    ///
    /// If a previous transaction was not stopped via `raw_stop()`, this should generate a repeated
    /// start condition.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_start(&mut self, address: u8, direction: Direction) -> Result<(), Error>;

    /// Write some bytes to the bus.
    ///
    /// This method must only be called after a transaction in write mode was successfully started.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_write(&mut self, bytes: &[u8]) -> Result<(), Error>;

    /// Read some bytes from the bus.
    ///
    /// This method must only be called after a transaction in read mode was successfully started.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_read(&mut self, buffer: &mut [u8]) -> Result<(), Error>;

    /// Send a stop-condition and release the bus.
    ///
    /// This method must only be called after successfully starting a bus transaction.  This method
    /// does not need to block until the stop condition has actually occured.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_stop(&mut self) -> Result<(), Error>;
}

/// I2C driver
///
/// # Example
/// (for Arduino Uno)
/// ```
/// let dp = arduino_hal::Peripherals::take().unwrap();
/// let pins = arduino_hal::pins!(dp);
///
/// let mut i2c = arduino_hal::I2c::new(
///     dp.TWI,
///     pins.a4.into_pull_up_input(),
///     pins.a5.into_pull_up_input(),
///     50000,
/// );
///
/// // i2c implements the embedded-hal traits so it can be used with generic drivers.
/// ```
pub struct I2c<H, I2C: I2cOps<H, SDA, SCL>, SDA, SCL, CLOCK> {
    p: I2C,
    #[allow(dead_code)]
    sda: SDA,
    #[allow(dead_code)]
    scl: SCL,
    _clock: PhantomData<CLOCK>,
    _h: PhantomData<H>,
}

impl<H, I2C, SDAPIN, SCLPIN, CLOCK>
    I2c<H, I2C, port::Pin<port::mode::Input, SDAPIN>, port::Pin<port::mode::Input, SCLPIN>, CLOCK>
where
    I2C: I2cOps<H, port::Pin<port::mode::Input, SDAPIN>, port::Pin<port::mode::Input, SCLPIN>>,
    SDAPIN: port::PinOps,
    SCLPIN: port::PinOps,
    CLOCK: crate::clock::Clock,
{
    /// Initialize an I2C peripheral on the given pins.
    ///
    /// Note that the SDA and SCL pins are hardwired for each I2C peripheral and you *must* pass
    /// the correct ones.  This is enforced at compile time.
    ///
    /// This method expects the internal pull-ups to be configured for both pins to comply with the
    /// I2C specification.  If you have external pull-ups connected, use
    /// [`I2c::with_external_pullup`] instead.
    pub fn new(
        p: I2C,
        sda: port::Pin<port::mode::Input<port::mode::PullUp>, SDAPIN>,
        scl: port::Pin<port::mode::Input<port::mode::PullUp>, SCLPIN>,
        speed: u32,
    ) -> Self {
        let mut i2c = Self {
            p,
            sda: sda.forget_imode(),
            scl: scl.forget_imode(),
            _clock: PhantomData,
            _h: PhantomData,
        };
        i2c.p.raw_setup::<CLOCK>(speed);
        i2c
    }

    /// Initialize an I2C peripheral on the given pins.
    ///
    /// Note that the SDA and SCL pins are hardwired for each I2C peripheral and you *must* pass
    /// the correct ones.  This is enforced at compile time.
    ///
    /// This method expects that external resistors pull up SDA and SCL.
    pub fn with_external_pullup(
        p: I2C,
        sda: port::Pin<port::mode::Input<port::mode::Floating>, SDAPIN>,
        scl: port::Pin<port::mode::Input<port::mode::Floating>, SCLPIN>,
        speed: u32,
    ) -> Self {
        let mut i2c = Self {
            p,
            sda: sda.forget_imode(),
            scl: scl.forget_imode(),
            _clock: PhantomData,
            _h: PhantomData,
        };
        i2c.p.raw_setup::<CLOCK>(speed);
        i2c
    }
}

impl<H, I2C: I2cOps<H, SDA, SCL>, SDA, SCL, CLOCK> I2c<H, I2C, SDA, SCL, CLOCK>
where
    CLOCK: crate::clock::Clock,
    crate::delay::Delay<CLOCK>: embedded_hal_v0::blocking::delay::DelayMs<u16>,
{
    /// Test whether a device answers on a certain address.
    pub fn ping_device(&mut self, address: u8, direction: Direction) -> Result<bool, Error> {
        match self.p.raw_start(address, direction) {
            Ok(_) => {
                self.p.raw_stop()?;
                Ok(true)
            }
            Err(Error::AddressNack) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Scan the bus for connected devices.  This method will output an summary in the format known
    /// from [`i2cdetect(8)`][i2cdetect-linux] on the selected serial connection.  For example:
    ///
    /// ```text
    /// -    0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
    /// 00:       -- -- -- -- -- -- -- -- -- -- -- -- -- --
    /// 10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    /// 20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    /// 30: -- -- -- -- -- -- -- -- 38 39 -- -- -- -- -- --
    /// 40: -- -- -- -- -- -- -- -- 48 -- -- -- -- -- -- --
    /// 50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    /// 60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
    /// 70: -- -- -- -- -- -- -- --
    /// ```
    ///
    /// [i2cdetect-linux]: https://man.archlinux.org/man/community/i2c-tools/i2cdetect.8.en
    pub fn i2cdetect<W: ufmt::uWrite>(
        &mut self,
        w: &mut W,
        direction: Direction,
    ) -> Result<(), W::Error> {
        use embedded_hal_v0::blocking::delay::DelayMs;
        let mut delay = crate::delay::Delay::<CLOCK>::new();

        w.write_str(
            "\
-    0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f\r\n\
00:      ",
        )?;

        fn u4_to_hex(b: u8) -> char {
            match b {
                x if x < 0xa => (0x30 + x).into(),
                x if x < 0x10 => (0x57 + x).into(),
                _ => '?',
            }
        }

        for address in 0x02..=0x77 {
            let (ah, al) = (u4_to_hex(address >> 4), u4_to_hex(address & 0xf));

            if address % 0x10 == 0 {
                w.write_str("\r\n")?;
                w.write_char(ah)?;
                w.write_str("0:")?;
            }

            match self.ping_device(address, direction) {
                Ok(true) => {
                    w.write_char(' ')?;
                    w.write_char(ah)?;
                    w.write_char(al)?;
                }
                Ok(false) => {
                    w.write_str(" --")?;
                }
                Err(e) => {
                    w.write_str(" E")?;
                    w.write_char(u4_to_hex(e as u8))?;
                }
            }

            delay.delay_ms(10u16);
        }

        w.write_str("\r\n")?;

        Ok(())
    }
}

impl<H, I2C: I2cOps<H, SDA, SCL>, SDA, SCL, CLOCK> embedded_hal_v0::blocking::i2c::Write
    for I2c<H, I2C, SDA, SCL, CLOCK>
{
    type Error = Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.p.raw_start(address, Direction::Write)?;
        self.p.raw_write(bytes)?;
        self.p.raw_stop()?;
        Ok(())
    }
}

impl<H, I2C: I2cOps<H, SDA, SCL>, SDA, SCL, CLOCK> embedded_hal_v0::blocking::i2c::Read
    for I2c<H, I2C, SDA, SCL, CLOCK>
{
    type Error = Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.p.raw_start(address, Direction::Read)?;
        self.p.raw_read(buffer)?;
        self.p.raw_stop()?;
        Ok(())
    }
}

impl<H, I2C: I2cOps<H, SDA, SCL>, SDA, SCL, CLOCK> embedded_hal_v0::blocking::i2c::WriteRead
    for I2c<H, I2C, SDA, SCL, CLOCK>
{
    type Error = Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.p.raw_start(address, Direction::Write)?;
        self.p.raw_write(bytes)?;
        self.p.raw_start(address, Direction::Read)?;
        self.p.raw_read(buffer)?;
        self.p.raw_stop()?;
        Ok(())
    }
}

impl<H, I2C: I2cOps<H, SDA, SCL>, SDA, SCL, CLOCK> embedded_hal::i2c::I2c<SevenBitAddress>
    for I2c<H, I2C, SDA, SCL, CLOCK>
{
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let mut previous_direction = Direction::Read;
        for (idx, operation) in operations.iter_mut().enumerate() {
            match operation {
                embedded_hal::i2c::Operation::Read(buffer) => {
                    if idx == 0 || previous_direction != Direction::Read {
                        self.p.raw_start(address, Direction::Read)?;
                    }
                    self.p.raw_read(buffer)?;
                    previous_direction = Direction::Read;
                }
                embedded_hal::i2c::Operation::Write(bytes) => {
                    if idx == 0 || previous_direction != Direction::Write {
                        self.p.raw_start(address, Direction::Write)?;
                    }
                    self.p.raw_write(bytes)?;
                    previous_direction = Direction::Write;
                }
            }
        }
        if operations.len() > 0 {
            self.p.raw_stop()?;
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! impl_i2c_twi {
    (
        hal: $HAL:ty,
        peripheral: $I2C:ty,
        sda: $sdapin:ty,
        scl: $sclpin:ty,
    ) => {
        impl
            $crate::i2c::I2cOps<
                $HAL,
                $crate::port::Pin<$crate::port::mode::Input, $sdapin>,
                $crate::port::Pin<$crate::port::mode::Input, $sclpin>,
            > for $I2C
        {
            #[inline]
            fn raw_setup<CLOCK: $crate::clock::Clock>(&mut self, speed: u32) {
                // Calculate TWBR register value
                let twbr = ((CLOCK::FREQ / speed) - 16) / 2;
                self.twbr.write(|w| unsafe { w.bits(twbr as u8) });

                // Disable prescaler
                self.twsr.write(|w| w.twps().prescaler_1());
            }

            #[inline]
            fn raw_start(&mut self, address: u8, direction: Direction) -> Result<(), Error> {
                // Write start condition
                self.twcr
                    .write(|w| w.twen().set_bit().twint().set_bit().twsta().set_bit());
                // wait()
                while self.twcr.read().twint().bit_is_clear() {}

                // Validate status
                match self.twsr.read().tws().bits() {
                    $crate::i2c::twi_status::TW_START | $crate::i2c::twi_status::TW_REP_START => (),
                    $crate::i2c::twi_status::TW_MT_ARB_LOST
                    | $crate::i2c::twi_status::TW_MR_ARB_LOST => {
                        return Err($crate::i2c::Error::ArbitrationLost);
                    }
                    $crate::i2c::twi_status::TW_BUS_ERROR => {
                        return Err($crate::i2c::Error::BusError);
                    }
                    _ => {
                        return Err($crate::i2c::Error::Unknown);
                    }
                }

                // Send slave address
                let dirbit = if direction == $crate::i2c::Direction::Read {
                    1
                } else {
                    0
                };
                let rawaddr = (address << 1) | dirbit;
                self.twdr.write(|w| unsafe { w.bits(rawaddr) });
                // transact()
                self.twcr.write(|w| w.twen().set_bit().twint().set_bit());
                while self.twcr.read().twint().bit_is_clear() {}

                // Check if the slave responded
                match self.twsr.read().tws().bits() {
                    $crate::i2c::twi_status::TW_MT_SLA_ACK
                    | $crate::i2c::twi_status::TW_MR_SLA_ACK => (),
                    $crate::i2c::twi_status::TW_MT_SLA_NACK
                    | $crate::i2c::twi_status::TW_MR_SLA_NACK => {
                        // Stop the transaction if it did not respond
                        self.raw_stop()?;
                        return Err($crate::i2c::Error::AddressNack);
                    }
                    $crate::i2c::twi_status::TW_MT_ARB_LOST
                    | $crate::i2c::twi_status::TW_MR_ARB_LOST => {
                        return Err($crate::i2c::Error::ArbitrationLost);
                    }
                    $crate::i2c::twi_status::TW_BUS_ERROR => {
                        return Err($crate::i2c::Error::BusError);
                    }
                    _ => {
                        return Err($crate::i2c::Error::Unknown);
                    }
                }

                Ok(())
            }

            #[inline]
            fn raw_write(&mut self, bytes: &[u8]) -> Result<(), Error> {
                for byte in bytes {
                    self.twdr.write(|w| unsafe { w.bits(*byte) });
                    // transact()
                    self.twcr.write(|w| w.twen().set_bit().twint().set_bit());
                    while self.twcr.read().twint().bit_is_clear() {}

                    match self.twsr.read().tws().bits() {
                        $crate::i2c::twi_status::TW_MT_DATA_ACK => (),
                        $crate::i2c::twi_status::TW_MT_DATA_NACK => {
                            self.raw_stop()?;
                            return Err($crate::i2c::Error::DataNack);
                        }
                        $crate::i2c::twi_status::TW_MT_ARB_LOST => {
                            return Err($crate::i2c::Error::ArbitrationLost);
                        }
                        $crate::i2c::twi_status::TW_BUS_ERROR => {
                            return Err($crate::i2c::Error::BusError);
                        }
                        _ => {
                            return Err($crate::i2c::Error::Unknown);
                        }
                    }
                }
                Ok(())
            }

            #[inline]
            fn raw_read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
                let last = buffer.len() - 1;
                for (i, byte) in buffer.iter_mut().enumerate() {
                    if i != last {
                        self.twcr
                            .write(|w| w.twint().set_bit().twen().set_bit().twea().set_bit());
                        // wait()
                        while self.twcr.read().twint().bit_is_clear() {}
                    } else {
                        self.twcr.write(|w| w.twint().set_bit().twen().set_bit());
                        // wait()
                        while self.twcr.read().twint().bit_is_clear() {}
                    }

                    match self.twsr.read().tws().bits() {
                        $crate::i2c::twi_status::TW_MR_DATA_ACK
                        | $crate::i2c::twi_status::TW_MR_DATA_NACK => (),
                        $crate::i2c::twi_status::TW_MR_ARB_LOST => {
                            return Err($crate::i2c::Error::ArbitrationLost);
                        }
                        $crate::i2c::twi_status::TW_BUS_ERROR => {
                            return Err($crate::i2c::Error::BusError);
                        }
                        _ => {
                            return Err($crate::i2c::Error::Unknown);
                        }
                    }

                    *byte = self.twdr.read().bits();
                }
                Ok(())
            }

            #[inline]
            fn raw_stop(&mut self) -> Result<(), Error> {
                self.twcr
                    .write(|w| w.twen().set_bit().twint().set_bit().twsto().set_bit());
                Ok(())
            }
        }
    };
}
