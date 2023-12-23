/*!
 * Blink the builtin LED - the "Hello World" of embedded programming, but with a twist:
 * the blink function is not aware of `avr-hal` and only uses `embedded-hal` traits.
 */
#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use panic_halt as _;

fn blink(led: &mut impl OutputPin, delay: &mut impl DelayNs) -> ! {
    loop {
        // TODO: once embedded-hal v1.0.0 is released switch to `StatefulOutputPin` & use `toggle` (not part of RC 3)
        led.set_low().unwrap();
        delay.delay_ms(100);
        led.set_high().unwrap();
        delay.delay_ms(100);
        led.set_low().unwrap();
        delay.delay_ms(100);
        led.set_high().unwrap();
        delay.delay_ms(800);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    let mut delay = arduino_hal::Delay::new();

    blink(&mut led, &mut delay);
}
