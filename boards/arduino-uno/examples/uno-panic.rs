#![no_std]
#![no_main]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial: arduino_uno::Serial<arduino_uno::hal::port::mode::Floating> = unsafe {
        core::mem::MaybeUninit::uninit().assume_init()
    };

    let _ = ufmt::uwriteln!(&mut serial, "Firmware panic!\r");

    if let Some(loc) = info.location() {
        let _ = ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        );
    }

    loop {}
}

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();
    // Panic messages cannot yet be captured because they rely on core::fmt
    // which is way too big for AVR
    panic!();
}
