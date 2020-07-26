#![no_std]
#![no_main]

extern crate panic_halt;
use bigavr6::prelude::*;

#[bigavr6::entry]
fn main() -> ! {
    let dp = bigavr6::Peripherals::take().unwrap();

    let mut delay = bigavr6::Delay::new();
    let mut porta = dp.PORTA.split();
    let mut led = porta.pa0.into_output(&mut porta.ddr);

    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(500);
    }
}
