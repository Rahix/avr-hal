/*!
 * Example of using simple_pwm to fade the built-in LED in and out.
 */

#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    // Digital pin 1 is also connected to an onboard LED marked "L"
    let mut pwm_led = pins.d1.into_output().into_pwm(&timer0);
    pwm_led.enable();

    loop {
        for x in (0..=255).chain((0..=254).rev()) {
            pwm_led.set_duty(x);
            arduino_hal::delay_ms(10);
        }
    }
}
