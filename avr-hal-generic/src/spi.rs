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
                clock: $clockmod:ident::$CLOCK:ident,
                piso: $pisomod:ident::$PISO:ident,
                posi: $posimod:ident::$POSI:ident,
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
                // TODO actually set up SPI peripheral
                $Spi {}
            }
        }

        impl $crate::hal::spi::FullDuplex<u8> for $Spi {
            type Error = $crate::spi::Error;

            fn write(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                // TODO implement write
                Ok(())
            }

            fn read(&mut self) -> $crate::nb::Result<u8, Self::Error> {
                // TODO implement read
                Ok(0)
            }
        }
    };
}
