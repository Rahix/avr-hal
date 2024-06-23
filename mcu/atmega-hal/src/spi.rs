#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::spi::*;

#[cfg(any(
    feature = "atmega128a",
    feature = "atmega1280",
    feature = "atmega2560",
    feature = "atmega32u4"
))]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Atmega,
    crate::pac::SPI,
    port::PB1,
    port::PB2,
    port::PB3,
    port::PB0,
>;
#[cfg(any(
    feature = "atmega128a",
    feature = "atmega1280",
    feature = "atmega2560",
    feature = "atmega32u4"
))]
avr_hal_generic::impl_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::SPI,
    sclk: port::PB1,
    mosi: port::PB2,
    miso: port::PB3,
    cs: port::PB0,
}

#[cfg(any(
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega48p",
    feature = "atmega8"
))]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Atmega,
    crate::pac::SPI,
    port::PB5,
    port::PB3,
    port::PB4,
    port::PB2,
>;
#[cfg(any(
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega48p",
    feature = "atmega8"
))]
avr_hal_generic::impl_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::SPI,
    sclk: port::PB5,
    mosi: port::PB3,
    miso: port::PB4,
    cs: port::PB2,
}

#[cfg(feature = "atmega328pb")]
pub type Spi0 = avr_hal_generic::spi::Spi<
    crate::Atmega,
    crate::pac::SPI0,
    port::PB5,
    port::PB3,
    port::PB4,
    port::PB2,
>;
#[cfg(feature = "atmega328pb")]
avr_hal_generic::impl_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::SPI0,
    sclk: port::PB5,
    mosi: port::PB3,
    miso: port::PB4,
    cs: port::PB2,
}
#[cfg(feature = "atmega328pb")]
pub type Spi1 = avr_hal_generic::spi::Spi<
    crate::Atmega,
    crate::pac::SPI1,
    port::PC1,
    port::PE3,
    port::PC0,
    port::PE2,
>;
#[cfg(feature = "atmega328pb")]
avr_hal_generic::impl_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::SPI1,
    sclk: port::PC1,
    mosi: port::PE3,
    miso: port::PC0,
    cs: port::PE2,
}

#[cfg(any(feature = "atmega1284p", feature = "atmega32a"))]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Atmega,
    crate::pac::SPI,
    port::PB7,
    port::PB5,
    port::PB6,
    port::PB4,
>;
#[cfg(any(feature = "atmega1284p", feature = "atmega32a"))]
avr_hal_generic::impl_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::SPI,
    sclk: port::PB7,
    mosi: port::PB5,
    miso: port::PB6,
    cs: port::PB4,
}
