#![no_std]
#![no_main]

extern crate panic_halt;
use bigavr6::prelude::*;

#[no_mangle]
pub extern fn main() -> ! {
    let dp = bigavr6::Peripherals::take().unwrap();

    let mut delay = bigavr6::Delay::new();
    let mut pins = bigavr6::Pins::new(
        dp.PORTA,
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
        dp.PORTF,
        dp.PORTG,
        dp.PORTH,
        dp.PORTJ,
        dp.PORTK,
        dp.PORTL,
    );

    let mut led = pins.a0.into_output(&mut pins.ddr);

    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(500);
    }
}
