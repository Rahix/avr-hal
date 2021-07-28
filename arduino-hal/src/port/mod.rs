//! GPIO & Pin control.
//!
//! This module contains a [`Pins`] struct which represents all pins of the board.  The [`Pins`]
//! struct is most easily constructed using the [`arduino_hal::pins!()`][pins] macro:
//!
//! ```no_run
//! let dp = arduino_hal::Peripherals::take().unwrap();
//! let pins = arduino_hal::pins!(dp);
//! ```
//!
//! Additionally, the [`mode`] submodule contains all valid types for the `MODE` generic parameter
//! of a pin.  The [`Pin`] type-alias represents a pin which can represent _any_ of the pins
//! dynamically (while usually each pin has its own type).
//!
//! Check the documentation for [`avr_hal_generic::port::Pin`] for a detailed explanation of GPIO
//! pins in `avr-hal`.

#[cfg(feature = "arduino-diecimila")]
mod diecimila;
#[cfg(feature = "arduino-diecimila")]
pub use diecimila::*;
#[cfg(feature = "arduino-leonardo")]
mod leonardo;
#[cfg(feature = "arduino-leonardo")]
pub use leonardo::*;
#[cfg(feature = "arduino-mega2560")]
mod mega2560;
#[cfg(feature = "arduino-mega2560")]
pub use mega2560::*;
#[cfg(any(feature = "arduino-nano", feature = "arduino-uno", feature = "nano168"))]
mod uno;
#[cfg(any(feature = "arduino-nano", feature = "arduino-uno", feature = "nano168"))]
pub use uno::*;
#[cfg(feature = "sparkfun-promicro")]
mod promicro;
#[cfg(feature = "sparkfun-promicro")]
pub use promicro::*;
#[cfg(feature = "trinket-pro")]
mod trinket_pro;
#[cfg(feature = "trinket-pro")]
pub use trinket_pro::*;
#[cfg(feature = "trinket")]
mod trinket;
#[cfg(feature = "trinket")]
pub use trinket::*;
