#![no_std]

//! `attiny-hal`
//! =============
//! Common HAL (hardware abstraction layer) for ATtiny* microcontrollers.
//!
//! **Note**: This version of the documentation was built for
#![cfg_attr(feature = "attiny85", doc = "**ATtiny85**.")]
#![cfg_attr(feature = "attiny88", doc = "**ATtiny88**.")]
#![cfg_attr(feature = "attiny167", doc = "**ATtiny167**.")]
#![cfg_attr(feature = "attiny2313", doc = "**ATtiny2313**.")]
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

    * attiny85
    * attiny88
    * attiny167
    * attiny2313
    "
);

#[cfg(feature = "attiny84")]
pub use avr_device::attiny84 as pac;

/// Reexport of `attiny85` from `avr-device`
#[cfg(feature = "attiny85")]
pub use avr_device::attiny85 as pac;

/// Reexport of `attiny88` from `avr-device`
#[cfg(feature = "attiny88")]
pub use avr_device::attiny88 as pac;

/// Reexport of `attiny167` from `avr-device`
#[cfg(feature = "attiny167")]
pub use avr_device::attiny167 as pac;

/// Reexport of `attiny2313` from `avr-device`
#[cfg(feature = "attiny2313")]
pub use avr_device::attiny2313 as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

#[cfg(feature = "device-selected")]
pub use pac::Peripherals;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;
pub use avr_hal_generic::prelude;

// ATtiny2313 does not have ADC and will not compile with this module
#[cfg(all(feature = "device-selected", not(feature = "attiny2313")))]
pub mod adc;
#[cfg(all(feature = "device-selected", not(feature = "attiny2313")))]
pub use adc::Adc;

#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub use port::Pins;

#[cfg(feature = "device-selected")]
pub mod simple_pwm;

pub struct Attiny;

#[cfg(feature = "attiny84")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB)
    };
}
#[cfg(feature = "attiny85")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTB)
    };
}
#[cfg(feature = "attiny88")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
    };
}
#[cfg(feature = "attiny167")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB)
    };
}
#[cfg(feature = "attiny2313")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB, $p.PORTD)
    };
}
