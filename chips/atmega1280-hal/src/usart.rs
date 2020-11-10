use crate::port::porte;
use crate::port::portd;
use crate::port::porth;
use crate::port::portj;
pub use avr_hal_generic::usart::*;

avr_hal_generic::impl_usart! {
    pub struct Usart0 {
        peripheral: crate::pac::USART0,
        pins: {
            rx: porte::PE0,
            tx: porte::PE1,
        },
        register_suffix: 0,
    }
}

avr_hal_generic::impl_usart! {
    pub struct Usart1 {
        peripheral: crate::pac::USART1,
        pins: {
            rx: portd::PD2,
            tx: portd::PD3,
        },
        register_suffix: 1,
    }
}

avr_hal_generic::impl_usart! {
    pub struct Usart2 {
        peripheral: crate::pac::USART2,
        pins: {
            rx: porth::PH0,
            tx: porth::PH1,
        },
        register_suffix: 2,
    }
}

avr_hal_generic::impl_usart! {
    pub struct Usart3 {
        peripheral: crate::pac::USART3,
        pins: {
            rx: portj::PJ0,
            tx: portj::PJ1,
        },
        register_suffix: 3,
    }
}

