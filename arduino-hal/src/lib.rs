#![no_std]
#![feature(doc_cfg)]

//! `arduino-hal`
//! =============
//! Common HAL (hardware abstraction layer) for Arduino boards.
//!
//! **Note**: This version of the documentation was built for
#![cfg_attr(feature = "arduino-diecimila", doc = "**Arduino Diecimila**.")]
#![cfg_attr(feature = "arduino-leonardo", doc = "**Arduino Leonardo**.")]
#![cfg_attr(feature = "arduino-mega2560", doc = "**Arduino Mega 2560**.")]
#![cfg_attr(feature = "arduino-mega1280", doc = "**Arduino Mega 1280**.")]
#![cfg_attr(feature = "arduino-nano", doc = "**Arduino Nano**.")]
#![cfg_attr(feature = "arduino-uno", doc = "**Arduino Uno**.")]
#![cfg_attr(feature = "sparkfun-promicro", doc = "**SparkFun ProMicro**.")]
#![cfg_attr(feature = "trinket-pro", doc = "**Trinket Pro**.")]
#![cfg_attr(feature = "trinket", doc = "**Trinket**.")]
#![cfg_attr(feature = "digispark", doc = "**Digispark kickstarter attiny85**.")]
#![cfg_attr(feature = "nano168", doc = "**Nano clone (ATmega168)**.")]
//! This means that only items which are available for this board are visible.  If you are using a
//! different board, try building the documentation locally with
//!
//! ```text
//! cargo doc --open
//! ```
//!
//! in your project (where `arduino-hal` is included with the feature-flag for your board).
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

#[cfg(not(feature = "board-selected"))]
compile_error!(
    "This crate requires you to specify your target Arduino board as a feature.

    Please select one of the following

    * arduino-diecimila
    * arduino-leonardo
    * arduino-mega2560
    * arduino-mega1280
    * arduino-nano
    * arduino-uno
    * digispark
    * sparkfun-promicro
    * trinket-pro
    * trinket
    * nano168
    "
);

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

#[doc(no_inline)]
#[cfg(feature = "mcu-atmega")]
pub use atmega_hal as hal;
#[doc(no_inline)]
#[cfg(feature = "mcu-atmega")]
pub use atmega_hal::pac;

#[doc(no_inline)]
#[cfg(feature = "mcu-attiny")]
pub use attiny_hal as hal;
#[doc(no_inline)]
#[cfg(feature = "mcu-attiny")]
pub use attiny_hal::pac;

#[doc(no_inline)]
#[cfg(feature = "board-selected")]
pub use hal::Peripherals;

#[cfg(feature = "board-selected")]
pub mod clock;
#[cfg(feature = "board-selected")]
pub use clock::default::DefaultClock;

#[cfg(feature = "board-selected")]
mod delay;
#[cfg(feature = "board-selected")]
pub use delay::{delay_ms, delay_us, Delay};

#[cfg(feature = "board-selected")]
pub mod port;

#[cfg(feature = "board-selected")]
pub mod simple_pwm;

#[doc(no_inline)]
#[cfg(feature = "board-selected")]
pub use port::Pins;

/// Analog to Digital converter.
#[cfg(feature = "mcu-atmega")]
pub mod adc {
    pub use crate::hal::adc::{
        channel, AdcChannel, AdcOps, AdcSettings, Channel, ClockDivider, ReferenceVoltage,
    };

    /// Check the [`avr_hal_generic::adc::Adc`] documentation.
    pub type Adc = crate::hal::Adc<crate::DefaultClock>;
}
#[doc(no_inline)]
#[cfg(feature = "mcu-atmega")]
pub use adc::Adc;

/// I2C bus controller.
#[cfg(feature = "mcu-atmega")]
pub mod i2c {
    pub use crate::hal::i2c::*;

    pub type I2c = crate::hal::i2c::I2c<crate::DefaultClock>;
}
#[doc(no_inline)]
#[cfg(feature = "mcu-atmega")]
pub use i2c::I2c;

/// SPI controller.
#[cfg(feature = "mcu-atmega")]
pub mod spi {
    pub use crate::hal::spi::*;

    pub type Spi = crate::hal::spi::Spi;
}
#[doc(no_inline)]
#[cfg(feature = "mcu-atmega")]
pub use spi::Spi;

#[cfg(feature = "mcu-atmega")]
pub mod usart {
    pub use crate::hal::usart::{Baudrate, UsartOps};

    pub type Usart<USART, RX, TX> = crate::hal::usart::Usart<USART, RX, TX, crate::DefaultClock>;
    pub type UsartWriter<USART, RX, TX> =
        crate::hal::usart::UsartWriter<USART, RX, TX, crate::DefaultClock>;
    pub type UsartReader<USART, RX, TX> =
        crate::hal::usart::UsartReader<USART, RX, TX, crate::DefaultClock>;
}

#[cfg(feature = "board-selected")]
pub mod eeprom {
    pub use crate::hal::eeprom::{Eeprom, EepromOps, OutOfBoundsError};
}
#[doc(no_inline)]
#[cfg(feature = "board-selected")]
pub use eeprom::Eeprom;

#[doc(no_inline)]
#[cfg(feature = "mcu-atmega")]
pub use usart::Usart;

#[cfg(feature = "mcu-atmega")]
pub mod prelude {
    pub use crate::hal::prelude::*;

    cfg_if::cfg_if! {
        if #[cfg(any(
            feature = "arduino-diecimila",
            feature = "arduino-mega2560",
            feature = "arduino-mega1280",
            feature = "arduino-uno"
        ))] {
            pub use crate::hal::usart::BaudrateArduinoExt as _;
        } else {
            pub use crate::hal::usart::BaudrateExt as _;
        }
    }
}

/// Convenience macro to instanciate the [`Pins`] struct for this board.
///
/// # Example
/// ```no_run
/// let dp = arduino_hal::Peripherals::take().unwrap();
/// let pins = arduino_hal::pins!(dp);
/// ```
#[cfg(feature = "board-selected")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::with_mcu_pins($crate::hal::pins!($p))
    };
}

#[cfg(any(feature = "arduino-leonardo"))]
#[macro_export]
macro_rules! default_serial {
    ($p:expr, $pins:expr, $baud:expr) => {
        $crate::Usart::new(
            $p.USART1,
            $pins.d0,
            $pins.d1.into_output(),
            $crate::hal::usart::BaudrateExt::into_baudrate($baud),
        )
    };
}
#[cfg(any(feature = "sparkfun-promicro"))]
#[macro_export]
macro_rules! default_serial {
    ($p:expr, $pins:expr, $baud:expr) => {
        $crate::Usart::new(
            $p.USART1,
            $pins.rx,
            $pins.tx.into_output(),
            $crate::hal::usart::BaudrateExt::into_baudrate($baud),
        )
    };
}
// See comment in avr-hal-generic/src/usart.rs for why these boards use
// the BaudrateArduinoExt trait instead of BaudrateExt
#[cfg(any(
    feature = "arduino-diecimila",
    feature = "arduino-mega2560",
    feature = "arduino-mega1280",
    feature = "arduino-uno"
))]
#[macro_export]
macro_rules! default_serial {
    ($p:expr, $pins:expr, $baud:expr) => {
        $crate::Usart::new(
            $p.USART0,
            $pins.d0,
            $pins.d1.into_output(),
            $crate::hal::usart::BaudrateArduinoExt::into_baudrate($baud),
        )
    };
}
#[cfg(any(feature = "arduino-nano", feature = "nano168"))]
#[macro_export]
macro_rules! default_serial {
    ($p:expr, $pins:expr, $baud:expr) => {
        $crate::Usart::new(
            $p.USART0,
            $pins.d0,
            $pins.d1.into_output(),
            $crate::hal::usart::BaudrateExt::into_baudrate($baud),
        )
    };
}
