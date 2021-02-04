//! Digital IO Implementations
//!
//! For a detailed explanation, refer to the [general Digital IO documentation][1].
//!
//! [1]: ../../avr_hal_generic/port/index.html

pub use avr_hal_generic::port::mode;

pub trait PortExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

avr_hal_generic::impl_port! {
    pub mod porta {
        #[port_ext]
        use super::PortExt;

        impl PortExt for crate::pac::PORTA {
            regs: (pina, ddra, porta),
            pa0: (PA0, 0),
            pa1: (PA1, 1),
            pa2: (PA2, 2),
            pa3: (PA3, 3),
            pa4: (PA4, 4),
            pa5: (PA5, 5),
            pa6: (PA6, 6),
            pa7: (PA7, 7),
        }
    }
}

avr_hal_generic::impl_port! {
    pub mod portb {
        #[port_ext]
        use super::PortExt;

        impl PortExt for crate::pac::PORTB {
            regs: (pinb, ddrb, portb),
            pb0: (PB0, 0),
            pb1: (PB1, 1),
            pb2: (PB2, 2),
            pb3: (PB3, 3),
            pb4: (PB4, 4),
            pb5: (PB5, 5),
            pb6: (PB6, 6),
            pb7: (PB7, 7),
        }
    }
}
