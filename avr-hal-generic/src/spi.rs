//! SPI Implementation
use crate::port;
use core::marker::PhantomData;
use embedded_hal::spi::{self, SpiBus};

/// Oscillator Clock Frequency division options.
///
/// The bus speed is calculated by dividing the IO clock by the prescaler:
///
/// ```text
/// F_sck = CLK_io / Prescaler
/// ```
///
/// Please note that the overall transfer speed might be lower due to software overhead while
/// sending / receiving.
///
/// | Prescale | 16 MHz Clock | 8 MHz Clock |
/// | --- | --- | --- |
/// | `OscfOver2` | 8 MHz | 4 MHz |
/// | `OscfOver4` | 4 MHz | 2 MHz |
/// | `OscfOver8` | 2 MHz | 1 MHz |
/// | `OscfOver16` | 1 MHz | 500 kHz |
/// | `OscfOver32` | 500 kHz | 250 kHz |
/// | `OscfOver64` | 250 kHz | 125 kHz |
/// | `OscfOver128` | 125 kHz | 62.5 kHz |
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SerialClockRate {
    OscfOver2 = 1,
    OscfOver4 = 2,
    OscfOver8 = 3,
    OscfOver16 = 4,
    OscfOver32 = 5,
    OscfOver64 = 6,
    OscfOver128 = 7,
}

impl SerialClockRate {
    pub fn into_divider(self) -> u8 {
        2u8.pow(self as u32)
    }
}

/// Order of data transmission, either MSB first or LSB first
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataOrder {
    MostSignificantFirst,
    LeastSignificantFirst,
}

/// Settings to pass to Spi.
///
/// Easiest way to initialize is with
/// `Settings::default()`.  Otherwise can be instantiated with alternate
/// settings directly.
#[derive(Clone, PartialEq, Eq)]
pub struct Settings {
    pub data_order: DataOrder,
    pub clock: SerialClockRate,
    pub mode: spi::Mode,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            data_order: DataOrder::MostSignificantFirst,
            clock: SerialClockRate::OscfOver4,
            mode: spi::MODE_1,
        }
    }
}

/// Internal trait for low-level SPI peripherals
///
/// This trait defines the common interface for all SPI peripheral variants.  It is used as an
/// intermediate abstraction ontop of which the [`Spi`] API is built.  **Prefer using the
/// [`Spi`] API instead of this trait.**
pub trait SpiOps<H, SCLK, MOSI, MISO, CS> {
    /// Sets up the control/status registers with the right settings for this secondary device
    fn raw_setup(&mut self, settings: &Settings);
    /// Disable the peripheral
    fn raw_release(&mut self);

    /// Check the interrupt flag to see if the write has completed
    ///
    /// Returns `true` if the bus is idle
    fn raw_check_iflag(&self) -> bool;
    /// Read a byte from the data register
    fn raw_read(&self) -> u8;
    /// Write a byte to the data register, which begins transmission
    /// automatically.
    fn raw_write(&mut self, byte: u8);
    /// Perform a transaction of a single byte
    fn raw_transaction(&mut self, byte: u8) -> u8;
}

/// Wrapper for the CS pin
///
/// Used to contain the chip-select pin during operation to prevent its mode from being
/// changed from Output. This is necessary because the SPI state machine would otherwise
/// reset itself to SPI slave mode immediately. This wrapper can be used just like an
/// output pin, because it implements all the same traits from embedded-hal.
pub struct ChipSelectPin<CSPIN>(port::Pin<port::mode::Output, CSPIN>);

impl<CSPIN: port::PinOps> ChipSelectPin<CSPIN> {
    /// Convert this `ChipSelectPin` into the underlying "real" `Pin<>` object.
    ///
    /// Safety
    /// ======
    /// This function is unsafe because the underlying pin can be converted into a non-output mode
    /// which would break SPI functionality.  The user must ensure the pin is only used in output
    /// modes after calling this function.
    pub unsafe fn into_pin_unchecked(self) -> port::Pin<port::mode::Output, CSPIN> {
        self.0
    }

    /// (Re)create a `ChipSelectPin` from a real `Pin<>` object.
    ///
    /// This function is only meant to be used when the pin was previously moved out of the
    /// `ChipSelectPin` using [`ChipSelectPin::into_pin_unchecked()`].
    pub unsafe fn from_pin(pin: port::Pin<port::mode::Output, CSPIN>) -> Self {
        Self(pin)
    }
}

impl<CSPIN: port::PinOps> embedded_hal_v0::digital::v2::OutputPin for ChipSelectPin<CSPIN> {
    type Error = core::convert::Infallible;
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0.set_low();
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0.set_high();
        Ok(())
    }
}

impl<CSPIN: port::PinOps> embedded_hal_v0::digital::v2::StatefulOutputPin for ChipSelectPin<CSPIN> {
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.0.is_set_low())
    }
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.0.is_set_high())
    }
}

impl<CSPIN: port::PinOps> embedded_hal_v0::digital::v2::ToggleableOutputPin
    for ChipSelectPin<CSPIN>
{
    type Error = core::convert::Infallible;
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.0.toggle();
        Ok(())
    }
}

impl<CSPIN: port::PinOps> embedded_hal::digital::ErrorType for ChipSelectPin<CSPIN> {
    type Error = core::convert::Infallible;
}

impl<CSPIN: port::PinOps> embedded_hal::digital::OutputPin for ChipSelectPin<CSPIN> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0.set_high();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0.set_low();
        Ok(())
    }
}

impl<CSPIN: port::PinOps> embedded_hal::digital::StatefulOutputPin for ChipSelectPin<CSPIN> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_set_high())
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_set_low())
    }
}

/// Behavior for a SPI interface.
///
/// Stores the SPI peripheral for register access.  In addition, it takes
/// ownership of the MOSI and MISO pins to ensure they are in the correct mode.
/// Instantiate with the `new` method.
///
/// This can be used both with the embedded-hal 0.2 [`spi::FullDuplex`] trait, and
/// with the embedded-hal 1.0 [`spi::SpiBus`] trait.
///
/// [`spi::FullDuplex`]: `embedded_hal_v0::spi::FullDuplex`
/// [`spi::SpiBus`]: `embedded_hal::spi::SpiBus`
pub struct Spi<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN> {
    p: SPI,
    sclk: port::Pin<port::mode::Output, SCLKPIN>,
    mosi: port::Pin<port::mode::Output, MOSIPIN>,
    miso: port::Pin<port::mode::Input, MISOPIN>,
    write_in_progress: bool,
    _cs: PhantomData<CSPIN>,
    _h: PhantomData<H>,
}

impl<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN> Spi<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>
where
    SPI: SpiOps<H, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>,
    SCLKPIN: port::PinOps,
    MOSIPIN: port::PinOps,
    MISOPIN: port::PinOps,
    CSPIN: port::PinOps,
{
    /// Instantiate an SPI with the registers, SCLK/MOSI/MISO/CS pins, and settings,
    /// with the internal pull-up enabled on the MISO pin.
    ///
    /// The pins are not actually used directly, but they are moved into the struct in
    /// order to enforce that they are in the correct mode, and cannot be used by anyone
    /// else while SPI is active.  CS is placed into a `ChipSelectPin` instance and given
    /// back so that its output state can be changed as needed.
    pub fn new(
        p: SPI,
        sclk: port::Pin<port::mode::Output, SCLKPIN>,
        mosi: port::Pin<port::mode::Output, MOSIPIN>,
        miso: port::Pin<port::mode::Input<port::mode::PullUp>, MISOPIN>,
        cs: port::Pin<port::mode::Output, CSPIN>,
        settings: Settings,
    ) -> (Self, ChipSelectPin<CSPIN>) {
        let mut spi = Self {
            p,
            sclk,
            mosi,
            miso: miso.forget_imode(),
            write_in_progress: false,
            _cs: PhantomData,
            _h: PhantomData,
        };
        spi.p.raw_setup(&settings);
        (spi, ChipSelectPin(cs))
    }

    /// Instantiate an SPI with the registers, SCLK/MOSI/MISO/CS pins, and settings,
    /// with an external pull-up on the MISO pin.
    ///
    /// The pins are not actually used directly, but they are moved into the struct in
    /// order to enforce that they are in the correct mode, and cannot be used by anyone
    /// else while SPI is active.
    pub fn with_external_pullup(
        p: SPI,
        sclk: port::Pin<port::mode::Output, SCLKPIN>,
        mosi: port::Pin<port::mode::Output, MOSIPIN>,
        miso: port::Pin<port::mode::Input<port::mode::Floating>, MISOPIN>,
        cs: port::Pin<port::mode::Output, CSPIN>,
        settings: Settings,
    ) -> (Self, ChipSelectPin<CSPIN>) {
        let mut spi = Self {
            p,
            sclk,
            mosi,
            miso: miso.forget_imode(),
            write_in_progress: false,
            _cs: PhantomData,
            _h: PhantomData,
        };
        spi.p.raw_setup(&settings);
        (spi, ChipSelectPin(cs))
    }

    /// Reconfigure the SPI peripheral after initializing
    pub fn reconfigure(&mut self, settings: Settings) -> nb::Result<(), core::convert::Infallible> {
        // wait for any in-flight writes to complete
        self.flush()?;
        self.p.raw_setup(&settings);
        Ok(())
    }

    /// Disable the SPI device and release ownership of the peripheral
    /// and pins.  Instance can no-longer be used after this is
    /// invoked.
    pub fn release(
        mut self,
        cs: ChipSelectPin<CSPIN>,
    ) -> (
        SPI,
        port::Pin<port::mode::Output, SCLKPIN>,
        port::Pin<port::mode::Output, MOSIPIN>,
        port::Pin<port::mode::Input, MISOPIN>,
        port::Pin<port::mode::Output, CSPIN>,
    ) {
        self.p.raw_release();
        (self.p, self.sclk, self.mosi, self.miso, cs.0)
    }

    fn flush(&mut self) -> nb::Result<(), core::convert::Infallible> {
        if self.write_in_progress {
            if self.p.raw_check_iflag() {
                self.write_in_progress = false;
            } else {
                return Err(nb::Error::WouldBlock);
            }
        }
        Ok(())
    }

    fn receive(&mut self) -> u8 {
        self.p.raw_read()
    }

    fn write(&mut self, byte: u8) {
        self.write_in_progress = true;
        self.p.raw_write(byte);
    }
}

/// FullDuplex trait implementation, allowing this struct to be provided to
/// drivers that require it for operation.  Only 8-bit word size is supported
/// for now.
impl<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN> embedded_hal_v0::spi::FullDuplex<u8>
    for Spi<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>
where
    SPI: SpiOps<H, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>,
    SCLKPIN: port::PinOps,
    MOSIPIN: port::PinOps,
    MISOPIN: port::PinOps,
    CSPIN: port::PinOps,
{
    type Error = core::convert::Infallible;

    /// Sets up the device for transmission and sends the data
    fn send(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.flush()?;
        self.write(byte);
        Ok(())
    }

    /// Reads and returns the response in the data register
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.flush()?;
        Ok(self.receive())
    }
}

impl<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN> embedded_hal::spi::ErrorType
    for Spi<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>
where
    SPI: SpiOps<H, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>,
    SCLKPIN: port::PinOps,
    MOSIPIN: port::PinOps,
    MISOPIN: port::PinOps,
    CSPIN: port::PinOps,
{
    type Error = core::convert::Infallible;
}

impl<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN> SpiBus
    for Spi<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>
where
    SPI: SpiOps<H, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>,
    SCLKPIN: port::PinOps,
    MOSIPIN: port::PinOps,
    MISOPIN: port::PinOps,
    CSPIN: port::PinOps,
{
    fn flush(&mut self) -> Result<(), Self::Error> {
        if self.write_in_progress {
            while !self.p.raw_check_iflag() {}
            self.write_in_progress = false;
        }

        Ok(())
    }
    fn read(&mut self, read: &mut [u8]) -> Result<(), Self::Error> {
        // Must flush() first, if asynchronous operations happened before this.
        // To be removed if in the future we "only" implement embedded_hal 1.0
        SpiBus::flush(self)?;

        for b in read.iter_mut() {
            // We send 0x00 on MOSI during "pure" reading
            *b = self.p.raw_transaction(0x00);
        }

        Ok(())
    }

    fn write(&mut self, write: &[u8]) -> Result<(), Self::Error> {
        // Must flush() first, if asynchronous operations happened before this.
        // To be removed if in the future we "only" implement embedded_hal 1.0
        SpiBus::flush(self)?;

        for b in write.iter() {
            self.p.raw_transaction(*b);
        }

        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        // Must flush() first, if asynchronous operations happened before this.
        // To be removed if in the future we "only" implement embedded_hal 1.0
        SpiBus::flush(self)?;

        let longest = read.len().max(write.len());
        for i in 0..longest {
            let r = self.p.raw_transaction(*write.get(i).unwrap_or(&0x00));
            if i < read.len() {
                read[i] = r;
            }
        }

        Ok(())
    }

    fn transfer_in_place(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Must flush() first, if asynchronous operations happened before this.
        // To be removed if in the future we "only" implement embedded_hal 1.0
        SpiBus::flush(self)?;

        for b in buffer.iter_mut() {
            *b = self.p.raw_transaction(*b)
        }

        Ok(())
    }
}

/// Default Transfer trait implementation. Only 8-bit word size is supported for now.
impl<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN> embedded_hal_v0::blocking::spi::transfer::Default<u8>
    for Spi<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>
where
    SPI: SpiOps<H, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>,
    SCLKPIN: port::PinOps,
    MOSIPIN: port::PinOps,
    MISOPIN: port::PinOps,
    CSPIN: port::PinOps,
{
}

/// Default Write trait implementation. Only 8-bit word size is supported for now.
impl<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN> embedded_hal_v0::blocking::spi::write::Default<u8>
    for Spi<H, SPI, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>
where
    SPI: SpiOps<H, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>,
    SCLKPIN: port::PinOps,
    MOSIPIN: port::PinOps,
    MISOPIN: port::PinOps,
    CSPIN: port::PinOps,
{
}

/// Implement traits for a SPI interface
#[macro_export]
macro_rules! impl_spi {
    (
        hal: $HAL:ty,
        peripheral: $SPI:ty,
        sclk: $sclkpin:ty,
        mosi: $mosipin:ty,
        miso: $misopin:ty,
        cs: $cspin:ty,
    ) => {
        impl $crate::spi::SpiOps<$HAL, $sclkpin, $mosipin, $misopin, $cspin> for $SPI {
            fn raw_setup(&mut self, settings: &Settings) {
                use $crate::hal::spi;

                // set up control register
                self.spcr.write(|w| {
                    // enable SPI
                    w.spe().set_bit();
                    // Set to primary mode
                    w.mstr().set_bit();
                    // set up data order control bit
                    match settings.data_order {
                        DataOrder::MostSignificantFirst => w.dord().clear_bit(),
                        DataOrder::LeastSignificantFirst => w.dord().set_bit(),
                    };
                    // set up polarity control bit
                    match settings.mode.polarity {
                        spi::Polarity::IdleHigh => w.cpol().set_bit(),
                        spi::Polarity::IdleLow => w.cpol().clear_bit(),
                    };
                    // set up phase control bit
                    match settings.mode.phase {
                        spi::Phase::CaptureOnFirstTransition => w.cpha().clear_bit(),
                        spi::Phase::CaptureOnSecondTransition => w.cpha().set_bit(),
                    };
                    // set up clock rate control bit
                    match settings.clock {
                        SerialClockRate::OscfOver2 => w.spr().fosc_4_2(),
                        SerialClockRate::OscfOver4 => w.spr().fosc_4_2(),
                        SerialClockRate::OscfOver8 => w.spr().fosc_16_8(),
                        SerialClockRate::OscfOver16 => w.spr().fosc_16_8(),
                        SerialClockRate::OscfOver32 => w.spr().fosc_64_32(),
                        SerialClockRate::OscfOver64 => w.spr().fosc_64_32(),
                        SerialClockRate::OscfOver128 => w.spr().fosc_128_64(),
                    }
                });
                // set up 2x clock rate status bit
                self.spsr.write(|w| match settings.clock {
                    SerialClockRate::OscfOver2 => w.spi2x().set_bit(),
                    SerialClockRate::OscfOver4 => w.spi2x().clear_bit(),
                    SerialClockRate::OscfOver8 => w.spi2x().set_bit(),
                    SerialClockRate::OscfOver16 => w.spi2x().clear_bit(),
                    SerialClockRate::OscfOver32 => w.spi2x().set_bit(),
                    SerialClockRate::OscfOver64 => w.spi2x().clear_bit(),
                    SerialClockRate::OscfOver128 => w.spi2x().clear_bit(),
                });
            }

            fn raw_release(&mut self) {
                self.spcr.write(|w| w.spe().clear_bit());
            }

            fn raw_check_iflag(&self) -> bool {
                self.spsr.read().spif().bit_is_set()
            }

            fn raw_read(&self) -> u8 {
                self.spdr.read().bits()
            }

            fn raw_write(&mut self, byte: u8) {
                self.spdr.write(|w| unsafe { w.bits(byte) });
            }

            fn raw_transaction(&mut self, byte: u8) -> u8 {
                self.raw_write(byte);
                while !self.raw_check_iflag() {}
                self.raw_read()
            }
        }
    };
}
