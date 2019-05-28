#![no_std]
#![no_main]

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

    // The following would also work, but needs +600% more bytes
    // writeln!(serial, "Hello from Arduino!\r").unwrap();
    serial.write_str("Hello from Arduino!\r\n").unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).unwrap();

        // Answer
        serial.write_str("You pressed ").unwrap();
        nb::block!(serial.write(b)).unwrap();
        serial.write_str("!\r\n").unwrap();
    }
}
