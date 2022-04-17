//! Timing Circuits
//!
//! This module contains low-level hardware implementations to be used for
//! time-keeping.
//!
//! Also see [avr_hal_generic::time]

pub use avr_hal_generic::time::Prescaler;
pub use avr_hal_generic::time::Resolution;
pub use avr_hal_generic::time::TimingCircuitOps;

use crate::pac;
use crate::Atmega as HAL;

/// Define the interrupt handler for the interrupt vector for the given
/// Timer peripheral
#[cfg(feature = "atmega328p")]
#[macro_export]
macro_rules! attach_timing_circuit_interrupt {
    (Timer0; $body:block) => {
        // The timer interrupt service routine
        #[$crate::avr_device::interrupt(atmega328p)]
        fn TIMER0_COMPA() $body
    };
}

#[cfg(any(feature = "atmega328p"))]
pub use pac::TC0 as Timer0;

// TODO: implement via macro
#[cfg(any(feature = "atmega328p"))]
impl TimingCircuitOps<HAL> for Timer0 {
    type Counter = <pac::tc0::tcnt0::TCNT0_SPEC as avr_device::generic::RegisterSpec>::Ux;

    fn read_counter(&self) -> (Self::Counter, bool) {
        (self.tcnt0.read().bits(), self.tifr0.read().ocf0a().bit())
    }

    fn initialize(&self, p: Prescaler, top: Self::Counter) {
        // Set top value
        self.ocr0a.write(|w| unsafe {
            // TODO: Why is this unsafe???
            // TODO: Safety, is this sound?
            w.bits(top)
        });
        // Set prescaler
        self.tccr0b.write(|w| match p {
            Prescaler::P1 => w.cs0().direct(),
            Prescaler::P8 => w.cs0().prescale_8(),
            Prescaler::P64 => w.cs0().prescale_64(),
            Prescaler::P256 => w.cs0().prescale_256(),
            Prescaler::P1024 => w.cs0().prescale_1024(),
        });
        // Set CTC mode, enable timer
        // TODO: might be better to be put into `enable_timer`
        self.tccr0a.write(|w| w.wgm0().ctc());
    }

    unsafe fn enable_interrupt(&self) {
        self.timsk0.write(|w| w.ocie0a().set_bit());
    }

    /// Disable this clock and disable any interrupts from it
    fn disable(&self) {
        // Stop clock
        self.tccr0b.write(|w| w.cs0().no_clock());
        // Disable all interrupts
        self.timsk0.write(|w| {
            w.ocie0a()
                .clear_bit() //
                .ocie0b()
                .clear_bit() //
                .toie0()
                .clear_bit() //
        });
    }
}
