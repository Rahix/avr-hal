#![no_std]
#![no_main]

extern crate panic_halt;
use sparkfun_pro_micro::prelude::*;

#[sparkfun_pro_micro::entry]
fn main() -> ! {
    let dp = sparkfun_pro_micro::Peripherals::take().unwrap();

    let mut pins = sparkfun_pro_micro::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut timer4 = sparkfun_pro_micro::pwm::Timer4Pwm::new(dp.TC4);

    let mut led = pins.d13.into_output(&mut pins.ddr).into_pwm(&mut timer4);

    led.set_duty(128);
    led.enable();

    loop {
        for i in 0..=255u16 {
            let duty: u16 = i * i / 256;
            led.set_duty(duty as u8);
            sparkfun_pro_micro::delay_ms(10);
        }
    }
}
