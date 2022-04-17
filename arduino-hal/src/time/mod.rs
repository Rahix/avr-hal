//! Time-keeping facilities
//!
//! TODO: how do you use that?
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

/// Supported timer devices
pub mod timers {
    pub use crate::hal::time::*;
}
