//! SPI Implementation

/// SPI Error
#[derive(Debug, Clone, Copy)]
pub enum Error { }

/// Implement traits for a SPI interface
#[macro_export]
macro_rules! impl_spi {
    (
        $(#[$spi_attr:meta])*
        pub struct $Spi:ident {
            peripheral: $SPI:ty,
            pins: {
                // Might not need references to these pins?  Seems like r/w and clock are handled by hardware.
                clock: $CLOCK:ident,
                piso: $PISO:ident,
                posi: $POSI:ident,
            },
            registers: {
                control: $control:ident,
                status: $status:ident,
                data: $data:ident,
            },
        }
    ) => {
        $(#[$spi_attr])*
        pub struct $Spi {
            // TODO add necessary properties
        }

        impl $Spi
        {
            // TODO add settings arguments besides secondary select (optional?)
            /// Initialize the SPI peripheral
            pub fn new(ss: $crate::hal::digital::v2::OutputPin) -> $Spi {
                // pull SS high
                // store secondary-select, control, status, and register pins to struct
                $Spi {}
            }
        }

        impl $crate::hal::spi::FullDuplex<u8> for $Spi {
            type Error = $crate::spi::Error;

            fn write(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                // I think it would be best to set all control bits for every write.  This way the user can have
                // multiple Spi instances that communicate with different secondaries with no problem, even if they
                // each have different settings.
                // make sure the entire control register is set in one instruction for efficiency
                // registers have modify/read/write/reset methods

                // pull SS (instance of embedded_hal::serial::v2::OutputPin) low
                // set SPIE (SPI enable) control bit to 1
                // set MSTR (primary/secondary select) control bit to 1

                // set DORD (data order) control bit to user-defined setting (default 1)
                // set CPOL (clock polarity) control bit to user-defined setting (default 0)
                // set CPHA (clock phase) control bit to user-defined setting (default 0)
                // set SPR (clock speed) control bits to user-defined setting (default 3)
                // set SPIX2 (x2 clock speed) status bit to user-defined setting (default 0)

                // set $data to byte

                // pull SS high
                Ok(())
            }

            fn read(&mut self) -> $crate::nb::Result<u8, Self::Error> {
                // return and dereference $data
                Ok(0)
            }
        }
    };
}
