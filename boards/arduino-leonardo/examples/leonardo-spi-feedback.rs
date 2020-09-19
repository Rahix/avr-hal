//! This example demonstrates how to set up a SPI interface and communicate
//! over it.  The physical hardware configuation consists of connecting a
//! jumper directly from ICSP pin 10 to ICSP pin 11.
//!
//! Once this program is written to the board, you can use the board's serial
//! connection to see the output.  You should see it output the line
//! `data: 15` repeatedly (aka 0b00001111).  If the output you see is
//! `data: 255`, you may need to check your jumper.

#![no_std]
#![no_main]

use arduino_leonardo::prelude::*;
use arduino_leonardo::spi;
use nb::block;
use panic_halt as _;

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut serial = arduino_leonardo::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    pins.led_rx.into_output(&mut pins.ddr); // SS must be set to output mode.

    // Create SPI interface.
    let mut spi = spi::Spi::new(
        dp.SPI,
        pins.sck.into_output(&mut pins.ddr),
        pins.mosi.into_output(&mut pins.ddr),
        pins.miso.into_pull_up_input(&mut pins.ddr),
        spi::Settings::default(),
    );

    loop {
        // Send a byte
        block!(spi.send(0b00001111)).void_unwrap();
        // Because MISO is connected to MOSI, the read data should be the same
        let data = block!(spi.read()).void_unwrap();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).void_unwrap();
        arduino_leonardo::delay_ms(1000);
    }
}
