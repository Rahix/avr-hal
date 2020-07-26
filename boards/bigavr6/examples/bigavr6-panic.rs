#![no_std]
#![no_main]

use bigavr6::prelude::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial: bigavr6::Serial<bigavr6::hal::port::mode::Floating> = unsafe {
        core::mem::uninitialized()
    };

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").unwrap();

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        );
    }

    loop {}
}

#[bigavr6::entry]
fn main() -> ! {
    let dp = bigavr6::Peripherals::take().unwrap();

    let mut porte = dp.PORTE.split();

    let mut serial = bigavr6::Serial::new(
        dp.USART0,
        porte.pe0,
        porte.pe1.into_output(&mut porte.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Hello from BIGAVR6!\r").unwrap();
    // Panic messages cannot yet be captured because they rely on core::fmt
    // which is way too big for AVR
    panic!();
}
