use crate::port::porte;
use crate::port::portd;
use crate::port::porth;
use crate::port::portj;
pub use avr_hal_generic::usart::*;

pub type Usart0<CLOCK, IMODE> = Usart<
    crate::pac::USART0,
    porte::PE0<crate::port::mode::Input<IMODE>>,
    porte::PE1<crate::port::mode::Output>,
    CLOCK,
>;
pub type Usart1<CLOCK, IMODE> = Usart<
    crate::pac::USART1,
    portd::PD2<crate::port::mode::Input<IMODE>>,
    portd::PD3<crate::port::mode::Output>,
    CLOCK,
>;
pub type Usart2<CLOCK, IMODE> = Usart<
    crate::pac::USART2,
    porth::PH0<crate::port::mode::Input<IMODE>>,
    porth::PH1<crate::port::mode::Output>,
    CLOCK,
>;
pub type Usart3<CLOCK, IMODE> = Usart<
    crate::pac::USART3,
    portj::PJ0<crate::port::mode::Input<IMODE>>,
    portj::PJ1<crate::port::mode::Output>,
    CLOCK,
>;

avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    rx: porte::PE0,
    tx: porte::PE1,
}
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::pac::USART1,
    register_suffix: 1,
    rx: portd::PD2,
    tx: portd::PD3,
}
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::pac::USART2,
    register_suffix: 2,
    rx: porth::PH0,
    tx: porth::PH1,
}
avr_hal_generic::impl_usart_traditional! {
    peripheral: crate::pac::USART3,
    register_suffix: 3,
    rx: portj::PJ0,
    tx: portj::PJ1,
}
