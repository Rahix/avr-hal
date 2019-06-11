#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

extern crate panic_halt;
use arduino_leonardo::prelude::*;

#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut delay = arduino_leonardo::Delay::new();
    let mut pins = arduino_leonardo::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
    );
    let mut serial = arduino_leonardo::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );
    let mut i2c = arduino_leonardo::I2c::new(
        dp.TWI,
        pins.d2.into_pull_up_input(&mut pins.ddr),
        pins.d3.into_pull_up_input(&mut pins.ddr),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").unwrap();
    i2c.i2cdetect(&mut serial, arduino_leonardo::hal::i2c::Direction::Write).unwrap();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").unwrap();
    i2c.i2cdetect(&mut serial, arduino_leonardo::hal::i2c::Direction::Read).unwrap();

    loop {
        delay.delay_ms(1000);
    }
}
