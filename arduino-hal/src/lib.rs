#![no_std]
#![feature(doc_cfg)]

//! `arduino-hal`
//! =============
//! Common HAL (hardware abstraction layer) for Arduino boards.
//!
//! ## Usage
//! For setting up a new project, the [`avr-hal-template`](https://github.com/Rahix/avr-hal-template)
//! is the recommended baseline.  Applications should be built ontop of the following skeleton:
//!
//! ```no_run
//! #![no_std]
//! #![no_main]
//!
//! use panic_halt as _;
//!
//! #[arduino_hal::entry]
//! fn main() -> ! {
//!     let dp = arduino_hal::Peripherals::take().unwrap();
//!     let pins = arduino_hal::pins!(dp);
//!
//!     loop { }
//! }
//! ```
//!
//! For examples, please check the `avr-hal` examples: <https://github.com/Rahix/avr-hal/tree/main/examples>

#[cfg(not(feature = "_board-selected"))]
compile_error!(
    "This crate requires you to specify your target Arduino board as a feature.

    Please select at least one of the following features

    * arduino-diecimila
    * arduino-leonardo
    * arduino-mega2560
    * arduino-mega1280
    * arduino-nano
    * arduino-uno
    * sparkfun-promicro
    * sparkfun-promini-3v3
    * sparkfun-promini-5v
    * trinket-pro
    * trinket
    * nano168
    "
);

pub(crate) mod r#impl;

pub mod adafruit;
pub mod arduino;
pub mod sparkfun;

/// Attribute to declare the entry point of the program
///
/// Exactly one entry point must be declared in the entire dependency tree.
///
/// ```
/// #[arduino_hal::entry]
/// fn main() -> ! {
///     // ...
/// }
/// ```
///
/// The entry function must have a signature of `[unsafe] fn() -> !`.
///
/// This macro is a reexport of [`avr_device::entry`].  It is only available when the `rt`
/// (runtime) feature is selected (it is by default).
#[cfg(any(feature = "rt", doc))]
#[doc(cfg(feature = "rt"))]
pub use avr_device::entry;

