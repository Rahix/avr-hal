//! SPI Implementation

/// Error type emitted by Spi in the event of a critical failure.  Errors have
/// no information attached.
#[derive(Debug, Clone, Copy)]
pub enum SpiError {}

/// Oscillator Clock Frequency division options.  Controls both SPR and SPI2X register bits.
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
pub enum DataOrder {
    MostSignificantFirst,
    LeastSignificantFirst,
}

/// Polarity of clock (whether SCLK idles at low state or high state)
pub enum SerialClockPolarity {
    IdleHigh,
    IdleLow,
}

/// Clock sampling phase (check at leading or trailing edge of signal)
pub enum SerialClockPhase {
    SampleLeading,
    SampleTrailing,
}

/// Implement traits for a SPI interface
#[macro_export]
macro_rules! impl_spi {
    (
        $(#[$spi_attr:meta])*
        pub struct $Spi:ident {
            peripheral: $SPI:ty,
            pins: {
                sclk: $SCLK:ident,
                posi: $POSI:ident,
                piso: $PISO:ident,
            }
        }
    ) => {
        type SCLK = $SCLK<mode::Output>;
        type POSI = $POSI<mode::Output>;
        type PISO = $PISO<mode::Input<mode::PullUp>>;

        /// Settings to pass to Spi.
        ///
        /// Easiest way to initialize is with
        /// `Settings::default()`.  Otherwise can be instantiated with alternate
        /// settings directly.
        pub struct Settings {
            pub data_order: DataOrder,
            pub clock: SerialClockRate,
            pub clock_polarity: SerialClockPolarity,
            pub clock_phase: SerialClockPhase,
        }

        impl Default for Settings {
            fn default() -> Self {
                Settings {
                    data_order: DataOrder::MostSignificantFirst,
                    clock: SerialClockRate::OscfOver4,
                    clock_polarity: SerialClockPolarity::IdleLow,
                    clock_phase: SerialClockPhase::SampleTrailing,
                }
            }
        }

        /// Behavior for a SPI interface.
        ///
        /// Stores the SPI peripheral for register access.  In addition, it takes
        /// ownership of the POSI and PISO pins to ensure they are in the correct mode.
        /// Instantiate with the `new` method.
        $(#[$spi_attr])*
        pub struct $Spi {
            peripheral: $SPI,
            sclk: SCLK,
            posi: POSI,
            piso: PISO,
            settings: Settings,
        }

        /// Implementation-specific behavior of the struct, including setup/tear-down
        impl $Spi {
            /// Instantiate an SPI with the registers, SCLK/POSI/PISO pins, and settings
            ///
            /// The pins are not actually used directly, but they are accepted in order to enforce
            /// that they are in the correct mode.
            pub fn new(peripheral: $SPI, sclk: SCLK, posi: POSI, piso: PISO, settings: Settings) -> $Spi {
                Spi {
                    peripheral,
                    sclk,
                    posi,
                    piso,
                    settings,
                }
            }

            /// Release ownership of the peripheral and pins.  Instance can no-longer
            /// be used after this is invoked.
            pub fn release(self) -> ($SPI, SCLK, POSI, PISO) {
                (self.peripheral, self.sclk, self.posi, self.piso)
            }

            /// Write a byte to the data register, which begins transmission
            /// automatically
            fn write(&self, byte: u8) {
                self.peripheral.spdr.write(|w| w.bits(byte));
            }

            /// Loop forever, checking the transmission complete bit until it is set
            fn block_until_transfer_complete(&self) {
                while self.peripheral.spsr.read().spif().bit_is_clear() { }
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
                    match self.settings.clock_polarity {
                        SerialClockPolarity::IdleHigh => w.cpol().set_bit(),
                        SerialClockPolarity::IdleLow => w.cpol().clear_bit(),
                    };
                    // set up phase control bit
                    match self.settings.clock_phase {
                        SerialClockPhase::SampleLeading => w.cpha().clear_bit(),
                        SerialClockPhase::SampleTrailing => w.cpha().set_bit(),
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
                    SerialClockRate::OscfOver128 => w.spi2x().set_bit(),
                });
            }
        }

        /// FullDuplex trait implementation, allowing this struct to be provided to
        /// drivers that require it for operation.  Only 8-bit word size is supported
        /// for now.
        impl $crate::hal::spi::FullDuplex<u8> for $Spi {
            type Error = $crate::spi::SpiError;

            /// Sets up the device for transmission and sends the data
            fn send(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                self.setup();
                self.write(byte);
                self.block_until_transfer_complete();
                Ok(())
            }

            /// Reads and returns the response in the data register
            fn read(&mut self) -> $crate::nb::Result<u8, Self::Error> {
                Ok(self.peripheral.spdr.read().bits())
            }
        }
    };
}
