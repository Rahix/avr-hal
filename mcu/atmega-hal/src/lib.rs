#![no_std]

//! `atmega-hal`
//! =============
//! Common HAL (hardware abstraction layer) for ATmega* microcontrollers.
//!

#[cfg(all(
    not(feature = "_mcu-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "You must specify your target chips as a feature.

    Please select at least one of the following features

    * atmega48p
    * atmega164pa
    * atmega168
    * atmega328p
    * atmega328pb
    * atmega32a
    * atmega32u4
    * atmega2560
    * atmega128a
    * atmega1280
    * atmega1284p
    * atmega8
    "
);

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;
pub use avr_hal_generic::prelude;

#[cfg(feature = "_mcu-atmega48p")]
pub mod atmega48p;

#[cfg(feature = "_mcu-atmega164pa")]
pub mod atmega164pa;

#[cfg(feature = "_mcu-atmega168")]
pub mod atmega168;

#[cfg(feature = "_mcu-atmega328p")]
pub mod atmega328p;

#[cfg(feature = "_mcu-atmega328pb")]
pub mod atmega328pb;

#[cfg(feature = "_mcu-atmega32a")]
pub mod atmega32a;

#[cfg(feature = "_mcu-atmega32u4")]
pub mod atmega32u4;

#[cfg(feature = "_mcu-atmega2560")]
pub mod atmega2560;

#[cfg(feature = "_mcu-atmega128a")]
pub mod atmega128a;

#[cfg(feature = "_mcu-atmega1280")]
pub mod atmega1280;

#[cfg(feature = "_mcu-atmega1284p")]
pub mod atmega1284p;

#[cfg(feature = "_mcu-atmega8")]
pub mod atmega8;


pub(crate) mod r#impl;

