pub use avr_hal_generic::clock::*;

pub(crate) mod default {
    cfg_if::cfg_if! {
        if #[cfg(feature = "arduino-uno")] {
            pub type DefaultClock = avr_hal_generic::clock::MHz16;
        }
    }
}
