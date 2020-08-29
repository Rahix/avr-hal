#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_diecimila::prelude::*;

#[arduino_diecimila::entry]
fn main() -> ! {
    let dp = arduino_diecimila::Peripherals::take().unwrap();

    let mut delay = arduino_diecimila::Delay::new();
    let mut pins = arduino_diecimila::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();

    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(200u8);
        led.toggle().void_unwrap();
        delay.delay_ms(200u8);
        led.toggle().void_unwrap();
        delay.delay_ms(200u8);
        led.toggle().void_unwrap();
        delay.delay_ms(200u8);
    }
}
