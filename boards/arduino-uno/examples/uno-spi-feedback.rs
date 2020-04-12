//! This example demonstrates how to set up a SPI interface and communicate
//! over it.  The physical hardware configuation consists of connecting a
//! jumper directly from pin `~11` to pin `~12`.
//!
//! Once this program is written to the board, the serial output can be
//! accessed with
//!
//! ```
//! sudo screen /dev/ttyACM0 57600
//! ```
//!
//! As long as the jumper is in place, you should repeatedly get the output
//! "Correct value transmitted!".  Try disconnecting it while running to see
//! what changes.

#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::spi::{Spi,Settings};
use nb::block;
#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut delay = arduino_uno::Delay::new();
    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );
    // set up serial interface for text output
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    pins.d10.into_output(&mut pins.ddr);// SS must be set to output mode
    // create SPI interface
    let mut spi = Spi::new(
        dp.SPI,
        pins.d13.into_output(&mut pins.ddr),
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
        Settings::default(),
    );

    loop {
        // Send a byte
        block!(spi.send(0b00001111)).unwrap();
        let data = block!(spi.read()).unwrap();

        // Because MISO is connected to MOSI, the read data should be the same
        if data == 0b00001111 {
            ufmt::uwriteln!(&mut serial, "Correct value transmitted!\r").unwrap();
        } else {
            ufmt::uwriteln!(&mut serial, "Data not fed back.  Make sure you have a jumper between pins 11 and 12.\r").unwrap();
        }
        delay.delay_ms(1000);
    }
}

