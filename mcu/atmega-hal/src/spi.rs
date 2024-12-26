//! SPI
//!
//! # Example
//!
//! Complete example source code can be found in the repository
//! [`atmega2560-spi-feedback.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-spi-feedback.rs)
//!
//! ```
//! let dp = atmega_hal::Peripherals::take().unwrap();
//! let pins = atmega_hal::pins!(dp);
//!
//! let (mut spi, mut cs) = spi::Spi::new(
//!     dp.SPI,
//!     pins.pb1.into_output(),
//!     pins.pb2.into_output(),
//!     pins.pb3.into_pull_up_input(),
//!     pins.pb0.into_output(),
//!     spi::Settings::default(),
//! );
//!
//! let data_out = b"Hello World!";
//! let mut data_in = [0u8; 12];
//!
//! cs.set_low().unwrap();
//! spi.transfer(&mut data_in, data_out).unwrap();
//! cs.set_high().unwrap();
//!
//! ufmt::uwriteln!(&mut serial, "data: {:?}", data_in).unwrap();
//! ```

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
