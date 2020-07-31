#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_mega2560::prelude::*;

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();

    let mut delay = arduino_mega2560::Delay::new();
    let mut pins = arduino_mega2560::Pins::new(
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

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(1000);
    }
}
