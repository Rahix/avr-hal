/*!
 * Example of using simple_pwm to generate some sound with piezo element
 */
#![no_std]
#![no_main]

use atmega_hal::delay::Delay;
use atmega_hal::simple_pwm::{IntoPwmPin as _, Prescaler, Timer1Pwm};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

// Define core clock in the root crate
type CoreClock = atmega_hal::clock::MHz8;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::<crate::CoreClock>::new();

    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    // Pin PB5 is connected to onboard piezo element
    let mut pwm_piezo = pins.pb5.into_output().into_pwm(&timer1);
    pwm_piezo.enable();

    loop {
        for (duty, duration, pause) in [
            (100u8, 250, 400),
            (150, 300, 300),
            (90, 375, 450),
            (120, 170, 500),
            (140, 200, 500),
            (180, 250, 600),
            (200, 300, 400),
            (160, 200, 375),
        ] {
            pwm_piezo.set_duty(duty);
            delay.delay_ms(duration);
            pwm_piezo.set_duty(0);
            delay.delay_ms(pause);
        }
        delay.delay_ms(1500);
    }
}
