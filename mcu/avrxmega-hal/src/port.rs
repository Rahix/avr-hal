pub use avr_hal_generic::port::{mode, PinOps, PinMode};

#[cfg(feature = "attiny404")]
avr_hal_generic::impl_port_xmega! {
    enum Ports {
        PORTA: (crate::pac::PORTA),
        PORTB: (crate::pac::PORTB),
    }

    pub struct Pins {
        pa0: PA0 = (crate::pac::PORTA, PORTA, 0, pin0ctrl),
        pa1: PA1 = (crate::pac::PORTA, PORTA, 1, pin1ctrl),
        pa2: PA2 = (crate::pac::PORTA, PORTA, 2, pin2ctrl),
        pa3: PA3 = (crate::pac::PORTA, PORTA, 3, pin3ctrl),
        pa4: PA4 = (crate::pac::PORTA, PORTA, 4, pin4ctrl),
        pa5: PA5 = (crate::pac::PORTA, PORTA, 5, pin5ctrl),
        pa6: PA6 = (crate::pac::PORTA, PORTA, 6, pin6ctrl),
        pa7: PA7 = (crate::pac::PORTA, PORTA, 7, pin7ctrl),

        pb0: PB0 = (crate::pac::PORTB, PORTB, 0, pin0ctrl),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1, pin1ctrl),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2, pin2ctrl),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3, pin3ctrl),
    }
}