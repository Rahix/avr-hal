#![no_std]

#[cfg(all(
    not(feature = "device-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "This crate requires you to specify your target chip as a feature.

    Please select one of the following

    * attiny85
    "
);

/// Reexport of `attiny85` from `avr-device`
#[cfg(feature = "attiny85")]
pub use avr_device::attiny85 as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

#[cfg(feature = "device-selected")]
pub use pac::Peripherals;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;

#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub use port::Pins;

pub struct ATtiny;

#[cfg(feature = "attiny85")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTB)
    };
}
