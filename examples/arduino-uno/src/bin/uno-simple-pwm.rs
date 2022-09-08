/*!
 * Example of using simple_pwm to fade a LED in and out on pin d5.
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

    // Digital pin 5 is connected to a LED and a resistor in series
    let mut pwm_led = pins.d5.into_output().into_pwm(&timer0);
    pwm_led.enable();

    loop {
        for x in (0..=255).chain((0..=254).rev()) {
            pwm_led.set_duty(x);
            arduino_hal::delay_ms(10);
        }
    }
}
