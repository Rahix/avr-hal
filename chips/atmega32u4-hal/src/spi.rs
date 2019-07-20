//! Implementation of the Rust Embedded-HAL SPI FullDuplex trait for AVR.
//!
//! The interface can be instantiated with the `new` method, and used directly
//! or passed into a driver.  Example usage:
//!
//! ```
//! pins.d10.into_output(&mut pins.ddr);// SS must be set to output mode
//! // create SPI interface
//! let mut spi = Spi::new(
//!     dp.SPI,// SPI peripheral
//!     pins.d11.into_output(&mut pins.ddr),// MOSI output pin
//!     pins.d12.into_pull_up_input(&mut pins.ddr),// MISO input pin
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

extern crate avr_hal_generic as avr_hal;

pub use avr_hal::spi::*;
use crate::port::{portb::PB1,portb::PB2,portb::PB3,mode};


avr_hal::impl_spi! {
    pub struct Spi {
        peripheral: crate::atmega32u4::SPI,
        pins: {
            sclk: PB1,
            posi: PB2,
            piso: PB3,
        }
    }
}
