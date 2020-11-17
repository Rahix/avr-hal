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

extern crate panic_halt;
use sparkfun_pro_micro::prelude::*;
use sparkfun_pro_micro::spi;
use nb::block;

#[sparkfun_pro_micro::entry]
fn main() -> ! {
    let dp = sparkfun_pro_micro::Peripherals::take().unwrap();

    let mut pins = sparkfun_pro_micro::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut serial = sparkfun_pro_micro::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    // Create SPI interface.
    let (mut spi, _) = spi::Spi::new(
        dp.SPI,
        pins.sck.into_output(&mut pins.ddr),
        pins.mosi.into_output(&mut pins.ddr),
        pins.miso.into_pull_up_input(&mut pins.ddr),
        pins.led_rx.into_output(&mut pins.ddr),
        spi::Settings::default(),
    );

    loop {
        // Send a byte
        block!(spi.send(0b00001111)).void_unwrap();
        // Because MISO is connected to MOSI, the read data should be the same
        let data = block!(spi.read()).void_unwrap();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).void_unwrap();
        sparkfun_pro_micro::delay_ms(1000);
    }
}
