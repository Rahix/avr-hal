//! SPI Implementation

pub use embedded_hal::spi;

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
    OscfOver2,
    OscfOver4,
    OscfOver8,
    OscfOver16,
    OscfOver32,
    OscfOver64,
    OscfOver128,
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
            mode: spi::Mode {
                polarity: spi::Polarity::IdleLow,
                phase: spi::Phase::CaptureOnSecondTransition,
            },
        }
    }
}

/// Implement traits for a SPI interface
#[macro_export]
macro_rules! impl_spi {
    (
        $(#[$spi_attr:meta])*
        pub struct $Spi:ident {
            peripheral: $SPI:ty,
            pins: {
                sclk: $sclkmod:ident::$SCLK:ident,
                mosi: $mosimod:ident::$MOSI:ident,
                miso: $misomod:ident::$MISO:ident,
            }
        }
    ) => {

        use $crate::void::Void;
        use $crate::hal::spi;
        pub use avr_hal::spi::*;

        type SCLK = $sclkmod::$SCLK<$crate::port::mode::Output>;
        type MOSI = $mosimod::$MOSI<$crate::port::mode::Output>;
        type MISO = $misomod::$MISO<$crate::port::mode::Input<$crate::port::mode::PullUp>>;

        /// Behavior for a SPI interface.
        ///
        /// Stores the SPI peripheral for register access.  In addition, it takes
        /// ownership of the MOSI and MISO pins to ensure they are in the correct mode.
        /// Instantiate with the `new` method.
        $(#[$spi_attr])*
        pub struct $Spi {
            peripheral: $SPI,
            sclk: SCLK,
            mosi: MOSI,
            miso: MISO,
            settings: Settings,
            is_write_in_progress: bool,
        }

        /// Implementation-specific behavior of the struct, including setup/tear-down
        impl $Spi {
            /// Instantiate an SPI with the registers, SCLK/MOSI/MISO pins, and settings
            ///
            /// The pins are not actually used directly, but they are moved into the struct in
            /// order to enforce that they are in the correct mode, and cannot be used by anyone
            /// else while SPI is active.
            pub fn new(peripheral: $SPI, sclk: SCLK, mosi: MOSI, miso: MISO, settings: Settings) -> $Spi {
                let spi = Spi {
                    peripheral,
                    sclk,
                    mosi,
                    miso,
                    settings,
                    is_write_in_progress: false,
                };
                spi.setup();
                spi
            }

            /// Disable the SPI device and release ownership of the peripheral
            /// and pins.  Instance can no-longer be used after this is
            /// invoked.
            pub fn release(self) -> ($SPI, SCLK, MOSI, MISO) {
                self.peripheral.spcr.write(|w| {
                    w.spe().clear_bit()
                });
                (self.peripheral, self.sclk, self.mosi, self.miso)
            }

            /// Write a byte to the data register, which begins transmission
            /// automatically.
            fn write(&mut self, byte: u8) {
                self.is_write_in_progress = true;
                self.peripheral.spdr.write(|w| unsafe { w.bits(byte) });
            }

            /// Check if write flag is set, and return a WouldBlock error if it is not.
            fn flush(&mut self) -> $crate::nb::Result<(), $crate::void::Void> {
                if self.is_write_in_progress {
                    if self.peripheral.spsr.read().spif().bit_is_set() {
                        self.is_write_in_progress = false;
                    } else {
                        return Err($crate::nb::Error::WouldBlock);
                    }
                }
                Ok(())
            }

            /// Sets up the control/status registers with the right settings for this secondary device
            fn setup(&self) {
                // set up control register
                self.peripheral.spcr.write(|w| {
                    // enable SPI
                    w.spe().set_bit();
                    // Set to primary mode
                    w.mstr().set_bit();
                    // set up data order control bit
                    match self.settings.data_order {
                        DataOrder::MostSignificantFirst => w.dord().clear_bit(),
                        DataOrder::LeastSignificantFirst => w.dord().set_bit(),
                    };
                    // set up polarity control bit
                    match self.settings.mode.polarity {
                        spi::Polarity::IdleHigh => w.cpol().set_bit(),
                        spi::Polarity::IdleLow => w.cpol().clear_bit(),
                    };
                    // set up phase control bit
                    match self.settings.mode.phase {
                        spi::Phase::CaptureOnFirstTransition => w.cpha().clear_bit(),
                        spi::Phase::CaptureOnSecondTransition => w.cpha().set_bit(),
                    };
                    // set up clock rate control bit
                    match self.settings.clock {
                        SerialClockRate::OscfOver2 => w.spr().val_0x00(),
                        SerialClockRate::OscfOver4 => w.spr().val_0x00(),
                        SerialClockRate::OscfOver8 => w.spr().val_0x01(),
                        SerialClockRate::OscfOver16 => w.spr().val_0x01(),
                        SerialClockRate::OscfOver32 => w.spr().val_0x02(),
                        SerialClockRate::OscfOver64 => w.spr().val_0x02(),
                        SerialClockRate::OscfOver128 => w.spr().val_0x03(),
                    }
                });
                // set up 2x clock rate status bit
                self.peripheral.spsr.write(|w| match self.settings.clock {
                    SerialClockRate::OscfOver2 => w.spi2x().set_bit(),
                    SerialClockRate::OscfOver4 => w.spi2x().clear_bit(),
                    SerialClockRate::OscfOver8 => w.spi2x().set_bit(),
                    SerialClockRate::OscfOver16 => w.spi2x().clear_bit(),
                    SerialClockRate::OscfOver32 => w.spi2x().set_bit(),
                    SerialClockRate::OscfOver64 => w.spi2x().clear_bit(),
                    SerialClockRate::OscfOver128 => w.spi2x().clear_bit(),
                });
            }
        }

        /// FullDuplex trait implementation, allowing this struct to be provided to
        /// drivers that require it for operation.  Only 8-bit word size is supported
        /// for now.
        impl $crate::hal::spi::FullDuplex<u8> for $Spi {
            type Error = $crate::void::Void;

            /// Sets up the device for transmission and sends the data
            fn send(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                self.flush()?;
                self.write(byte);
                Ok(())
            }

            /// Reads and returns the response in the data register
            fn read(&mut self) -> $crate::nb::Result<u8, Self::Error> {
                self.flush()?;
                Ok(self.peripheral.spdr.read().bits())
            }
        }
    };
}
