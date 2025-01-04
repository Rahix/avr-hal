#![no_std]

//! `attiny-hal`
//! =============
//! Common HAL (hardware abstraction layer) for ATtiny* microcontrollers.

#[cfg(all(
    not(feature = "_mcu-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "You must specify your target chips as a feature.

    Please select at least one of the following features

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

#[cfg(feature = "_mcu-attiny167")]
pub mod attiny167;
#[cfg(feature = "_mcu-attiny2313")]
pub mod attiny2313;
#[cfg(feature = "_mcu-attiny84")]
pub mod attiny84;
#[cfg(feature = "_mcu-attiny85")]
pub mod attiny85;
#[cfg(feature = "_mcu-attiny88")]
pub mod attiny88;

