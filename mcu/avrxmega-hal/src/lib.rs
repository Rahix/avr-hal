#![no_std]

//! `avrxmega-hal`
//! =============
//! Common HAL (hardware abstraction layer) for AVR XMEGA microcontrollers.
//!
//! **Note**: This version of the documentation was built for
#![cfg_attr(feature = "attiny404", doc = "**ATtiny404**.")]
//! This means that only items which are available for this MCU are visible.  If you are using
//! a different chip, try building the documentation locally with:
//!
//! ```text
//! cargo doc --features <your-mcu> --open
//! ```

#[cfg(all(
    not(feature = "device-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "This crate requires you to specify your target chip as a feature.

    Please select one of the following

    * attiny404
    "
);

/// Reexport of `attiny404` from `avr-device`
#[cfg(feature = "attiny404")]
pub use avr_device::attiny404 as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

#[cfg(feature = "device-selected")]
pub use pac::Peripherals;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;
pub use avr_hal_generic::prelude;

#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub use port::Pins;

#[cfg(feature = "attiny404")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB)
    };
}
