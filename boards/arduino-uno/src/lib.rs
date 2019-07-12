#![no_std]

pub extern crate atmega328p_hal as hal;

mod pins;

pub use atmega328p_hal::atmega328p;
pub use crate::atmega328p::Peripherals;
pub use atmega328p_hal::prelude;
pub use atmega328p_hal::spi;
pub use crate::pins::*;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = hal::usart::Usart0<hal::clock::MHz16, IMODE>;
pub type I2c<M> = hal::i2c::I2c<hal::clock::MHz16, M>;
