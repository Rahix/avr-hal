pub use avr_hal_generic::clock::*;

pub(crate) mod default {
    #[cfg(feature = "sparkfun-promicro")]
    pub type DefaultClock = avr_hal_generic::clock::MHz16;
}
