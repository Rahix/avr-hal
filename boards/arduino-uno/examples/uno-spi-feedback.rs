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

use arduino_uno::prelude::*;
use arduino_uno::spi;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    // Create SPI interface.
    let (mut spi, _) = spi::Spi::new(
        dp.SPI,
        pins.d13.into_output(&mut pins.ddr),        //sclk
        pins.d11.into_output(&mut pins.ddr),        //mosi
        pins.d12.into_pull_up_input(&mut pins.ddr), //miso
        pins.d10.into_output(&mut pins.ddr),        //cs
        spi::Settings::default(),
    );

    loop {
        // echo
        let data = nb::block!(spi.read()).void_unwrap();
        arduino_uno::delay_ms(1000);
        nb::block!(spi.send(data)).void_unwrap();
    }
}
