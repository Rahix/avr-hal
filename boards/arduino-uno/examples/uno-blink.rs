#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();

    loop {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(200);
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(800);
    }
}
