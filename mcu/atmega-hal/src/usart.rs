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
