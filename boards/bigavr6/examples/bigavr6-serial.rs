#![no_std]
#![no_main]

extern crate panic_halt;
use bigavr6::prelude::*;

#[no_mangle]
pub extern fn main() -> ! {
    let dp = bigavr6::Peripherals::take().unwrap();

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

    let mut serial = bigavr6::Serial::new(
        dp.USART1,
        pins.d2,
        pins.d3.into_output(&mut pins.ddr),
        57600,
    );

    // The following would also work, but needs +600% more bytes
    // writeln!(serial, "Hello from Arduino!\r").unwrap();
    serial.write_str("Hello from BIGAVR6!\r\n").unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).unwrap();

        // Answer
        serial.write_str("You pressed ").unwrap();
        nb::block!(serial.write(b)).unwrap();
        serial.write_str("!\r\n").unwrap();
    }
}
