pub use avr_hal_generic::usart::*;
#[allow(unused_imports)]
use crate::port;

#[cfg(feature = "atmega328p")]
pub type Usart0<CLOCK, IMODE> = Usart<
    crate::RawPeripheral<crate::pac::USART0>,
    port::Pin<port::mode::Input<IMODE>, port::PD0>,
    port::Pin<port::mode::Output, port::PD1>,
    CLOCK,
>;
#[cfg(feature = "atmega328p")]
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::RawPeripheral<crate::pac::USART0>,
    register_suffix: 0,
    rx: port::PD0,
    tx: port::PD1,
}

#[cfg(feature = "atmega32u4")]
pub type Usart1<CLOCK, IMODE> = Usart<
    crate::RawPeripheral<crate::pac::USART1>,
    port::Pin<port::mode::Input<IMODE>, port::PD2>,
    port::Pin<port::mode::Output, port::PD3>,
    CLOCK,
>;
#[cfg(feature = "atmega32u4")]
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::RawPeripheral<crate::pac::USART1>,
    register_suffix: 1,
    rx: port::PD2,
    tx: port::PD3,
}

#[cfg(feature = "atmega2560")]
pub type Usart0<CLOCK, IMODE> = Usart<
    crate::RawPeripheral<crate::pac::USART0>,
    port::Pin<port::mode::Input<IMODE>, port::PE0>,
    port::Pin<port::mode::Output, port::PE1>,
    CLOCK,
>;
#[cfg(feature = "atmega2560")]
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::RawPeripheral<crate::pac::USART0>,
    register_suffix: 0,
    rx: port::PE0,
    tx: port::PE1,
}

#[cfg(feature = "atmega2560")]
pub type Usart1<CLOCK, IMODE> = Usart<
    crate::RawPeripheral<crate::pac::USART1>,
    port::Pin<port::mode::Input<IMODE>, port::PD2>,
    port::Pin<port::mode::Output, port::PD3>,
    CLOCK,
>;
#[cfg(feature = "atmega2560")]
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::RawPeripheral<crate::pac::USART1>,
    register_suffix: 1,
    rx: port::PD2,
    tx: port::PD3,
}

#[cfg(feature = "atmega2560")]
pub type Usart2<CLOCK, IMODE> = Usart<
    crate::RawPeripheral<crate::pac::USART2>,
    port::Pin<port::mode::Input<IMODE>, port::PH0>,
    port::Pin<port::mode::Output, port::PH1>,
    CLOCK,
>;
#[cfg(feature = "atmega2560")]
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::RawPeripheral<crate::pac::USART2>,
    register_suffix: 2,
    rx: port::PH0,
    tx: port::PH1,
}

#[cfg(feature = "atmega2560")]
pub type Usart3<CLOCK, IMODE> = Usart<
    crate::RawPeripheral<crate::pac::USART3>,
    port::Pin<port::mode::Input<IMODE>, port::PH0>,
    port::Pin<port::mode::Output, port::PH1>,
    CLOCK,
>;
#[cfg(feature = "atmega2560")]
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::RawPeripheral<crate::pac::USART3>,
    register_suffix: 2,
    rx: port::PJ0,
    tx: port::PJ1,
}

