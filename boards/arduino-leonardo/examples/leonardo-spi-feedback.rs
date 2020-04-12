//! This example demonstrates how to set up a SPI interface and communicate
//! over it.  The physical hardware configuation consists of connecting a
//! jumper directly from ICSP pin 10 to ICSP pin 11.
//!
//! Once this program is written to the board, you can use the board's serial
//! connection to see the output.
//!
//! As long as the jumper is in place, you should repeatedly get the output
//! "Correct value transmitted!".  Try disconnecting it while running to see
//! what changes.

#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
extern crate panic_halt;
use arduino_leonardo::prelude::*;
use arduino_leonardo::spi::{Settings, Spi};
use nb::block;
#[no_mangle]
pub extern "C" fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();
    let mut delay = arduino_leonardo::Delay::new();
    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    let mut serial = arduino_leonardo::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    pins.led_rx.into_output(&mut pins.ddr); // SS must be set to output mode.

    // Create SPI interface.
    let mut spi = Spi::new(
        dp.SPI,
        pins.sck.into_output(&mut pins.ddr),
        pins.mosi.into_output(&mut pins.ddr),
        pins.miso.into_pull_up_input(&mut pins.ddr),
        Settings::default(),
    );

    loop {
        // Send a byte
        block!(spi.send(64)).unwrap();
        let data: u8 = block!(spi.read()).unwrap();

        // Input in pull-up mode, so always reading high if no connection
        if data != 0b11111111 {
            ufmt::uwrite!(&mut serial, "Character fed back: ").unwrap();
            block!(serial.write(data)).unwrap();
            ufmt::uwrite!(&mut serial, "\r\n").unwrap();
        } else {
            ufmt::uwriteln!(&mut serial, "Character not fed back.  Make sure you have a jumper between the MISO and MOSI pins.\r").unwrap();
        }
        delay.delay_ms(1000);
    }
}
