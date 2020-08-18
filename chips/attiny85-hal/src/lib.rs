#![no_std]

extern crate avr_hal_generic as avr_hal;

pub use avr_device::attiny85;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;
/// See [`avr_device::interrupt`](https://docs.rs/avr-device/latest/avr_device/attr.interrupt.html).
#[cfg(feature = "rt")]
pub use avr_device::interrupt;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;

pub mod prelude {
    pub use crate::avr_hal::prelude::*;
    pub use crate::port::PortExt as _;
}
