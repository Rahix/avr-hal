#![no_std]

//! `avrmodern-hal`
//! =============
//! Common HAL (hardware abstraction layer) for modern AVR microcontrollers.
//!
//! **Note**: This version of the documentation was built for
#![cfg_attr(feature = "attiny402", doc = "**ATtiny402**.")]
#![cfg_attr(feature = "attiny1614", doc = "**ATtiny1614**.")]
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

    * attiny402
    * attiny1614
    "
);

#[cfg(feature = "attiny1614")]
pub use avr_device::attiny1614 as pac;
/// Reexport of `attiny402`, etc from `avr-device`
///
#[cfg(feature = "attiny402")]
pub use avr_device::attiny402 as pac;
// #[cfg(feature = "attiny3224")]
// pub use avr_device::attiny3224 as pac;
// #[cfg(feature = "avr128db28")]
// pub use avr_device::avr128db28 as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

#[cfg(feature = "device-selected")]
pub use pac::Peripherals;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;
pub use avr_hal_generic::prelude;

//#[cfg(feature = "device-selected")]
//pub mod adc;
//#[cfg(feature = "device-selected")]
//pub use adc::Adc;

//#[cfg(feature = "device-selected")]
//pub mod i2c;
//#[cfg(feature = "device-selected")]
//pub use i2c::I2c;

//#[cfg(feature = "device-selected")]
//pub mod spi;
//#[cfg(feature = "device-selected")]
//pub use spi::Spi;

#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub use port::Pins;

//#[cfg(feature = "device-selected")]
//pub mod simple_pwm;

#[cfg(feature = "device-selected")]
pub mod usart;
#[cfg(feature = "device-selected")]
pub use usart::Usart;

//#[cfg(feature = "device-selected")]
//pub mod wdt;
//#[cfg(feature = "device-selected")]
//pub use wdt::Wdt;

//#[cfg(feature = "device-selected")]
//pub mod eeprom;
//#[cfg(feature = "device-selected")]
//pub use eeprom::Eeprom;

pub struct Avrmodern;

#[cfg(any(feature = "attiny402",))]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA)
    };
}

#[cfg(any(feature = "attiny1614",))]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB)
    };
}
