pub use avr_hal::port::mode;

pub trait PortExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

avr_hal::impl_generic_pin! {
    pub enum Pin {
        A(crate::atmega1280::PORTA, porta, pina),
        B(crate::atmega1280::PORTB, portb, pinb),
        C(crate::atmega1280::PORTC, portc, pinc),
        D(crate::atmega1280::PORTD, portd, pind),
        E(crate::atmega1280::PORTE, porte, pine),
        F(crate::atmega1280::PORTF, portf, pinf),
        G(crate::atmega1280::PORTG, portg, ping),
        H(crate::atmega1280::PORTH, porth, pinh),
        J(crate::atmega1280::PORTJ, portj, pinj),
        K(crate::atmega1280::PORTK, portk, pink),
        L(crate::atmega1280::PORTL, portl, pinl),
    }
}

avr_hal::impl_port! {
    pub mod porta {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::A;

        impl PortExt for atmega1280::PORTA {
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

avr_hal::impl_port! {
    pub mod portb {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::B;

        impl PortExt for atmega1280::PORTB {
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

        impl PortExt for atmega1280::PORTC {
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

        impl PortExt for atmega1280::PORTD {
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

        impl PortExt for atmega1280::PORTE {
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

        impl PortExt for atmega1280::PORTF {
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

avr_hal::impl_port! {
    pub mod portg {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::G;

        impl PortExt for atmega1280::PORTG {
            regs: (ping, ddrg, portg),
            pg0: (PG0, 0),
            pg1: (PG1, 1),
            pg4: (PG4, 4),
            pg5: (PG5, 5),
            pg6: (PG6, 6),
            pg7: (PG7, 7),
        }
    }
}

avr_hal::impl_port! {
    pub mod porth {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::H;

        impl PortExt for atmega1280::PORTH {
            regs: (pinh, ddrh, porth),
            ph0: (PH0, 0),
            ph1: (PH1, 1),
            ph4: (PH4, 4),
            ph5: (PH5, 5),
            ph6: (PH6, 6),
            ph7: (PH7, 7),
        }
    }
}

avr_hal::impl_port! {
    pub mod portj {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::J;

        impl PortExt for atmega1280::PORTJ {
            regs: (pinj, ddrj, portj),
            pj0: (PJ0, 0),
            pj1: (PJ1, 1),
            pj4: (PJ4, 4),
            pj5: (PJ5, 5),
            pj6: (PJ6, 6),
            pj7: (PJ7, 7),
        }
    }
}

avr_hal::impl_port! {
    pub mod portk {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::K;

        impl PortExt for atmega1280::PORTK {
            regs: (pink, ddrk, portk),
            pk0: (PK0, 0),
            pk1: (PK1, 1),
            pk4: (PK4, 4),
            pk5: (PK5, 5),
            pk6: (PK6, 6),
            pk7: (PK7, 7),
        }
    }
}

avr_hal::impl_port! {
    pub mod portl {
        #[port_ext]
        use super::PortExt;

        #[generic_pin]
        use Pin::L;

        impl PortExt for atmega1280::PORTL {
            regs: (pinl, ddrl, portl),
            pl0: (Pl0, 0),
            pl1: (Pl1, 1),
            pl4: (Pl4, 4),
            pl5: (Pl5, 5),
            pl6: (Pl6, 6),
            pl7: (Pl7, 7),
        }
    }
}