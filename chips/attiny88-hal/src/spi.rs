//! Implementation of the Rust Embedded-HAL SPI FullDuplex trait for AVR.
//!
//! The interface can be instantiated with the `new` method, and used directly
//! or passed into a driver.  Example usage:
//!
//! ```
//! portb::PB2.into_output(&mut pins.ddr);// SS must be set to output mode
//! // create SPI interface
//! let mut spi = Spi::new(
//!     dp.SPI,// SPI peripheral
//!     portb::PB5.into_output(&mut pins.ddr),// SCLK
//!     portb::PB3.into_output(&mut pins.ddr),// MOSI output pin
//!     portb::PB4.into_pull_up_input(&mut pins.ddr),// MISO input pin
//!     Settings::default(),
//! );
//!
//! // Send a byte
//! let sent = 0b10101010;
//! spi.send(sent).unwrap();
//! let response = spi.read().unwrap();
//! ```
//! In the example above, all of the settings are left at the default.  You can
//! also instantiate a Settings object with the other options available.

use crate::port::portb;
pub use avr_hal::spi::*;

avr_hal::impl_spi! {
    pub struct Spi {
        peripheral: crate::attiny88::SPI,
        pins: {
            sclk: portb::PB5,
            mosi: portb::PB3,
            miso: portb::PB4,
        }
    }
}
