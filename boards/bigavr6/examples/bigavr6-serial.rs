#![no_std]
#![no_main]

extern crate panic_halt;
use bigavr6::prelude::*;

#[no_mangle]
pub extern fn main() -> ! {
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
