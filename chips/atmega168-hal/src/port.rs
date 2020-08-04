//! `PORTB` - `PORTD` digital IO
pub use avr_hal::port::mode;

pub trait PortExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

avr_hal::impl_generic_pin! {
    pub enum Pin {
        B(crate::atmega168::PORTB, portb, pinb, ddrb),
        C(crate::atmega168::PORTC, portc, pinc, ddrc),
        D(crate::atmega168::PORTD, portd, pind, ddrd),
    }
}

avr_hal::impl_port! {
    pub mod portb {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::B;

        impl PortExt for crate::atmega168::PORTB {
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

        impl PortExt for crate::atmega168::PORTC {
            regs: (pinc, ddrc, portc),
            pc0: (PC0, 0),
            pc1: (PC1, 1),
            pc2: (PC2, 2),
            pc3: (PC3, 3),
            pc4: (PC4, 4),
            pc5: (PC5, 5),
            pc6: (PC6, 6),
        }
    }
}

avr_hal::impl_port! {
    pub mod portd {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::D;

        impl PortExt for crate::atmega168::PORTD {
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

