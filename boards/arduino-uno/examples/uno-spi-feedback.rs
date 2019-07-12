#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::spi::{Spi,Settings};
#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut delay = arduino_uno::Delay::new();
    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    pins.d10.into_output(&mut pins.ddr);// SS must be set to output mode
    let mut spi = Spi::new(
        dp.SPI,
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
        Settings::default(),
    );

    loop {
        spi.send(0b00001111).unwrap();
        let data = spi.read().unwrap();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).unwrap();
        delay.delay_ms(1000);
    }
}

