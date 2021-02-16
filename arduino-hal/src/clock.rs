pub use avr_hal_generic::clock::*;

pub(crate) mod default {
    #[cfg(any(
        feature = "arduino-uno",
        feature = "arduino-leonardo",
        feature = "arduino-mega"
    ))]
    pub type DefaultClock = avr_hal_generic::clock::MHz16;
}
