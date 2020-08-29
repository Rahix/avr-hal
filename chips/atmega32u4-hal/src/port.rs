//! Digital IO Implementations
//!
//! For a detailed explanation, refer to the [general Digital IO documentation][1].
//!
//! [1]: ../../avr_hal_generic/port/index.html

pub use avr_hal::port::mode;

pub trait PortExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

avr_hal::impl_generic_pin! {
    pub enum Pin {
        B(crate::atmega32u4::PORTB, portb, pinb, ddrb),
        C(crate::atmega32u4::PORTC, portc, pinc, ddrc),
        D(crate::atmega32u4::PORTD, portd, pind, ddrd),
        E(crate::atmega32u4::PORTE, porte, pine, ddre),
        F(crate::atmega32u4::PORTF, portf, pinf, ddrf),
    }
}

avr_hal::impl_port! {
    pub mod portb {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::B;

        impl PortExt for crate::atmega32u4::PORTB {
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

avr_hal::impl_port! {
    pub mod portc {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::C;

        impl PortExt for crate::atmega32u4::PORTC {
            regs: (pinc, ddrc, portc),
            pc6: (PC6, 6),
            pc7: (PC7, 7),
        }
    }
}

avr_hal::impl_port! {
    pub mod portd {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::D;

        impl PortExt for crate::atmega32u4::PORTD {
            regs: (pind, ddrd, portd),
            pd0: (PD0, 0),
            pd1: (PD1, 1),
            pd2: (PD2, 2),
            pd3: (PD3, 3),
            pd4: (PD4, 4),
            pd5: (PD5, 5),
            pd6: (PD6, 6),
            pd7: (PD7, 7),
        }
    }
}

avr_hal::impl_port! {
    pub mod porte {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::E;

        impl PortExt for crate::atmega32u4::PORTE {
            regs: (pine, ddre, porte),
            pe2: (PE2, 2),
            pe6: (PE6, 6),
        }
    }
}

avr_hal::impl_port! {
    pub mod portf {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::F;

        impl PortExt for crate::atmega32u4::PORTF {
            regs: (pinf, ddrf, portf),
            pf0: (PF0, 0),
            pf1: (PF1, 1),
            pf4: (PF4, 4),
            pf5: (PF5, 5),
            pf6: (PF6, 6),
            pf7: (PF7, 7),
        }
    }
}
