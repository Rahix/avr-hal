#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]
extern crate panic_halt;
use arduino_uno::prelude::*;
#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut delay = arduino_uno::Delay::new();
    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );
    pins.d10.into_output(&mut pins.ddr);// POSI pin must be made an ouptput
    pins.d11.into_output(&mut pins.ddr);// secondary select pin must be made an output
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );
    dp.SPI.spcr.write(|w| {
        w.spie().clear_bit();
        w.spe().set_bit();// must enable SPI
        w.dord().clear_bit();
        w.mstr().set_bit();// must set to primary mode
        w.cpol().clear_bit();
        w.cpha().clear_bit();
        w.spr().val_0x00()
    });
    dp.SPI.spsr.write(|w| w.spi2x().clear_bit());

    loop {
        dp.SPI.spdr.write(|w| w.bits(0b10101010));
        while dp.SPI.spsr.read().spif().bit_is_clear() {}
        let read_data = dp.SPI.spdr.read().bits();

        ufmt::uwriteln!(&mut serial, "data: {}\r", read_data).unwrap();
        delay.delay_ms(1000);
    }
}

