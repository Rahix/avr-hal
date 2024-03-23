#![no_std]
#![no_main]

use atmega_hal::clock;
use atmega_hal::usart::{Baudrate, Usart};
use panic_halt as _;

type I2c = atmega_hal::i2c::I2c<clock::MHz16>;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    // set up serial interface for text output
    let mut serial = Usart::new(
        dp.USART0,
        pins.pe0,
        pins.pe1.into_output(),
        Baudrate::<clock::MHz16>::new(57600),
    );

    let mut i2c = I2c::new(
        dp.TWI,
        pins.pd1.into_pull_up_input(),
        pins.pd0.into_pull_up_input(),
        50_000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").unwrap();
    i2c.i2cdetect(&mut serial, atmega_hal::i2c::Direction::Write)
        .unwrap();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").unwrap();
    i2c.i2cdetect(&mut serial, atmega_hal::i2c::Direction::Read)
        .unwrap();

    loop {}
}
