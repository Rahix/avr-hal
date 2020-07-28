#![no_std]
#![no_main]

extern crate panic_halt;
use bigavr6::prelude::*;

#[bigavr6::entry]
fn main() -> ! {
    let dp = bigavr6::Peripherals::take().unwrap();

    let mut porte = dp.PORTE.split();
    let mut serial = bigavr6::Serial::new(
        dp.USART0,
        porte.pe0,
        porte.pe1.into_output(&mut porte.ddr),
        57600,
    );

    // The following would also work, but needs +600% more bytes
    // writeln!(serial, "Hello from Arduino!\r").unwrap();
    serial.write_str("Hello from BIGAVR6!\r\n").void_unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        serial.write_str("You pressed ").void_unwrap();
        nb::block!(serial.write(b)).void_unwrap();
        serial.write_str("!\r\n").void_unwrap();
    }
}
