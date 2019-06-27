#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;

#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut delay = arduino_uno::Delay::new();
    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();

    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(200);
        led.toggle().void_unwrap();
        delay.delay_ms(200);
        led.toggle().void_unwrap();
        delay.delay_ms(200);
        led.toggle().void_unwrap();
        delay.delay_ms(800);
    }
}
