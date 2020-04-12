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
//! As long as the jumper is in place, you should repeatedly get the output
//! "Correct value transmitted!".  Try disconnecting it while running to see
//! what changes.

#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::spi::{Settings, Spi};
use nb::block;
#[no_mangle]
pub extern "C" fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut delay = arduino_uno::Delay::new();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    // set up serial interface for text output
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    pins.d10.into_output(&mut pins.ddr); // SS must be set to output mode.

    // Create SPI interface.
    let mut spi = Spi::new(
        dp.SPI,
        pins.d13.into_output(&mut pins.ddr),
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
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
            ufmt::uwriteln!(
                &mut serial,
                "Character not fed back.  Make sure you have a jumper between pins 11 and 12.\r"
            )
            .unwrap();
        }
        delay.delay_ms(1000);
    }
}
