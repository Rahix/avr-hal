/*!
 * Blink the builtin LED - the "Hello World" of embedded programming.
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    let mut led = pins.led.into_output();
    led.set_high();

    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
        led.toggle();
        arduino_hal::delay_ms(100);
        led.toggle();
        arduino_hal::delay_ms(100);
        led.toggle();
        arduino_hal::delay_ms(800);
    }
}
