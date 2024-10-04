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

// Supress warning because it doesn't recognise us using it in macros properly.
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
