#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_diecimila::prelude::*;

#[arduino_diecimila::entry]
fn main() -> ! {
    let dp = arduino_diecimila::Peripherals::take().unwrap();

    let mut delay = arduino_diecimila::Delay::new();
    let mut pins = arduino_diecimila::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Timer 0 is connected to pin d6.
    let mut timer0 = arduino_diecimila::pwm::Timer0Pwm::new(dp.TC0, arduino_diecimila::pwm::Prescaler::Prescale64);

    // Use pin d6 in PWM mode.
    let mut led = pins.d6.into_output(&mut pins.ddr).into_pwm(&mut timer0);

    led.set_duty(128);
    led.enable();

    loop {
        for i in 0..=255u16 {
            let duty: u16 = i * i / 256;
            led.set_duty(duty as u8);
            delay.delay_ms(10u16);
        }
    }
}
