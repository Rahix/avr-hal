#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::spi::*;

#[cfg(feature = "attiny88")]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Attiny,
    crate::pac::SPI,
    port::PB5,
    port::PB3,
    port::PB4,
    port::PB2,
>;
#[cfg(feature = "attiny88")]
avr_hal_generic::impl_spi! {
    hal: crate::Attiny,
    peripheral: crate::pac::SPI,
    sclk: port::PB5,
    mosi: port::PB3,
    miso: port::PB4,
    cs: port::PB2,
}

#[cfg(feature = "attiny167")]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Attiny,
    crate::pac::SPI,
    port::PA5,
    port::PA4,
    port::PA2,
    port::PA6,
>;
#[cfg(feature = "attiny167")]
avr_hal_generic::impl_spi! {
    hal: crate::Attiny,
    peripheral: crate::pac::SPI,
    sclk: port::PA5,
    mosi: port::PA4,
    miso: port::PA2,
    cs: port::PA6,
}
