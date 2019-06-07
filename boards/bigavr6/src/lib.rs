#![no_std]

pub extern crate atmega1280_hal as hal;

mod pins;

pub use atmega1280_hal::atmega1280;
pub use crate::atmega1280::Peripherals;
pub use atmega1280_hal::prelude;
pub use crate::pins::*;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = atmega1280_hal::usart::Usart1<hal::clock::MHz16, IMODE>;
