#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_leonardo::prelude::*;

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut delay = arduino_leonardo::Delay::new();
    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    let mut timer4 = arduino_leonardo::pwm::Timer4Pwm::new(dp.TC4);

    let mut led = pins.d13.into_output(&mut pins.ddr).into_pwm(&mut timer4);

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
