#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::usart::*;

pub type Usart<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::Usart<crate::Atmega, USART, RX, TX, CLOCK>;
pub type UsartWriter<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::UsartWriter<crate::Atmega, USART, RX, TX, CLOCK>;
pub type UsartReader<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::UsartReader<crate::Atmega, USART, RX, TX, CLOCK>;

#[cfg(any(feature = "atmega168", feature = "atmega328p", feature = "atmega328pb", feature = "atmega1284p"))]
pub type Usart0<CLOCK> = Usart<
    crate::pac::USART0,
    port::Pin<port::mode::Input, port::PD0>,
    port::Pin<port::mode::Output, port::PD1>,
    CLOCK,
>;
#[cfg(any(feature = "atmega168", feature = "atmega328p", feature = "atmega328pb", feature = "atmega1284p"))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    rx: port::PD0,
    tx: port::PD1,
}

#[cfg(feature = "atmega328pb")]
pub type Usart1<CLOCK> = Usart<
    crate::pac::USART1,
    port::Pin<port::mode::Input, port::PB4>,
    port::Pin<port::mode::Output, port::PB3>,
    CLOCK,
>;
#[cfg(feature = "atmega328pb")]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART1,
    register_suffix: 1,
    rx: port::PB4,
    tx: port::PB3,
}

#[cfg(any(feature = "atmega32u4", feature = "atmega1280", feature = "atmega2560", feature = "atmega1284p"))]
pub type Usart1<CLOCK> = Usart<
    crate::pac::USART1,
    port::Pin<port::mode::Input, port::PD2>,
    port::Pin<port::mode::Output, port::PD3>,
    CLOCK,
>;
#[cfg(any(feature = "atmega32u4", feature = "atmega1280", feature = "atmega2560", feature = "atmega1284p"))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART1,
    register_suffix: 1,
    rx: port::PD2,
    tx: port::PD3,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart0<CLOCK> = Usart<
    crate::pac::USART0,
    port::Pin<port::mode::Input, port::PE0>,
    port::Pin<port::mode::Output, port::PE1>,
    CLOCK,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    rx: port::PE0,
    tx: port::PE1,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart2<CLOCK> = Usart<
    crate::pac::USART2,
    port::Pin<port::mode::Input, port::PH0>,
    port::Pin<port::mode::Output, port::PH1>,
    CLOCK,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART2,
    register_suffix: 2,
    rx: port::PH0,
    tx: port::PH1,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart3<CLOCK> = Usart<
    crate::pac::USART3,
    port::Pin<port::mode::Input, port::PJ0>,
    port::Pin<port::mode::Output, port::PJ1>,
    CLOCK,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART3,
    register_suffix: 3,
    rx: port::PJ0,
    tx: port::PJ1,
}
