#![no_std]
#![no_main]

use atmega_hal::prelude::*;
use atmega_hal::usart::{Baudrate, Usart};
use panic_halt as _;

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz16;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    let mut serial = Usart::new(
        dp.USART0,
        pins.pe0,
        pins.pe1.into_output(),
        Baudrate::<crate::CoreClock>::new(57600),
    );

    ufmt::uwriteln!(&mut serial, "Hello from ATmega!\r").unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).unwrap();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap();
    }
}
