/*!
 * Example of using simple_pwm to fade the built-in LED in and out.
 */

#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::adafruit::trinket as board;
use board::simple_pwm::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let pins = board::pins!(dp);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    // Digital pin 1 is also connected to an onboard LED marked "L"
    let mut pwm_led = pins.d1.into_output().into_pwm(&timer0);
    pwm_led.enable();

    loop {
        for x in (0..=255).chain((0..=254).rev()) {
            pwm_led.set_duty(x);
            board::delay_ms(10);
        }
    }
}
