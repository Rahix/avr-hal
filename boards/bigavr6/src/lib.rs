#![no_std]

// Expose hal & pac crates
pub use atmega1280_hal as hal;
pub use crate::hal::pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use crate::hal::entry;

pub use crate::pac::Peripherals;
pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::hal::usart::BaudrateExt as _;
}

pub type Delay = crate::hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = crate::hal::usart::Usart0<hal::clock::MHz16, IMODE>;
pub type I2c<M> = crate::hal::i2c::I2c<hal::clock::MHz16, M>;
