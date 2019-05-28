#![no_std]

pub extern crate atmega32u4_hal as hal;

mod pins;

pub use atmega32u4_hal::atmega32u4;
pub use crate::atmega32u4::Peripherals;
pub use atmega32u4_hal::prelude;
pub use crate::pins::*;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = atmega32u4_hal::usart::Usart1<hal::clock::MHz16, IMODE>;
