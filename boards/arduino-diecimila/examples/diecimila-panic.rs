#![no_std]
#![no_main]

use arduino_diecimila::prelude::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial: arduino_diecimila::Serial<arduino_diecimila::hal::port::mode::Floating> =
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

#[arduino_diecimila::entry]
fn main() -> ! {
    let dp = arduino_diecimila::Peripherals::take().unwrap();

    let mut pins = arduino_diecimila::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_diecimila::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
    // Panic messages cannot yet be captured because they rely on core::fmt
    // which is way too big for AVR
    panic!();
}
