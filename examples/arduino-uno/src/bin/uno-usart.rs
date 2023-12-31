/*!
 * Demonstration of writing to and reading from the serial console.
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use embedded_hal_v0::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        if true {
            // Read a byte from the serial connection
            let b = nb::block!(serial.read()).void_unwrap();

            // Answer
            ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
        } else {
            avr_portable::report(&mut serial);
        }
    }
}
