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

use arduino_hal::prelude::*;
use arduino_hal::spi;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // set up serial interface for text output
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Create SPI interface.
    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.sck.into_output(),
        pins.mosi.into_output(),
        pins.miso.into_pull_up_input(),
        pins.led_rx.into_output(),
        spi::Settings::default(),
    );

    loop {
        // Send a byte
        nb::block!(spi.send(0b00001111)).unwrap_infallible();
        // Because MISO is connected to MOSI, the read data should be the same
        let data = nb::block!(spi.read()).unwrap_infallible();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).unwrap_infallible();
        arduino_hal::delay_ms(1000);
    }
}
