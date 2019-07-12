//! This example demonstrates how to set up a SPI interface and communicate
//! over it.  The physical hardware configuation consists of connecting a
//! jumper directly from ICSP pin 10 to ICSP pin 11.
//!
//! Once this program is written to the board, the serial output can be accessed
//! with
//!
//! ```
//! sudo screen /tty/ACM0 57600
//! ```
//!
//! You should see it output the line `data: 15` repeatedly (aka 0b00001111)

#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
extern crate panic_halt;
use arduino_leonardo::prelude::*;
use arduino_leonardo::spi::{Spi,Settings};
#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();
    let mut delay = arduino_leonardo::Delay::new();
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

    pins.d10.into_output(&mut pins.ddr);// SS must be set to output mode
    // create SPI interface
    let mut spi = Spi::new(
        dp.SPI,
        pins.icsp10.into_output(&mut pins.ddr),
        pins.icsp11.into_pull_up_input(&mut pins.ddr),
        Settings::default(),
    );

    loop {
        // Send a byte
        spi.send(0b00001111).unwrap();
        // Because PISO is connected to POSI, the read data should be the same
        let data = spi.read().unwrap();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).unwrap();
        delay.delay_ms(1000);
    }
}

