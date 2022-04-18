//! Timing Circuits
//!
//! This module contains low-level hardware implementations to be used for
//! time-keeping.
//!
//! Also see [avr_hal_generic::time]

use avr_hal_generic::time::Prescaler;
use avr_hal_generic::time::TimingCircuitOps;

use crate::HAL;

#[cfg(feature = "atmega328p")]
avr_hal_generic::impl_timer_circuit_via_TCn_OCRnA!{
    hal: HAL,
    chip: atmega328p,
    // TODO: `TC1` does not work, because `WGM1_W` has no `ctc` function.
    timers: [ 0, /* 1, */ 2 ],
}
