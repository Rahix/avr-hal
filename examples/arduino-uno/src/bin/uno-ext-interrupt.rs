/*!
 * Blinks a 4 leds in sequence on pins D3 - D6. When an external interrupt on D2/INT0 comes in
 * the sequence is reversed.
 * 
 * Note: The use of the either crate requires the deactivation of std to use it in core. See the Cargo.toml 
 * in this directory for details.
 */
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use core::ops::Range;
use core::sync::atomic::{AtomicBool, Ordering};
use arduino_hal::port::{mode, Pin};
use either::*;

static REVERSED: AtomicBool = AtomicBool::new(false);

fn is_reversed() -> bool {
    return REVERSED.load(Ordering::SeqCst);
}

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    let current = REVERSED.load(Ordering::SeqCst);
    REVERSED.store(!current, Ordering::SeqCst);
}

fn blink_for_range(range : Range<u16>, leds : &mut[Pin<mode::Output>]) {
    range.map(|i| i * 100).for_each(|ms| {
        let iter = if is_reversed() {
            Left(leds.iter_mut().rev())
        } else {
            Right(leds.iter_mut())
        };
        iter.for_each(|led| {
            led.toggle();
            arduino_hal::delay_ms(ms as u16);
        })
    });
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // thanks to tsemczyszyn and Rahix: https://github.com/Rahix/avr-hal/issues/240
    // Configure INT0 for falling edge. 0x03 would be rising edge.
    dp.EXINT.eicra.modify(|_, w| w.isc0().bits(0x02));
    // Enable the INT0 interrupt source.
    dp.EXINT.eimsk.modify(|_, w| w.int0().set_bit());

    let mut leds: [Pin<mode::Output>; 4] = [
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
    ];

    unsafe { avr_device::interrupt::enable() };

    loop {
        blink_for_range(0..10, &mut leds);
        blink_for_range(10..0, &mut leds);
    }
}
