

use embedded_hal as hal;
use nb;
use crate::atmega328p::SPI;

/// Oscillator Clock Frequency division options
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

/// Polarity of clock (rising edge is tick or falling edge)
pub enum SerialClockPolarity {
    IdleHigh,
    IdleLow,
}

/// Clock sampling phase (check at leading or trailing edge of signal)
pub enum SerialClockPhase {
    SampleLeading,
    SampleTrailing,
}

/// Settings to pass to Spi.  Easiest way to initialize is with `Settings::default()`
pub struct Settings {
    data_order: DataOrder,
    clock: SerialClockRate,
    clock_polarity: SerialClockPolarity,
    clock_phase: SerialClockPhase,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            data_order: DataOrder::MostSignificantFirst,
            clock: SerialClockRate::OscfOver4,
            clock_polarity: SerialClockPolarity::IdleLow,
            clock_phase: SerialClockPhase::SampleLeading,
        }
    }
}

/// Behavior for a SPI interface.  Stores the SPI peripheral along with a secondary-select pin and the settings
pub struct Spi<SS, OutputPinError> where
    SS: hal::digital::v2::OutputPin<Error = OutputPinError>
{
    peripheral: SPI,
    secondary_select: SS,
    settings: Settings,
}

/// General SPI methods for reading/wrigint
impl<SS, OutputPinError> Spi<SS, OutputPinError> where
    SS: hal::digital::v2::OutputPin<Error = OutputPinError>
{
    /// Instantiate an Spi interface
    pub fn new(peripheral: SPI, secondary_select: SS, settings: Settings) -> Result<Spi<SS, OutputPinError>, OutputPinError> {
        let mut instance = Spi {
            peripheral,
            secondary_select,
            settings,
        };
        instance.disable_secondary()?;
        Ok(instance)
    }

    /// Write a byte to the data register and begin transmission
    fn write(&self, byte: u8) {
        self.peripheral.spdr.write(|w| w.bits(byte));
    }

    /// Enable the secondary by settings its pin to low
    fn enable_secondary(&mut self) -> Result<(), OutputPinError> {
        self.secondary_select.set_low()
    }

    /// Disable the secondary by settings its pin to high
    fn disable_secondary(&mut self) -> Result<(), OutputPinError> {
        self.secondary_select.set_high()
    }

    /// Loop and keep checking that the SPI transmission is complete, returning when it has
    fn block_until_transfer_complete(&self) {
        while self.peripheral.spsr.read().spif().bit_is_clear() { }
    }

    /// Sets up the control/status registers with the right settings for this secondary device
    fn setup(&self) {
        // set up control register
        self.peripheral.spcr.write(|w| {
            w
                .spe().set_bit()// enable SPI
                .mstr().set_bit();// set to primary mode
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

impl<SS, OutputPinError> hal::spi::FullDuplex<u8> for Spi<SS, OutputPinError> where
    SS: hal::digital::v2::OutputPin<Error = OutputPinError>
{
    type Error = OutputPinError;

    /// Sets up the device for transmission and sends the data
    fn send(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.setup();
        self.enable_secondary()?;
        self.write(byte);
        self.block_until_transfer_complete();
        self.disable_secondary()?;
        Ok(())
    }

    /// Reads and returns the response in the data register
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        Ok(self.peripheral.spdr.read().bits())
    }
}