pub use avr_hal_generic::port::{mode, PinMode, PinOps};

#[cfg(feature = "attiny2313")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2],
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6],
    }
}

#[cfg(feature = "attiny167")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
    }
}

#[cfg(feature = "attiny84")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
        B: crate::pac::PORTB = [0, 1, 2, 3],
    }
}

#[cfg(feature = "attiny85")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5],
    }
}

#[cfg(feature = "attiny88")]
avr_hal_generic::impl_port_traditional! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3],
        B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
        D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
    }
}
