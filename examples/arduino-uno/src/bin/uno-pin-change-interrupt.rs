/*!
 * Example of enabling and handling pin change interrupts
 *
 * In this example we can get an interrupt when pin 2 changes
 * and use that to move a stepper motor.
 *
 */
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

use core::sync::atomic::{AtomicBool, Ordering};

static PIN_CHANGED: AtomicBool = AtomicBool::new(false);

//This function is called on change of pin 2
#[avr_device::interrupt(atmega328p)]
#[allow(non_snake_case)]
fn PCINT2() {
    PIN_CHANGED.store(true, Ordering::SeqCst);
}

fn rotate(flag: &AtomicBool) -> bool {
    avr_device::interrupt::free(|_cs| {
        if flag.load(Ordering::SeqCst) {
            flag.store(false, Ordering::SeqCst);
            true
        } else {
            false
        }
    })
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    //Pins used to drive the stepper motor
    let mut dir_pin = pins.d4.into_output();
    let mut step_pin = pins.d5.into_output();

    //Rotary encoder attached on these pins
    let rotary_pins = [
        pins.d2.into_floating_input().downgrade(), //CLK
        pins.d3.into_floating_input().downgrade(), //DT
    ];

    // Enable the PCINT2 pin change interrupt
    dp.EXINT.pcicr.write(|w| unsafe { w.bits(0b100) });

    // Enable pin change interrupts on PCINT18 which is pin PD2 (= d2)
    dp.EXINT.pcmsk2.write(|w| w.bits(0b100));

    //From this point on an interrupt can happen
    unsafe { avr_device::interrupt::enable() };

    loop {
        if rotate(&PIN_CHANGED) {
            //Check which direction the rotary encoder was turned
            if rotary_pins[0].is_high() != rotary_pins[1].is_high() {
                dir_pin.set_high();
            } else {
                dir_pin.set_low();
            }

            //Move the stepper motor
            for _ in 0..=50 {
                step_pin.set_high();
                arduino_hal::delay_us(2000);
                step_pin.set_low();
                arduino_hal::delay_us(2000);
            }
        }
    }
}
