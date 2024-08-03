#![feature(panic_info_message)]
//! Replacement for avr-std-stub with a custom panic handler.

use core::panic::PanicInfo;

use arduino_hal::prelude::*;
use arduino_hal::{delay_ms, pins, Peripherals};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let dp = unsafe { Peripherals::steal() };
    let pins = pins!(dp);
    let mut status = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "I panicked because {}!\r", _info.message().as_str().unwrap_or("useless")).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "I panicked at {}!\r", _info.location().unwrap().file()).unwrap_infallible();

    delay_ms(100);

    ufmt::uwriteln!(&mut serial, "I panicked on {}!\r", _info.location().unwrap().line()).unwrap_infallible();

    if let Some(s) = _info.payload().downcast_ref::<&str>() {
        ufmt::uwriteln!(&mut serial, "More info: {}!\r", s).unwrap_infallible();
    } 

    loop {
        status.set_high();
        delay_ms(100);
        status.set_low();
        delay_ms(100);
        status.set_high();
        delay_ms(100);
        status.set_low();
        delay_ms(100);
        status.set_high();
        delay_ms(400);
        status.set_low();
        delay_ms(1000);
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() {}