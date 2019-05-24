#![no_std]

extern crate avr_hal_generic as avr_hal;
pub extern crate avr_device as attiny85;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;

pub mod prelude {
    pub use crate::avr_hal::prelude::*;
    pub use crate::port::PortExt as _attiny_PortExt;
}
