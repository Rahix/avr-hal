pub use avr_hal_generic::port::{mode, PinMode, PinOps};

#[cfg(any(feature = "atmega48p", feature = "atmega168", feature = "atmega328p"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
    }
}

#[cfg(any(feature = "atmega164pa"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6 ,7],
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6 ,7],
        C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6 ,7],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6 ,7],
    }
}

#[cfg(feature = "atmega8u2")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        PORTB: (crate::pac::PORTB, portb, pinb, ddrb),
        PORTC: (crate::pac::PORTC, portc, pinc, ddrc),
        PORTD: (crate::pac::PORTD, portd, pind, ddrd),
    }

    pub struct Pins {
        pb0: PB0 = (crate::pac::PORTB, PORTB, 0, portb, pinb, ddrb),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1, portb, pinb, ddrb),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2, portb, pinb, ddrb),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3, portb, pinb, ddrb),
        pb4: PB4 = (crate::pac::PORTB, PORTB, 4, portb, pinb, ddrb),
        pb5: PB5 = (crate::pac::PORTB, PORTB, 5, portb, pinb, ddrb),
        pb6: PB6 = (crate::pac::PORTB, PORTB, 6, portb, pinb, ddrb),
        pb7: PB7 = (crate::pac::PORTB, PORTB, 7, portb, pinb, ddrb),
        pc0: PC0 = (crate::pac::PORTC, PORTC, 0, portc, pinc, ddrc),
        pc1: PC1 = (crate::pac::PORTC, PORTC, 1, portc, pinc, ddrc),
        pc2: PC2 = (crate::pac::PORTC, PORTC, 2, portc, pinc, ddrc),
        pc3: PC3 = (crate::pac::PORTC, PORTC, 3, portc, pinc, ddrc),
        pc4: PC4 = (crate::pac::PORTC, PORTC, 4, portc, pinc, ddrc),
        pc5: PC5 = (crate::pac::PORTC, PORTC, 5, portc, pinc, ddrc),
        pc6: PC6 = (crate::pac::PORTC, PORTC, 6, portc, pinc, ddrc),
        pc7: PC7 = (crate::pac::PORTC, PORTC, 7, portc, pinc, ddrc),
        pd0: PD0 = (crate::pac::PORTD, PORTD, 0, portd, pind, ddrd),
        pd1: PD1 = (crate::pac::PORTD, PORTD, 1, portd, pind, ddrd),
        pd2: PD2 = (crate::pac::PORTD, PORTD, 2, portd, pind, ddrd),
        pd3: PD3 = (crate::pac::PORTD, PORTD, 3, portd, pind, ddrd),
        pd4: PD4 = (crate::pac::PORTD, PORTD, 4, portd, pind, ddrd),
        pd5: PD5 = (crate::pac::PORTD, PORTD, 5, portd, pind, ddrd),
        pd6: PD6 = (crate::pac::PORTD, PORTD, 6, portd, pind, ddrd),
        pd7: PD7 = (crate::pac::PORTD, PORTD, 7, portd, pind, ddrd),
    }
}
#[cfg(feature = "atmega328pb")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
        E: crate::pac::PORTE = [0, 1, 2, 3],
    }
}

#[cfg(feature = "atmega32u4")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [6, 7],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
        E: crate::pac::PORTE = [2, 6],
        F: crate::pac::PORTF = [0, 1, 4, 5, 6, 7],
    }
}

#[cfg(any(feature = "atmega128a"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
        E: crate::pac::PORTE = [0, 1, 2, 3, 4, 5, 6, 7],
        F: crate::pac::PORTF = [0, 1, 2, 3, 4, 5, 6, 7],
        G: crate::pac::PORTG = [0, 1, 2, 3, 4],
    }
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
        E: crate::pac::PORTE = [0, 1, 2, 3, 4, 5, 6, 7],
        F: crate::pac::PORTF = [0, 1, 2, 3, 4, 5, 6, 7],
        G: crate::pac::PORTG = [0, 1, 2, 3, 4, 5],
        H: crate::pac::PORTH = [0, 1, 2, 3, 4, 5, 6, 7],
        J: crate::pac::PORTJ = [0, 1, 2, 3, 4, 5, 6, 7],
        K: crate::pac::PORTK = [0, 1, 2, 3, 4, 5, 6, 7],
        L: crate::pac::PORTL = [0, 1, 2, 3, 4, 5, 6, 7],
    }
}

#[cfg(any(feature = "atmega1284p", feature = "atmega32a"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
    }
}

#[cfg(any(feature = "atmega8"))]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
    }
}
