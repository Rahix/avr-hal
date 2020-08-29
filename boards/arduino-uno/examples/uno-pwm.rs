#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use arduino_uno::pwm;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut timer1 = pwm::Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale64);

    let mut pin = pins.d9.into_output(&mut pins.ddr).into_pwm(&mut timer1);

    pin.set_duty(128);
    pin.enable();

    loop {
        for i in 0..=255u16 {
            let duty: u16 = i * i / 256;
            pin.set_duty(duty as u8);
            arduino_uno::delay_ms(10);
        }
    }
}
