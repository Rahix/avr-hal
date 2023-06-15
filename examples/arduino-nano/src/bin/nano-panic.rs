/*!
 * Example of a custom panic handler.
 *
 * The panic handler will print out where the panic occurred and then blink the oboard LED rapidly
 * to make the user aware of the problem.
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;

// Documentation build does not like this and fails with the following error, even though
// everything is fine when compiling the binary.
//
//      error[E0152]: found duplicate lang item `panic_impl`
//
// Ignore the panic handler in documentation builds.  This is not needed in downstream code, it is
// an artifact of the avr-hal examples structure.
#[cfg(not(doc))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Print out panic location
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

    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
    ufmt::uwriteln!(&mut serial, "Panic in 5 seconds!\r").void_unwrap();

    arduino_hal::delay_ms(5000);

    // Panic messages cannot yet be captured because they rely on core::fmt
    // which is way too big for AVR
    panic!();
}
