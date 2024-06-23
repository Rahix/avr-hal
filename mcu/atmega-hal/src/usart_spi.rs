//! The following list details how many USARTs and if the USARTs support MSPIM for each board choosable.
//!
//! | Board | USARTs | SPI |
//! |-------|--------|-----|
//! | `atmega48p` | 1 | Yes |
//! | `atmega164pa`, `atmega1284p` | 2 | Yes |
//! | `atmega328p`, `atmega328pb` | 1 | Yes |
//! | `atmega32a` | 1 | No |
//! | `atmega32u4` | 1 | Yes |
//! | `atmega2560`, `atmega1280` | 4 | Yes |
//! | `atmega128a` | 2 | No |
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
