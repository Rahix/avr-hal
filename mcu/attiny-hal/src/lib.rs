#![no_std]

//! `attiny-hal`
//! =============
//! Common HAL (hardware abstraction layer) for ATtiny* microcontrollers.

// This crate can be configured in one of two ways: either you specify deprecated-globals and exactly one MCU
// Or you don't specify deprecated globals and at least one MCU
#[cfg(all(
    not(feature = "device-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "You must specify your target chip as a feature.

    Please select one of the following:

    * attiny84
    * attiny85
    * attiny88
    * attiny167
    * attiny2313
    "
);

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;
pub use avr_hal_generic::prelude;

pub(crate) mod r#impl;

#[cfg(feature = "attiny167")]
pub mod attiny167;
#[cfg(feature = "attiny2313")]
pub mod attiny2313;
#[cfg(feature = "attiny84")]
pub mod attiny84;
#[cfg(feature = "attiny85")]
pub mod attiny85;
#[cfg(feature = "attiny88")]
pub mod attiny88;

#[cfg(not(feature = "no-globals"))]
mod globals;
#[cfg(not(feature = "no-globals"))]
pub use globals::*;
