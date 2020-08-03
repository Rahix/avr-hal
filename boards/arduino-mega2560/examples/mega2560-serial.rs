#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_mega2560::prelude::*;

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();

    let mut porte = dp.PORTE.split();
    let mut serial = arduino_mega2560::Serial::new(
        dp.USART0,
        porte.pe0,
        porte.pe1.into_output(&mut porte.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        serial.write_str("You pressed ").void_unwrap();
        nb::block!(serial.write(b)).void_unwrap();
        serial.write_str("!\r\n").void_unwrap();
    }
}
