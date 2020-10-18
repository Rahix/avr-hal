#![no_std]

// Expose hal & pac crates
pub use atmega168_hal as hal;
pub use crate::hal::pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use crate::hal::entry;

mod pins;

pub use crate::pac::Peripherals;
pub use crate::pins::*;
pub use crate::hal::adc;
pub use crate::hal::prelude;
pub use crate::hal::pwm;
pub use crate::hal::spi;

pub type Delay = crate::hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = crate::hal::usart::Usart0<hal::clock::MHz16, IMODE>;
pub type I2c<M> = crate::hal::i2c::I2c<hal::clock::MHz16, M>;
