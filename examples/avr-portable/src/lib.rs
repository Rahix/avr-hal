#![no_std]

//! If you want library routines to be portable between different AVR implementations,
//! it is best to use types from [avr_hal_generic] instead of [arduino_hal]

use avr_hal_generic::usart::{Usart, UsartOps};
// use avr_hal_generic::serial::Read;
use embedded_hal::serial::Read;
pub use void::ResultVoidErrExt as _;
pub use void::ResultVoidExt as _;

pub fn report<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK>(
    serial: &mut Usart<H, USART, RX, TX, CLOCK>,
) {
    // Read a byte from the serial connection
    let b = nb::block!(serial.read()).void_unwrap();

    // Answer
    ufmt::uwriteln!(serial, "Got {}!\r", b).void_unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
