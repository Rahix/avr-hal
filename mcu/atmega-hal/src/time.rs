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
    timers_8_bit: [ 0, 2 ],
    timers_16_bit: [ 1, ],
}
