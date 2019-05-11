extern crate avr_hal_generic as avr_hal;
pub extern crate avr_device as attiny85;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;
