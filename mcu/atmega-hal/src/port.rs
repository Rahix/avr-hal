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
