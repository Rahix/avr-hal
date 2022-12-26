//! MCU core clock support.
//!
//! This module contains common definitions to abtract over the MCU core clock speed.  `avr-hal`
//! does not support changing the clock-speed at runtime.
//!
//! Most items in this module are re-exported from [`avr_hal_generic::clock`].
pub use avr_hal_generic::clock::*;

pub(crate) mod default {
    /// Default clock speed for this board.
    ///
    /// `arduino-hal` contains a lot of type aliases for assuming this clock speed.  As such it is
    /// easiest to keep the processor at the selected default speed.
    ///
    /// However, you can of course still use other clock speeds but you'll then need to correctly
    /// name the types from the HAL crate using your own clock definition.
    #[cfg(any(
        feature = "arduino-diecimila",
        feature = "arduino-leonardo",
        feature = "arduino-mega2560",
        feature = "arduino-mega1280",
        feature = "arduino-nano",
        feature = "arduino-uno",
        feature = "sparkfun-promicro",
        feature = "trinket-pro",
        feature = "nano168",
        feature = "digispark",
    ))]
    pub type DefaultClock = avr_hal_generic::clock::MHz16;
    #[cfg(feature = "trinket")]
    pub type DefaultClock = avr_hal_generic::clock::MHz8;
}
