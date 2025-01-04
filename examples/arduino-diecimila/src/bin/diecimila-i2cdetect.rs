#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::arduino::diecimila as board;
use board::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let pins = board::pins!(dp);
    let mut serial = board::default_serial!(dp, pins, 57600);

    let mut i2c = board::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").unwrap_infallible();
    i2c.i2cdetect(&mut serial, board::i2c::Direction::Write)
        .unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").unwrap_infallible();
    i2c.i2cdetect(&mut serial, board::i2c::Direction::Read)
        .unwrap_infallible();

    loop {}
}
