//! Time-keeping facilities
//!
//! This module allows you to use a timer peripheral to implement a clock that
//! gives the wall time since the start of the Arduino.
//! This clock gives essentially the same functionality as the `millis` and
//! `micros` functions of the C++ Arduino framework.
//! However, unlike the C++ framework, you have to initialize this facility
//! yourself, but you can choose which timer will be used.
//!
//! # Example
//!
//!```
//! // Needed for the timer interrupt that is attached via `impl_timepiece`
//! #![feature(abi_avr_interrupt)]
//!
//! use arduino_hal::impl_timepiece;
//! use arduino_hal::prelude::*;
//! use arduino_hal::time::embedded_time::duration::Microseconds;
//! use arduino_hal::time::embedded_time::duration::Seconds;
//! use arduino_hal::time::embedded_time::fixed_point::FixedPoint;
//! use arduino_hal::time::Chronometer;
//! use core::convert::TryFrom;
//! use panic_halt as _;
//!
//! use embedded_hal::serial::Read;
//!
//! // Prepare `Timer0` to be used for time-keeping.
//! // This will define the configuration and attach its timer interrupt.
//! impl_timepiece! {
//!     pub timepiece MyTimepiece {
//!         // Selecting which hardware timer to use, here: TC0
//!         peripheral: Timer0,
//!         // Specifies the CPU clock rate, here: the default
//!         cpu_clock: arduino_hal::DefaultClock,
//!         // Specifies which type to use for storing the milliseconds counter
//!         millis: u32,
//!         // Specifies which type to use for storing the microseconds counter
//!         micros: u32,
//!         // Selecting the interval of the timer interrupt, affects the
//!         // resolution of the timestamps.
//!         resolution: arduino_hal::time::Resolution::MS_4,
//!     }
//! }
//!
//! #[arduino_hal::entry]
//! fn main() -> ! {
//!     // Take your peripherals
//!     let dp = arduino_hal::Peripherals::take().unwrap();
//!
//!     // Take the timer peripheral (TC0 for Timer0) and wrap it in our timepiece wrapper
//!     let timepiece = MyTimepiece::new(dp.TC0);
//!     // Initialize the time-keeping facility
//!     let clock = Chronometer::new(timepiece);
//!
//!     // Since the Chronometer relies on interrupts, we have to enable them
//!     // globally:
//!     unsafe {
//!         // SAFETY: This is not within `interrupt::free`
//!         avr_device::interrupt::enable();
//!     }
//!
//!     // Do something time consuming...
//!
//!     // Using the local `Chronometer` instance, it has up to microseconds
//!     // precision
//!     let time = clock.now().duration_since_epoch();
//!     let passed_us = Microseconds::<u32>::try_from(time).unwrap();
//!
//!     // Alternatively, you can use the static clock (it is statically
//!     // accessible), which has just milliseconds precision with whatever
//!     // resolution you configured your timepiece
//!     let time = MyTimepiece::CLOCK.now().duration_since_epoch();
//!     let passed_seconds = Seconds::<u32>::try_from(time).unwrap();
//! }
//! ```
//!

mod chrono;
#[doc(hidden)]
pub mod macros;
mod piece;

pub use chrono::Chronometer;
pub use chrono::StaticChronometer;
#[doc(hidden)] // Used in macros
pub use piece::update_timer;
pub use piece::Timepiece;

pub use avr_hal_generic::time::Prescaler;
pub use avr_hal_generic::time::Resolution;
pub use avr_hal_generic::time::TimingCircuitOps;

pub use embedded_time;

/// Supported timer devices
pub mod timers {
    pub use crate::hal::time::*;
}
