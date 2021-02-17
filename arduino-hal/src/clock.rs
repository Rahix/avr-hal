pub use avr_hal_generic::clock::*;

pub(crate) mod default {
    #[cfg(feature = "arduino-uno")]
    pub type DefaultClock = avr_hal_generic::clock::MHz16;
    #[cfg(feature = "arduino-leonardo")]
    pub type DefaultClock = avr_hal_generic::clock::MHz16;
    #[cfg(feature = "arduino-mega2560")]
    pub type DefaultClock = avr_hal_generic::clock::MHz16;
}
