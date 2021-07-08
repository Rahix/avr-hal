pub use avr_hal_generic::clock::*;

pub(crate) mod default {
    #[cfg(any(
        feature = "arduino-diecimila",
        feature = "arduino-leonardo",
        feature = "arduino-mega2560",
        feature = "arduino-nano",
        feature = "arduino-uno",
        feature = "sparkfun-promicro",
    ))]
    pub type DefaultClock = avr_hal_generic::clock::MHz16;
}