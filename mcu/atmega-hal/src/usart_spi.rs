//! USART MSPIM implimentations
//!
//! The following list details how many USARTs and if the USARTs support MSPIM for each board choosable.
//!
//! | Board | USARTs | SPI |
//! |-------|--------|-----|
//! | `atmega48p` | 1 | Yes |
//! | `atmega164pa`| 2 | Yes |
//! | `atmega168` | 1 | Yes |
//! | `atmega328p` | 1 | Yes |
//! | `atmega328pb` | 1 | Yes |
//! | `atmega32a` | 1 | No |
//! | `atmega32u4` | 1 | Yes |
//! | `atmega2560` | 4 | Yes |
//! | `atmega128a` | 2 | No |
//! | `atmega1280` | 4 | Yes |
//! | `atmega1284p` | 2 | Yes |
//! | `atmega8` | 1 | No |
//!
//! # Example
//!
//! Complete example source code can be found in the repository:
//! [`atmega2560-usart_spi-feedback.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-usart_spi-feedback.rs)
//!
//! ```
//! let dp = atmega_hal::Peripherals::take().unwrap();
//! let pins = atmega_hal::pins!(dp);
//!
//! let mut spi = usart_spi::Usart1Spi::new_from_usart(
//!     dp.USART1,
//!     pins.pd5.into_output(),
//!     pins.pd3.into_output(),
//!     pins.pd2.into_pull_up_input(),
//!     atmega_hal::spi::Settings::default(),
//! );
//!
//! let data_out = b"Hello World!";
//! let mut data_in = [0u8; 12];
//!
//! spi.transfer(&mut data_in, data_out).unwrap();
//!
//! ufmt::uwriteln!(&mut serial, "data: {:?}", data_in).unwrap();
//! ```

// Suppress warning because it doesn't recognise us using it in macros properly.
#[allow(unused_imports)]
use crate::port;

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::add_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    sclk: port::PE2,
    mosi: port::PE1,
    miso: port::PE0,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::add_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART1,
    register_suffix: 1,
    sclk: port::PD5,
    mosi: port::PD3,
    miso: port::PD2,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::add_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART2,
    register_suffix: 2,
    sclk: port::PH2,
    mosi: port::PH1,
    miso: port::PH0,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::add_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART3,
    register_suffix: 3,
    sclk: port::PJ2,
    mosi: port::PJ1,
    miso: port::PJ0,
}

#[cfg(any(
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega328pb",
    feature = "atmega48p"
))]
avr_hal_generic::add_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    sclk: port::PD4,
    mosi: port::PD1,
    miso: port::PD0,
}

#[cfg(any(feature = "atmega1284p", feature = "atmega164pa",))]
avr_hal_generic::add_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    sclk: port::PB0,
    mosi: port::PD1,
    miso: port::PD0,
}

#[cfg(any(feature = "atmega1284p", feature = "atmega164pa",))]
avr_hal_generic::add_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART1,
    register_suffix: 1,
    sclk: port::PD4,
    mosi: port::PD3,
    miso: port::PD2,
}
