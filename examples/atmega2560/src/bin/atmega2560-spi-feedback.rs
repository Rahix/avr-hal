//! This example demonstrates how to set up a SPI interface and communicate
//! over it.  The physical hardware configuration consists of connecting a
//! jumper directly from pin `PB2` to pin `PB3`.
//!
//! Run the program using `cargo run`.
//! You should see it output the line `data: 42` repeatedly.
//! If the output you see is `data: 255`, you may need to check your jumper.

#![no_std]
#![no_main]

use atmega_hal::delay::Delay;
use atmega_hal::spi;
use atmega_hal::usart::{Baudrate, Usart};
use embedded_hal::delay::DelayNs;
use embedded_hal::spi::SpiBus;
use panic_halt as _;

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz16;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::<crate::CoreClock>::new();

    // set up serial interface for text output
    let mut serial = Usart::new(
        dp.USART0,
        pins.pe0,
        pins.pe1.into_output(),
        Baudrate::<crate::CoreClock>::new(57600),
    );

    // Create SPI interface.
    let (mut spi, _) = spi::Spi::new(
        dp.SPI,
        pins.pb1.into_output(),
        pins.pb2.into_output(),
        pins.pb3.into_pull_up_input(),
        pins.pb0.into_output(),
        spi::Settings::default(),
    );

    loop {
        // Send a byte
        let data_out: [u8; 1] = [42];
        let mut data_in: [u8; 1] = [0];
        // Send a byte
        // Because MISO is connected to MOSI, the read data should be the same
        spi.transfer(&mut data_in, &data_out).unwrap();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data_in[0]).unwrap();
        delay.delay_ms(1000);
    }
}
