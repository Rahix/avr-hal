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
//! You should see it output the line `data: 15` repeatedly (aka 0b00001111).
//! If the output you see is `data: 255`, you may need to check your jumper.

#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::spi::{Settings, Spi};
use nb::block;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut delay = arduino_uno::Delay::new();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    // set up serial interface for text output
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    pins.d10.into_output(&mut pins.ddr); // SS must be set to output mode.

    // Create SPI interface.
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
        // Because MISO is connected to MOSI, the read data should be the same
        let data = block!(spi.read()).unwrap();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).unwrap();
        delay.delay_ms(1000);
    }
}
