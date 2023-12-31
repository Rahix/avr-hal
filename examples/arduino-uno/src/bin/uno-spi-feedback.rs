/*!
 * Example of the SPI bus, by looping back output to input.
 *
 * This example demonstrates how to set up a SPI interface and communicate
 * over it.  The physical hardware configuation consists of connecting a
 * jumper directly from pin `D11` to pin `D12`.
 *
 * If done correctly, you should see it output the line `data: 15` repeatedly (aka 0b00001111).  If
 * the output you see is `data: 255`, you may need to check your jumper.
 *
 * Connections:
 *  - `D11` connected directly to `D12` (loop MOSI to MISO)
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::spi;
use embedded_hal_v0::spi::FullDuplex;
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
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    loop {
        let data = if true {
            // Send a byte
            nb::block!(spi.send(0b00001111)).void_unwrap();
            // Because MISO is connected to MOSI, the read data should be the same
            nb::block!(spi.read()).void_unwrap()
        } else {
            avr_portable::spi_loopback(&mut spi, 0b00001111)
        };

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}
