#![no_std]
#![no_main]

extern crate panic_halt;
use trinket_pro::prelude::*;

#[trinket_pro::entry]
fn main() -> ! {
    let dp = trinket_pro::Peripherals::take().unwrap();

    let mut delay = trinket_pro::Delay::new();
    let mut pins = trinket_pro::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(500u16);
    }
}
