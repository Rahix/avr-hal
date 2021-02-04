#![no_std]

/// Reexport of `attiny861` from `avr-device`
pub use avr_device::attiny861 as pac;

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

pub mod i2c {
	use crate::port::{portc, mode};
	struct I2c {
		peripheral: crate::pac::USI,
		sda: portb::PA0<mode::Input<mode::Floating>>,
		scl: portb::PA2<mode::Input<mode::Floating>>,
		speed: u32,
	}
}
