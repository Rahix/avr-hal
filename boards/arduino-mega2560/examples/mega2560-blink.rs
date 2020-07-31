#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_mega2560::prelude::*;

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();

    let mut delay = arduino_mega2560::Delay::new();
    let mut portb = dp.PORTB.split();
    let mut led = portb.pb7.into_output(&mut portb.ddr);

    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(1000);
    }
}
