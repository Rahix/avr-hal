#![no_std]

pub extern crate atmega2560_hal as hal;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use hal::entry;

mod pins;

pub use atmega2560_hal::atmega2560;
pub use crate::atmega2560::Peripherals;
pub use atmega2560_hal::prelude;
pub use atmega2560_hal::spi;
pub use atmega2560_hal::adc;
pub use crate::pins::*;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = atmega2560_hal::usart::Usart0<hal::clock::MHz16, IMODE>;
pub type I2c<M> = hal::i2c::I2c<hal::clock::MHz16, M>;
