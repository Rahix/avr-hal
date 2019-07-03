

use embedded_hal as hal;
use nb;

#[derive(Debug, Clone, Copy)]
pub enum Error { }

pub struct Spi<SS> where
    SS: hal::digital::v2::OutputPin
{
    ss: SS,
    // TODO add necessary properties
}

impl<SS> Spi<SS> where
    SS: hal::digital::v2::OutputPin
{
    // TODO add settings arguments besides secondary select (optional?)
    /// Initialize the SPI peripheral
    pub fn new(mut ss: SS) -> Spi<SS> {
        // start by closing communication with secondary
        ss.set_high();
        // TODO control, status, and register pins to struct
        Spi {
            ss: ss
        }
    }
}

impl<SS> hal::spi::FullDuplex<u8> for Spi<SS> where
    SS: hal::digital::v2::OutputPin
{
    type Error = Error;

    fn send(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        // I think it would be best to set all control bits for every write.  This way the user can have
        // multiple Spi instances that communicate with different secondaries with no problem, even if they
        // each have different settings.
        // make sure the entire control register is set in one instruction for efficiency
        // registers have modify/read/write/reset methods

        // open communication with secondary via secondary-select pin
        self.ss.set_low();

        // pull SS (instance of embedded_hal::serial::v2::OutputPin) low
        // set SPIE (SPI enable) control bit to 1
        // set MSTR (primary/secondary select) control bit to 1

        // set DORD (data order) control bit to user-defined setting (default 1)
        // set CPOL (clock polarity) control bit to user-defined setting (default 0)
        // set CPHA (clock phase) control bit to user-defined setting (default 0)
        // set SPR (clock speed) control bits to user-defined setting (default 3)
        // set SPIX2 (x2 clock speed) status bit to user-defined setting (default 0)

        // set $data to byte

        // close communication with secondary via secondary-select pin
        self.ss.set_high();
        Ok(())
    }

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        // return and dereference $data
        Ok(0)
    }
}