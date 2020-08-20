#![no_std]

pub extern crate atmega168_hal as hal;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use hal::entry;

mod pins;

pub use crate::atmega168::Peripherals;
pub use crate::pins::*;
pub use atmega168_hal::adc;
pub use atmega168_hal::atmega168;
pub use atmega168_hal::prelude;
pub use atmega168_hal::pwm;
pub use atmega168_hal::spi;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = hal::usart::Usart0<hal::clock::MHz16, IMODE>;
pub type I2c<M> = hal::i2c::I2c<hal::clock::MHz16, M>;
