/*!
 * Example of using simple_pwm to fade in and out the builtin LED.
 *
 * The Arduino Micro's bootloader does a similar animation, with a period of
 * about one second per pulse. This example animation has a period of about five
 * seconds per pulse in order to make it easily distinguishable from the
 * bootloader's animation.
 */
#![no_std]
#![no_main]

use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::simple_pwm::Timer4Pwm;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer4 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);

    let mut pwm_led = pins.d13.into_output().into_pwm(&timer4);
    pwm_led.enable();

    loop {
        for x in (0..=255).chain((0..=254).rev()) {
            pwm_led.set_duty(x);
            arduino_hal::delay_ms(10);
        }
    }
}
