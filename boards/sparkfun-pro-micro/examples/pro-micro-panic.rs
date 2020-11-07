#![no_std]
#![no_main]

use sparkfun_pro_micro::prelude::*;
use sparkfun_pro_micro::hal::port::mode;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial: sparkfun_pro_micro::Serial<mode::Floating> = unsafe {
        core::mem::MaybeUninit::uninit().assume_init()
    };

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        ).void_unwrap();
    }

    loop {}
}

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

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
    // Panic messages cannot yet be captured because they rely on core::fmt
    // which is way too big for AVR
    panic!();
}
