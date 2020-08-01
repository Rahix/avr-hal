pub trait PortExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

avr_hal::impl_generic_pin! {
    pub enum Pin {
        A(crate::attiny88::PORTA, porta, pina),
        B(crate::attiny88::PORTB, portb, pinb),
        C(crate::attiny88::PORTC, portc, pinc),
        D(crate::attiny88::PORTD, portd, pind),
    }
}

avr_hal::impl_port! {
    pub mod porta {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::A;

        impl PortExt for crate::attiny88::PORTA {
            regs: (pina, ddra, porta),
            pa0: (PA0, 0),
            pa1: (PA1, 1),
            pa2: (PA2, 2),
            pa3: (PA3, 3),
        }
    }
}

avr_hal::impl_port! {
    pub mod portb {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::B;

        impl PortExt for crate::attiny88::PORTB {
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

        impl PortExt for crate::attiny88::PORTC {
            regs: (pinc, ddrc, portc),
            pc0: (PC0, 0),
            pc1: (PC1, 1),
            pc2: (PC2, 2),
            pc3: (PC3, 3),
            pc4: (PC4, 4),
            pc5: (PC5, 5),
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

        impl PortExt for crate::attiny88::PORTD {
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
