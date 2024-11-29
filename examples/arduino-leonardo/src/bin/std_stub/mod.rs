#![feature(panic_info_message)]
//! Replacement for avr-std-stub with a custom panic handler.

// use core::fmt::{self, Debug};
use core::panic::PanicInfo;

use arduino_hal::hal::port::{PD2, PD3};
use arduino_hal::hal::Atmega;
use arduino_hal::pac::USART1;
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::{prelude::*, Usart};
use arduino_hal::{delay_ms, pins, Peripherals};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // use ::core::fmt::Write as _;

    let dp = unsafe { Peripherals::steal() };
    let pins = pins!(dp);
    let mut status = pins.d13.into_output();
    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // struct UartWriter {
    //     uart: Usart<USART1, Pin<Input, PD2>, Pin<Output, PD3>>
    // }
    // impl ::core::fmt::Write for UartWriter {
    //     fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
    //         ufmt::uwriteln!(&mut self.uart, "{}", s).unwrap_infallible();
    //         Ok(())
    //     }
    // }

    // ufmt::uwriteln!(&mut serial, "I panicked at {}!\r", _info.location().unwrap().file()).unwrap_infallible();

    // delay_ms(100);

    // ufmt::uwriteln!(&mut serial, "I panicked on {}!\r", _info.location().unwrap().line()).unwrap_infallible();

    // if let Some(s) = _info.payload().downcast_ref::<&str>() {
    //     ufmt::uwriteln!(&mut serial, "More info: {}!\r", s).unwrap_infallible();
    // } 

    // let mut uart = UartWriter { uart: serial };
    // ::core::writeln!(uart, "{}", _info).ok();

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