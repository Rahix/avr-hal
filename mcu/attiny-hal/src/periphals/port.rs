//! Port
//!
//! # Example
//!
//! For full source code, please refer to the ATmega port example:
//! [`atmega2560-blink.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-blink.rs)
//!
//! ```
//! let dp = attiny_hal::Peripherals::take().unwrap();
//! let pins = attiny_hal::pins!(dp);
//!
//! let mut led = pins.pb2.into_output();
//!
//! loop {
//!     led.toggle();
//!     delay_ms(1000);
//! }
//! ```

pub use avr_hal_generic::port::{mode, PinMode, PinOps};
