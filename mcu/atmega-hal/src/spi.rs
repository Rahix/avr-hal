#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::spi::*;

#[cfg(feature = "atmega328p")]
pub type Spi<MisoInputMode> = avr_hal_generic::spi::Spi<
    crate::Atmega,
    crate::pac::SPI,
    port::Pin<port::mode::Output, port::PB5>,
    port::Pin<port::mode::Output, port::PB3>,
    port::Pin<port::mode::Input<MisoInputMode>, port::PB4>,
    port::Pin<port::mode::Output, port::PB2>,
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
