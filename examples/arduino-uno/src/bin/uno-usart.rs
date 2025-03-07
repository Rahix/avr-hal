/*!
 * Demonstration of writing to and reading from the serial console.
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap_infallible();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).unwrap_infallible();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap_infallible();
    }
}
