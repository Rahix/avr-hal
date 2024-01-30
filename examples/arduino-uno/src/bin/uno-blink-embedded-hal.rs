/*!
 * Blink the builtin LED - the "Hello World" of embedded programming, but with a twist:
 * the blink function is not aware of `avr-hal` and only uses `embedded-hal` traits.
 */
#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::StatefulOutputPin;

use panic_halt as _;

fn blink(led: &mut impl StatefulOutputPin, delay: &mut impl DelayNs) -> ! {
    loop {
        led.toggle().unwrap();
        delay.delay_ms(100);
        led.toggle().unwrap();
        delay.delay_ms(100);
        led.toggle().unwrap();
        delay.delay_ms(100);
        led.toggle().unwrap();
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
