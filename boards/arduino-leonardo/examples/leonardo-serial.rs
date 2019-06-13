#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

extern crate panic_halt;
use arduino_leonardo::prelude::*;

#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut pins = arduino_leonardo::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
    );

    let mut serial = arduino_leonardo::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).unwrap();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap();
    }
}
