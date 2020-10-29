#![no_std]

pub extern crate atmega1280_hal as hal;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use hal::entry;

pub use atmega1280_hal::atmega1280;
pub use crate::atmega1280::Peripherals;
pub use atmega1280_hal::prelude;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = atmega1280_hal::usart::Usart0<hal::clock::MHz16, IMODE>;
pub type I2cMaster<M> = hal::i2c::I2cMaster<hal::clock::MHz16, M>;
pub type I2cSlave<M> = hal::i2c::I2cSlave<hal::clock::MHz16, M>;
