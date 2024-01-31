/*!
 * Example of the SPI bus, by looping back output to input, but with the
 * feedback function not being aware of the SPI part of `avr-hal` and using
 * only the `embedded-hal` and `embedded-hal-bus` traits.
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

use arduino_hal::hal::port::{PD0, PD1};
use arduino_hal::pac::USART0;
use arduino_hal::port::{
    mode::{Input, Output},
    Pin,
};
use arduino_hal::prelude::*;
use arduino_hal::spi;
use arduino_hal::Usart;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Set up the serial interface for text output
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Create SPI interface.
    let (spi_bus, cs_pin) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );
    let mut spi = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi_bus, cs_pin);

    feedback(&mut spi, &mut serial);
}

fn feedback(
    spi: &mut impl embedded_hal::spi::SpiDevice,
    serial: &mut Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>,
) -> ! {
    loop {
        // Send a byte and read data, which should be the same because MISO is
        // connected to MOSI
        let write = [15u8];
        let mut read = [255u8; 1];
        spi.transfer(&mut read, &write).unwrap();

        ufmt::uwriteln!(serial, "data: {}\r", read[0]).unwrap_infallible();
        arduino_hal::delay_ms(1000);
    }
}
