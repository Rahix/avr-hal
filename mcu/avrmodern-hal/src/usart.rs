//! USART
//!
//! # Example
//!
//! Complete example source code can be found in the repository:
//! [`atmega2560-usart.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-usart.rs)
//!
//! *Note: [ufmt](https://crates.io/crates/ufmt/) is used instead of `core::fmt` because
//! `core::fmt` code quickly grows too large for AVR platforms.*
//!
//! ```
//! let dp = atmega_hal::Peripherals::take().unwrap();
//! let pins = atmega_hal::pins!(dp);
//!
//! let mut serial = Usart::new(
//!     dp.USART0,
//!     pins.pe0,
//!     pins.pe1.into_output(),
//!     Baudrate::<crate::CoreClock>::new(57600),
//! );
//!
//! ufmt::uwriteln!(&mut serial, "Hello from ATmega!").unwrap();
//!
//! loop {
//!     // Read a byte from the serial connection
//!     let b = nb::block!(serial.read()).unwrap();
//!     // Answer
//!     ufmt::uwriteln!(&mut serial, "Got {}!", b).unwrap();
//! }
//! ```

#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::usart::*;

pub type Usart<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::Usart<crate::Avrmodern, USART, RX, TX, CLOCK>;
pub type UsartWriter<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::UsartWriter<crate::Avrmodern, USART, RX, TX, CLOCK>;
pub type UsartReader<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::UsartReader<crate::Avrmodern, USART, RX, TX, CLOCK>;

#[cfg(any(feature = "attiny402"))]
pub type Usart0<CLOCK> = Usart<
    crate::pac::USART0,
    port::Pin<port::mode::Input, port::PA7>,
    port::Pin<port::mode::Output, port::PA6>,
    CLOCK,
>;

#[cfg(feature = "attiny402")]
avr_hal_generic::impl_usart_modern! {
    hal: crate::Avrmodern,
    peripheral: crate::pac::USART0,
    rx: port::PA7,
    tx: port::PA6,
}

#[cfg(any(feature = "attiny1614"))]
pub type Usart0<CLOCK> = Usart<
    crate::pac::USART0,
    port::Pin<port::mode::Input, port::PB3>,
    port::Pin<port::mode::Output, port::PB2>,
    CLOCK,
>;

#[cfg(feature = "attiny1614")]
avr_hal_generic::impl_usart_modern! {
    hal: crate::Avrmodern,
    peripheral: crate::pac::USART0,
    rx: port::PB3,
    tx: port::PB2,
}
