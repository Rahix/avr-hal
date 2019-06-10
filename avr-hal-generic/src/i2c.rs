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

#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    ArbitrationLost,
    AddressNack,
    DataNack,
    BusError,
    Unknown,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Write,
    Read,
}

#[doc(hidden)]
pub fn i2cdetect<W: ufmt::uWrite, F>(s: &mut W, mut f: F) -> Result<(), W::Error>
where
    // Detection function
    F: FnMut(u8) -> Result<bool, Error>,
{
    s.write_str("\
-    0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f\r\n\
00:      ")?;

    fn u4_to_hex(b: u8) -> char {
        match b {
            x if x < 0xa => (0x30 + x).into(),
            x if x < 0x10 => (0x57 + x).into(),
            _ => '?',
        }
    }

    for addr in 0x02..=0x77 {
        let (ah, al) = (u4_to_hex(addr >> 4), u4_to_hex(addr & 0xf));

        if addr % 0x10 == 0 {
            s.write_str("\r\n")?;
            s.write_char(ah)?;
            s.write_str("0:")?;
        }

        match f(addr) {
            Ok(true) => {
                s.write_char(' ')?;
                s.write_char(ah)?;
                s.write_char(al)?;
            },
            Ok(false) => {
                s.write_str(" --")?;
            },
            Err(e) => {
                s.write_str(" E")?;
                s.write_char(u4_to_hex(e as u8))?;
            },
        }
    }

    s.write_str("\r\n")?;

    Ok(())
}

#[doc(hidden)]
pub type I2cMode = crate::port::mode::Input<crate::port::mode::Floating>;

#[macro_export]
macro_rules! impl_twi_i2c {
    (
        pub struct $I2c:ident {
            peripheral: $I2C:ty,
            pins: {
                sda: $sdamod:ident::$SDA:ident,
                scl: $sclmod:ident::$SCL:ident,
            },
            registers: {
                control: $twcr:ident {
                    enable: $twen:ident,
                    ack: $twea:ident,
                    int: $twint:ident,
                    start: $twstart:ident,
                    stop: $twstop:ident,
                },
                status: $twsr:ident {
                    prescaler: $twps:ident,
                    status: $tws:ident,
                },
                bitrate: $twbr:ident,
                data: $twdr:ident,
            },
        }
    ) => {
        pub struct $I2c<CLOCK: $crate::clock::Clock> {
            p: $I2C,
            _clock: ::core::marker::PhantomData<CLOCK>,
            sda: $sdamod::$SDA<$crate::i2c::I2cMode>,
            scl: $sclmod::$SCL<$crate::i2c::I2cMode>,
        }

        impl<CLOCK> $I2c<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
        {
            pub fn new(
                p: $I2C,
                sda: $sdamod::$SDA<$crate::i2c::I2cMode>,
                scl: $sclmod::$SCL<$crate::i2c::I2cMode>,
                speed: u32,
            ) -> $I2c<CLOCK> {
                // Calculate TWBR
                let twbr = ((CLOCK::FREQ / speed) - 16) / 2;
                p.$twbr.write(|w| w.bits(twbr as u8));
                // Disable prescaler
                p.$twsr.modify(|_, w| w.$twps().prescaler_1());

                $I2c {
                    p,
                    sda,
                    scl,
                    _clock: ::core::marker::PhantomData,
                }
            }

            pub fn ping_slave(
                &mut self,
                addr: u8,
                dir: $crate::i2c::Direction,
            ) -> Result<bool, $crate::i2c::Error> {
                match self.start(addr, dir) {
                    Err($crate::i2c::Error::AddressNack) => Ok(false),
                    Err(e) => Err(e),
                    Ok(()) => {
                        self.stop();
                        Ok(true)
                    },
                }
            }

            fn start(
                &mut self,
                addr: u8,
                dir: $crate::i2c::Direction,
            ) -> Result<(), $crate::i2c::Error> {
                // Write start condition
                self.p.$twcr.write(|w| w
                    .$twen().set_bit()
                    .$twint().set_bit()
                    .$twstart().set_bit()
                );
                self.wait();

                // Validate status
                match self.p.$twsr.read().$tws().bits() {
                      $crate::i2c::twi_status::TW_START
                    | $crate::i2c::twi_status::TW_REP_START => (),
                      $crate::i2c::twi_status::TW_MT_ARB_LOST
                    | $crate::i2c::twi_status::TW_MR_ARB_LOST => {
                        return Err($crate::i2c::Error::ArbitrationLost);
                    },
                    $crate::i2c::twi_status::TW_BUS_ERROR => {
                        return Err($crate::i2c::Error::BusError);
                    },
                    _ => {
                        return Err($crate::i2c::Error::Unknown);
                    },
                }

                // Send slave address
                let dirbit = if dir == $crate::i2c::Direction::Read { 1 } else { 0 };
                let rawaddr = (addr << 1) | dirbit;
                self.p.$twdr.write(|w| w.bits(rawaddr));
                self.transact();

                // Check if the slave responded
                match self.p.$twsr.read().$tws().bits() {
                      $crate::i2c::twi_status::TW_MT_SLA_ACK
                    | $crate::i2c::twi_status::TW_MR_SLA_ACK => (),
                      $crate::i2c::twi_status::TW_MT_SLA_NACK
                    | $crate::i2c::twi_status::TW_MR_SLA_NACK => {
                        // Stop the transaction if it did not respond
                        self.stop();
                        return Err($crate::i2c::Error::AddressNack);
                    },
                      $crate::i2c::twi_status::TW_MT_ARB_LOST
                    | $crate::i2c::twi_status::TW_MR_ARB_LOST => {
                        return Err($crate::i2c::Error::ArbitrationLost);
                    },
                    $crate::i2c::twi_status::TW_BUS_ERROR => {
                        return Err($crate::i2c::Error::BusError);
                    },
                    _ => {
                        return Err($crate::i2c::Error::Unknown);
                    },
                }

                Ok(())
            }

            fn wait(&mut self) {
                while self.p.$twcr.read().$twint().bit_is_clear() { }
            }

            fn transact(&mut self) {
                self.p.$twcr.write(|w| w.$twen().set_bit().$twint().set_bit());
                while self.p.$twcr.read().$twint().bit_is_clear() { }
            }

            fn write_data(&mut self, bytes: &[u8]) -> Result<(), $crate::i2c::Error> {
                for byte in bytes {
                    self.p.$twdr.write(|w| w.bits(*byte));
                    self.transact();

                    match self.p.$twsr.read().$tws().bits() {
                        $crate::i2c::twi_status::TW_MT_DATA_ACK => (),
                        $crate::i2c::twi_status::TW_MT_DATA_NACK => {
                            self.stop();
                            return Err($crate::i2c::Error::DataNack);
                        },
                        $crate::i2c::twi_status::TW_MT_ARB_LOST => {
                            return Err($crate::i2c::Error::ArbitrationLost);
                        },
                        $crate::i2c::twi_status::TW_BUS_ERROR => {
                            return Err($crate::i2c::Error::BusError);
                        },
                        _ => {
                            return Err($crate::i2c::Error::Unknown);
                        },
                    }
                }
                Ok(())
            }

            fn read_data(&mut self, buffer: &mut [u8]) -> Result<(), $crate::i2c::Error> {
                let last = buffer.len() - 1;
                for (i, byte) in buffer.iter_mut().enumerate() {
                    if i != last {
                        self.p.$twcr.write(|w| w.$twint().set_bit().$twen().set_bit().$twea().set_bit());
                        self.wait();
                    } else {
                        self.p.$twcr.write(|w| w.$twint().set_bit().$twen().set_bit());
                        self.wait();
                    }

                    match self.p.$twsr.read().$tws().bits() {
                          $crate::i2c::twi_status::TW_MR_DATA_ACK
                        | $crate::i2c::twi_status::TW_MR_DATA_NACK => (),
                        $crate::i2c::twi_status::TW_MR_ARB_LOST => {
                            return Err($crate::i2c::Error::ArbitrationLost);
                        },
                        $crate::i2c::twi_status::TW_BUS_ERROR => {
                            return Err($crate::i2c::Error::BusError);
                        },
                        _ => {
                            return Err($crate::i2c::Error::Unknown);
                        },
                    }

                    *byte = self.p.$twdr.read().bits();
                }
                Ok(())
            }

            fn stop(&mut self) {
                // Send stop
                self.p.$twcr.write(|w| w
                    .$twen().set_bit()
                    .$twint().set_bit()
                    .$twstop().set_bit()
                );
            }
        }

        impl<CLOCK> $I2c<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
            $crate::delay::Delay<CLOCK>: $crate::hal::blocking::delay::DelayMs<u16>,
        {
            pub fn i2cdetect<W: $crate::ufmt::uWrite>(
                &mut self,
                w: &mut W,
                dir: $crate::i2c::Direction,
            ) -> Result<(), W::Error> {
                let mut delay = $crate::delay::Delay::<CLOCK>::new();
                $crate::i2c::i2cdetect(w, |a| {
                    use $crate::prelude::*;

                    delay.delay_ms(10u16);
                    self.ping_slave(a, dir)
                })
            }
        }


        impl<CLOCK> $crate::hal::blocking::i2c::Write for $I2c<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
        {
            type Error = $crate::i2c::Error;

            fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                self.start(address, $crate::i2c::Direction::Write)?;
                self.write_data(bytes)?;
                self.stop();
                Ok(())
            }
        }

        impl<CLOCK> $crate::hal::blocking::i2c::Read for $I2c<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
        {
            type Error = $crate::i2c::Error;

            fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                self.start(address, $crate::i2c::Direction::Read)?;
                self.read_data(buffer)?;
                self.stop();
                Ok(())
            }
        }

        impl<CLOCK> $crate::hal::blocking::i2c::WriteRead for $I2c<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
        {
            type Error = $crate::i2c::Error;

            fn write_read(
                &mut self,
                address: u8,
                bytes: &[u8],
                buffer: &mut [u8],
            ) -> Result<(), Self::Error> {
                self.start(address, $crate::i2c::Direction::Write)?;
                self.write_data(bytes)?;
                self.start(address, $crate::i2c::Direction::Read)?;
                self.read_data(buffer)?;
                self.stop();
                Ok(())
            }
        }
    };
}
