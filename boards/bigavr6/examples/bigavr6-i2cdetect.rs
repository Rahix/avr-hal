#![no_std]
#![no_main]

extern crate panic_halt;
use bigavr6::prelude::*;

#[bigavr6::entry]
fn main() -> ! {
    let dp = bigavr6::Peripherals::take().unwrap();

    let mut delay = bigavr6::Delay::new();

    let mut porte = dp.PORTE.split();
    let mut portd = dp.PORTD.split();

    let mut serial = bigavr6::Serial::new(
        dp.USART0,
        porte.pe0,
        porte.pe1.into_output(&mut porte.ddr),
        57600.into_baudrate(),
    );
    let mut i2c = bigavr6::I2cMaster::new(
        dp.TWI,
        portd.pd1.into_pull_up_input(&mut portd.ddr),
        portd.pd0.into_pull_up_input(&mut portd.ddr),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, bigavr6::hal::i2c::Direction::Write).void_unwrap();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, bigavr6::hal::i2c::Direction::Read).void_unwrap();

    loop {
        delay.delay_ms(1000 as u16);
    }
}
