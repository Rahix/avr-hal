//! This example demonstrates how to set up a SPI interface and communicate
//! over it.  The physical hardware configuation consists of connecting a
//! jumper directly from pin `~50` to pin `~51`.
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
use arduino_mega2560::prelude::*;
use arduino_mega2560::spi::{Settings, Spi};

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();
    let mut delay = arduino_mega2560::Delay::new();
    let mut pins = arduino_mega2560::Pins::new(dp.PORTD);

    let mut serial = arduino_mega2560::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    // Create SPI interface.
    let (mut spi, _) = Spi::new(
        dp.SPI,
        pins.d52.into_output(&mut pins.ddr),        //sclk
        pins.d51.into_output(&mut pins.ddr),        //mosi
        pins.d50.into_pull_up_input(&mut pins.ddr), //miso
        pins.d53.into_output(&mut pins.ddr),        //cs
        Settings::default(),
    );

    loop {
        let to_send = 0b00001111;
        ufmt::uwriteln!(&mut serial, "Sendig {}!\r", to_send).void_unwrap();
        // Send to Uno
        spi.send(to_send).unwrap();
        delay.delay_ms(1000u16);
        // receive from Uno
        let data = spi.read().unwrap();

        // Echo to PC
        ufmt::uwriteln!(&mut serial, "Got {}!\r", data).void_unwrap();

        delay.delay_ms(1000u16);
    }
}
