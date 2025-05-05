//! SPI
//!
//! # Example
//!
//! For full source code, please refer to the ATmega SPI example:
//! [`atmega2560-spi-feedback.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-spi-feedback.rs)
//!
//! ```
//! let dp = attiny_hal::Peripherals::take().unwrap();
//! let pins = attiny_hal::pins!(dp);
//!
//! let (mut spi, mut cs) = spi::Spi::new(
//!     dp.SPI,
//!     pins.pa4.into_output(),
//!     pins.pa6.into_output(),
//!     pins.pa5.into_pull_up_input(),
//!     pins.pa3.into_output(),
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
