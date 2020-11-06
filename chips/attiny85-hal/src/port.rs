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
        }
    }
}
