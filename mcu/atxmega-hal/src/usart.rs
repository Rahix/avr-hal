#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::usart::*;

pub type Usart<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::Usart<crate::Atxmega, USART, RX, TX, CLOCK>;
pub type UsartWriter<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::UsartWriter<crate::Atxmega, USART, RX, TX, CLOCK>;
pub type UsartReader<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::UsartReader<crate::Atxmega, USART, RX, TX, CLOCK>;

#[cfg(feature = "atmega4809")]
pub type Usart1<CLOCK> = Usart<
    crate::pac::USART1,
    port::Pin<port::mode::Input, port::PC5>,
    port::Pin<port::mode::Output, port::PC4>,
    CLOCK,
>;
#[cfg(feature = "atmega4809")]
avr_hal_generic::impl_usart_new! {
    hal: crate::Atxmega,
    peripheral: crate::pac::USART1,
    rx: port::PC5,
    tx: port::PC4,
}

#[cfg(feature = "atmega4809")]
pub type Usart3<CLOCK> = Usart<
    crate::pac::USART3,
    port::Pin<port::mode::Input, port::PB5>,
    port::Pin<port::mode::Output, port::PB4>,
    CLOCK,
>;
#[cfg(feature = "atmega4809")]
avr_hal_generic::impl_usart_new! {
    hal: crate::Atxmega,
    peripheral: crate::pac::USART3,
    rx: port::PB5,
    tx: port::PB4,
}
