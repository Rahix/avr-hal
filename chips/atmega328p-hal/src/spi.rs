

use embedded_hal as hal;
use nb;
use crate::atmega328p::SPI;

#[derive(Debug, Clone, Copy)]
pub enum SpiError { }

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

pub enum DataOrder {
    MostSignificantFirst,
    LeastSignificantFirst,
}

pub enum SerialClockPolarity {
    IdleHigh,
    IdleLow,
}

pub enum SerialClockPhase {
    SampleLeading,
    SampleTrailing,
}

pub struct Settings {
    data_order: DataOrder,
    clock: SerialClockRate,
    clock_polarity: SerialClockPolarity,
    clock_phase: SerialClockPhase,
}

pub struct Spi<SS> where
    SS: hal::digital::v2::OutputPin<Error = ()>,
{
    peripheral: SPI,
    secondary_select: SS,
    settings: Settings,
    // TODO add necessary properties
}

impl<SS> Spi<SS> where
    SS: hal::digital::v2::OutputPin<Error = ()>,
{
    // TODO add settings arguments besides secondary select (optional?)
    /// Initialize the SPI peripheral
    pub fn new(peripheral: SPI, mut secondary_select: SS, settings: Settings) -> Spi<SS> {
        // start by closing communication with secondary
        secondary_select.set_high().unwrap();
        // TODO control, status, and register pins to struct
        Spi {
            peripheral,
            secondary_select,
            settings,
        }
    }
}

impl<SS> hal::spi::FullDuplex<u8> for Spi<SS> where
    SS: hal::digital::v2::OutputPin<Error = ()>,
{
    type Error = SpiError;

    /// Sets up control/status register before writing data to ensure settings are always correct
    fn send(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        // open communication with secondary via secondary-select pin
        self.secondary_select.set_low().unwrap();

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

        // write byte to data register which triggers transmission
        self.peripheral.spdr.write(|w| w.bits(byte));

        // wait until transmission is complete
        while self.peripheral.spsr.read().spif().bit_is_clear() { }

        // close communication with secondary via secondary-select pin

        self.secondary_select.set_high().unwrap();
        Ok(())
    }

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        Ok(self.peripheral.spdr.read().bits())
    }
}