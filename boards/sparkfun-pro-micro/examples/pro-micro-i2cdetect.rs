#![no_std]
#![no_main]

extern crate panic_halt;
use sparkfun_pro_micro::prelude::*;

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
    let mut i2c = sparkfun_pro_micro::I2c::new(
        dp.TWI,
        pins.d2.into_pull_up_input(&mut pins.ddr),
        pins.d3.into_pull_up_input(&mut pins.ddr),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, sparkfun_pro_micro::hal::i2c::Direction::Write)
        .void_unwrap();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, sparkfun_pro_micro::hal::i2c::Direction::Read)
        .void_unwrap();

    loop {
        sparkfun_pro_micro::delay_ms(1000);
    }
}
