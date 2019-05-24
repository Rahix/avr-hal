#![no_std]
#![no_main]

extern crate panic_halt;
use trinket::prelude::*;

#[no_mangle]
pub extern fn main() -> ! {
    let dp = trinket::Peripherals::take().unwrap();

    let mut delay = trinket::Delay::new();
    let mut pins = trinket::Pins::new(dp.PORTB);

    let mut led = pins.d1.into_output(&mut pins.ddr);

    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(500u16);
    }
}
