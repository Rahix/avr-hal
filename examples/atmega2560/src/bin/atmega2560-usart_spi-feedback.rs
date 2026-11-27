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
use atmega_hal::usart::{Baudrate, Usart};
use atmega_hal::usart_spi;
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
    let mut spi = usart_spi::Usart1Spi::new_from_usart(
        dp.USART1,
        pins.pd5.into_output(),
        pins.pd3.into_output(),
        pins.pd2.into_pull_up_input(),
        atmega_hal::spi::Settings::default(),
    );

    // Other SPI examples for other USART's

    // let mut spi = usart_spi::Usart2Spi::new_from_usart(
    //     dp.USART2,
    //     pins.ph2.into_output(),
    //     pins.ph1.into_output(),
    //     pins.ph0.into_pull_up_input(),
    //     atmega_hal::spi::Settings::default(),
    // );

    // let mut spi = usart_spi::Usart3Spi::new_from_usart(
    //     dp.USART3,
    //     pins.pj2.into_output(),
    //     pins.pj1.into_output(),
    //     pins.pj0.into_pull_up_input(),
    //     atmega_hal::spi::Settings::default(),
    // );

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
