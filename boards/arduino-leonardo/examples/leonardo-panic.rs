#![no_std]
#![no_main]

use arduino_leonardo::hal::port::mode;
use arduino_leonardo::prelude::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial: arduino_leonardo::Serial<mode::Floating> =
        unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .void_unwrap();
    }

    loop {}
}

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut serial = arduino_leonardo::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
    // Panic messages cannot yet be captured because they rely on core::fmt
    // which is way too big for AVR
    panic!();
}
