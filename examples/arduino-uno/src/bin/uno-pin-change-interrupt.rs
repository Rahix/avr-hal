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

use arduino_hal;
use panic_halt as _;

use core:: sync::atomic::{AtomicBool, Ordering};

static PIN_CHANGED: AtomicBool = AtomicBool::new(false);

//This function is called on change of pin 2
#[avr_device::interrupt(atmega328p)]
#[allow(non_snake_case)]
fn PCINT2() {
	avr_device::interrupt::free(|_cs| {
        PIN_CHANGED.store(true, Ordering::SeqCst);
    });
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
        pins.d3.into_floating_input().downgrade() //DT
    ];

    //Needed to enable pin change interrupts
    dp.EXINT.pcicr.write(|w| unsafe { w.bits(0b100) });
    dp.EXINT.pcmsk2.write(|w| unsafe { w.bits(0b100) });

    //From this point on an interrupt can happen
    unsafe { avr_device::interrupt::enable() };

    loop {
        avr_device::interrupt::free(|_cs| {
            if PIN_CHANGED.load(Ordering::SeqCst) {
                PIN_CHANGED.store(false, Ordering::SeqCst);

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
        });
    }
}
