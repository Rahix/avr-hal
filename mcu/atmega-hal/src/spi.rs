#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::spi::*;

#[cfg(feature = "atmega328p")]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Atmega,
    crate::pac::SPI,
    port::PB5,
    port::PB3,
    port::PB4,
    port::PB2,
>;
#[cfg(feature = "atmega328p")]
avr_hal_generic::impl_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::SPI,
    sclk: port::PB5,
    mosi: port::PB3,
    miso: port::PB4,
    cs: port::PB2,
}
