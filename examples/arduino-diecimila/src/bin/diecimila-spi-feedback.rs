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

use arduino_hal::arduino::diecimila as board;
use board::prelude::*;
use board::spi;
use embedded_hal_v0::spi::FullDuplex;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let pins = board::pins!(dp);

    // set up serial interface for text output
    let mut serial = board::default_serial!(dp, pins, 57600);

    // Create SPI interface.
    let (mut spi, _) = board::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    loop {
        // Send a byte
        nb::block!(spi.send(0b00001111)).unwrap_infallible();
        // Because MISO is connected to MOSI, the read data should be the same
        let data = nb::block!(spi.read()).unwrap_infallible();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).unwrap_infallible();
        board::delay_ms(1000);
    }
}
