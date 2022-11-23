/*!
 * Demonstration of writing to and reading from the eeprom.
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

// use embedded_hal::serial::Read;

use embedded_storage::nor_flash::ReadNorFlash;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut ep = arduino_hal::Eeprom {};
    let ep_capacity = ep.capacity();
    ufmt::uwriteln!(&mut serial, "eeprom capacity is:{}\r", ep_capacity).void_unwrap();

    // atmega328p eeprom size is 1024
    let mut data = [0_u8; 1024];
    let _start_address: u16 = 0;

    if ep.read(0, &mut data).is_err() {
        ufmt::uwriteln!(&mut serial, "read eeprom fail:\r").void_unwrap();
        loop {}
    }

    ufmt::uwriteln!(&mut serial, "Got:\r").void_unwrap();
    for i in data {
        ufmt::uwriteln!(&mut serial, "{}", i).void_unwrap();
    }
    loop {}
}
