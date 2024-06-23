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

use crate::port;
use crate::spi::Settings;

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart0Spi = avr_hal_generic::usart_spi::UsartSpi<
    crate::Atmega,
    crate::pac::USART0,
    port::PE2,
    port::PE1,
    port::PE0,
    port::Dynamic,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    sclk: port::PE2,
    mosi: port::PE1,
    miso: port::PE0,
    cs: port::Dynamic,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart1Spi = avr_hal_generic::usart_spi::UsartSpi<
    crate::Atmega,
    crate::pac::USART1,
    port::PD5,
    port::PD3,
    port::PD2,
    port::Dynamic,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART1,
    register_suffix: 1,
    sclk: port::PD5,
    mosi: port::PD3,
    miso: port::PD2,
    cs: port::Dynamic,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart2Spi = avr_hal_generic::usart_spi::UsartSpi<
    crate::Atmega,
    crate::pac::USART2,
    port::PH2,
    port::PH1,
    port::PH0,
    port::Dynamic,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART2,
    register_suffix: 2,
    sclk: port::PH2,
    mosi: port::PH1,
    miso: port::PH0,
    cs: port::Dynamic,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart3Spi = avr_hal_generic::usart_spi::UsartSpi<
    crate::Atmega,
    crate::pac::USART3,
    port::PJ2,
    port::PJ1,
    port::PJ0,
    port::Dynamic,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART3,
    register_suffix: 3,
    sclk: port::PJ2,
    mosi: port::PJ1,
    miso: port::PJ0,
    cs: port::Dynamic,
}
