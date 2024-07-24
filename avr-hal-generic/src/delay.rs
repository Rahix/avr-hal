//! Delay implementations

use core::marker;
use embedded_hal::delay::DelayNs;
use embedded_hal_v0::blocking::delay as delay_v0;

#[cfg(all(target_arch = "avr", avr_hal_asm_macro))]
use core::arch::asm;

/// A busy-loop delay implementation
///
/// # Example
/// ```rust
/// // Instead of arduino_hal below you may also use a different
/// // HAL based on avr_hal_generic like attiny_hal or atmega_hal
/// // depending on actual hardware. For example:
/// //
/// // use attiny_hal as hal;
///
/// use arduino_hal as hal;
/// use embedded_hal_v0::prelude::*;
///
/// let mut delay = embedded_hal_v0::delay::Delay::<hal::clock::MHz16>::new();
///
/// // Wait 1 second
/// delay.delay_ms(1000);
/// ```
///
/// # Warning
/// The delay is not accurate for values above 4095µs because of a loop whose
/// overhead is not accounted for.  This will be fixed in a future version.
#[derive(Debug, Clone, Copy)]
pub struct Delay<SPEED> {
    _speed: marker::PhantomData<SPEED>,
}

impl<SPEED> Delay<SPEED> {
    pub fn new() -> Delay<SPEED> {
        Delay {
            _speed: marker::PhantomData,
        }
    }
}

// based on https://github.com/arduino/ArduinoCore-avr/blob/master/cores/arduino/wiring.c

cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "avr", avr_hal_asm_macro))] {
        #[allow(unused_assignments)]
        fn busy_loop(mut c: u16) {
            unsafe {
                asm!(
                    "1:",
                    "sbiw {c}, 1",
                    "brne 1b",
                    c = inout(reg_iw) c,
                );
            }
        }
    } else if #[cfg(target_arch = "avr")] {
        #[allow(unused_assignments)]
        fn busy_loop(mut c: u16) {
            unsafe {
                llvm_asm!("1: sbiw $0,1\n\tbrne 1b"
                     : "=w"(c)
                     : "0"(c)
                     :
                     : "volatile"
                 );
            }
        }
    } else {
        fn busy_loop(_c: u16) {
            unimplemented!("Implementation is only available for avr targets!")
        }
    }
}

// Clock-Specific Delay Implementations ----------------------------------- {{{
impl delay_v0::DelayUs<u16> for Delay<crate::clock::MHz24> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 24 crate::clock::MHz clock for the aventurous ones, trying to overclock

        // zero delay fix
        if us == 0 {
            return;
        } // = 3 cycles, (4 when true)

        // the following loop takes a 1/6 of a microsecond (4 cycles)
        // per iteration, so execute it six times for each microsecond of
        // delay requested.
        us *= 6; // x6 us, = 7 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 22 (24) cycles above, remove 5, (5*4=20)
        // us is at least 6 so we can substract 5
        us -= 5; //=2 cycles

        busy_loop(us);
    }
}

impl delay_v0::DelayUs<u16> for Delay<crate::clock::MHz20> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 20 crate::clock::MHz clock on rare Arduino boards

        // for a one-microsecond delay, simply return.  the overhead
        // of the function call takes 18 (20) cycles, which is 1us
        #[cfg(all(target_arch = "avr", avr_hal_asm_macro))]
        unsafe {
            asm!("nop", "nop", "nop", "nop");
        }

        #[cfg(all(target_arch = "avr", not(avr_hal_asm_macro)))]
        unsafe {
            llvm_asm!("nop\nnop\nnop\nnop" :::: "volatile");
        }

        if us <= 1 {
            return;
        } // = 3 cycles, (4 when true)

        // the following loop takes a 1/5 of a microsecond (4 cycles)
        // per iteration, so execute it five times for each microsecond of
        // delay requested.
        us = (us << 2) + us; // x5 us, = 7 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 26 (28) cycles above, remove 7, (7*4=28)
        // us is at least 10 so we can substract 7
        us -= 7; // 2 cycles

        busy_loop(us);
    }
}

impl delay_v0::DelayUs<u16> for Delay<crate::clock::MHz16> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 16 crate::clock::MHz clock on most Arduino boards

        // for a one-microsecond delay, simply return.  the overhead
        // of the function call takes 14 (16) cycles, which is 1us
        if us <= 1 {
            return;
        } // = 3 cycles, (4 when true)

        // the following loop takes 1/4 of a microsecond (4 cycles)
        // per iteration, so execute it four times for each microsecond of
        // delay requested.
        us <<= 2; // x4 us, = 4 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 19 (21) cycles above, remove 5, (5*4=20)
        // us is at least 8 so we can substract 5
        us -= 5; // = 2 cycles,

        busy_loop(us);
    }
}

impl delay_v0::DelayUs<u16> for Delay<crate::clock::MHz12> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 12 crate::clock::MHz clock if somebody is working with USB

        // for a 1 microsecond delay, simply return.  the overhead
        // of the function call takes 14 (16) cycles, which is 1.5us
        if us <= 1 {
            return;
        } // = 3 cycles, (4 when true)

        // the following loop takes 1/3 of a microsecond (4 cycles)
        // per iteration, so execute it three times for each microsecond of
        // delay requested.
        us = (us << 1) + us; // x3 us, = 5 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 20 (22) cycles above, remove 5, (5*4=20)
        // us is at least 6 so we can substract 5
        us -= 5; //2 cycles

        busy_loop(us);
    }
}

impl delay_v0::DelayUs<u16> for Delay<crate::clock::MHz10> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 10 crate::clock::MHz clock if somebody is working with USB

        // for a 1 microsecond delay, simply return.  the overhead
        // of the function call takes 14 (16) cycles, which is 1.5us
        if us <= 1 {
            return;
        } // = 3 cycles, (4 when true)

        // 4 cycles per busy_loop iteration = 0.4 us per busy loop, so 2.5 times to get 1 us
        us = ((us << 2) + us) >> 1; // x2.5

        busy_loop(us);
    }
}

impl delay_v0::DelayUs<u16> for Delay<crate::clock::MHz8> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 8 crate::clock::MHz internal clock

        // for a 1 and 2 microsecond delay, simply return.  the overhead
        // of the function call takes 14 (16) cycles, which is 2us
        if us <= 2 {
            return;
        } // = 3 cycles, (4 when true)

        // the following loop takes 1/2 of a microsecond (4 cycles)
        // per iteration, so execute it twice for each microsecond of
        // delay requested.
        us <<= 1; //x2 us, = 2 cycles

        // account for the time taken in the preceeding commands.
        // we just burned 17 (19) cycles above, remove 4, (4*4=16)
        // us is at least 6 so we can substract 4
        us -= 4; // = 2 cycles

        busy_loop(us);
    }
}

impl delay_v0::DelayUs<u16> for Delay<crate::clock::MHz1> {
    fn delay_us(&mut self, mut us: u16) {
        // for the 1 crate::clock::MHz internal clock (default settings for common Atmega microcontrollers)

        // the overhead of the function calls is 14 (16) cycles
        if us <= 16 {
            return;
        } //= 3 cycles, (4 when true)
        if us <= 25 {
            return;
        } //= 3 cycles, (4 when true), (must be at least 25 if we want to substract 22)

        // compensate for the time taken by the preceeding and next commands (about 22 cycles)
        us -= 22; // = 2 cycles
                  // the following loop takes 4 microseconds (4 cycles)
                  // per iteration, so execute it us/4 times
                  // us is at least 4, divided by 4 gives us 1 (no zero delay bug)
        us >>= 2; // us div 4, = 4 cycles

        busy_loop(us);
    }
}

// ------------------------------------------------------------------------ }}}

impl<SPEED> delay_v0::DelayUs<u8> for Delay<SPEED>
where
    Delay<SPEED>: delay_v0::DelayUs<u16>,
{
    fn delay_us(&mut self, us: u8) {
        delay_v0::DelayUs::<u16>::delay_us(self, us as u16);
    }
}

impl<SPEED> delay_v0::DelayUs<u32> for Delay<SPEED>
where
    Delay<SPEED>: delay_v0::DelayUs<u16>,
{
    fn delay_us(&mut self, us: u32) {
        // TODO: Somehow fix the overhead induced by this loop
        // This was previously a range-based for loop, but that would
        // compile down to fairly poor code. This is slightly better,
        // but still has some overhead and may not lead to cycle-accurate
        // delays.
        let iters = us >> 12;
        let mut i = 0;
        while i < iters {
            delay_v0::DelayUs::<u16>::delay_us(self, 0xfff);
            i += 1;
        }
        delay_v0::DelayUs::<u16>::delay_us(self, (us & 0xfff) as u16);
    }
}

impl<SPEED> delay_v0::DelayMs<u16> for Delay<SPEED>
where
    Delay<SPEED>: delay_v0::DelayUs<u32>,
{
    fn delay_ms(&mut self, ms: u16) {
        delay_v0::DelayUs::<u32>::delay_us(self, ms as u32 * 1000);
    }
}

impl<SPEED> delay_v0::DelayMs<u8> for Delay<SPEED>
where
    Delay<SPEED>: delay_v0::DelayMs<u16>,
{
    fn delay_ms(&mut self, ms: u8) {
        delay_v0::DelayMs::<u16>::delay_ms(self, ms as u16);
    }
}

impl<SPEED> DelayNs for Delay<SPEED>
where
    Delay<SPEED>: delay_v0::DelayUs<u16>,
{
    fn delay_ns(&mut self, ns: u32) {
        // quick-win to get an initial implementation.
        // note that the trait does not guarantee nanosecond-accuracy.
        delay_v0::DelayUs::<u32>::delay_us(self, ns.div_ceil(1000))
    }

    fn delay_us(&mut self, us: u32) {
        delay_v0::DelayUs::<u32>::delay_us(self, us);
    }
}
