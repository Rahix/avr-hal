#![no_std]

pub extern crate atmega1280_hal as hal;

pub use atmega1280_hal::atmega1280;
pub use crate::atmega1280::Peripherals;
pub use atmega1280_hal::prelude;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = atmega1280_hal::usart::Usart0<hal::clock::MHz16, IMODE>;
pub type I2c<M> = hal::i2c::I2c<hal::clock::MHz16, M>;