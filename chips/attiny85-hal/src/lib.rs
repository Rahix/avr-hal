#![no_std]

/// Reexport of `attiny85` from `avr-device`
pub use avr_device::attiny85 as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;

pub mod port;

pub mod prelude {
    pub use avr_hal_generic::prelude::*;
    pub use crate::port::PortExt as _;
}
