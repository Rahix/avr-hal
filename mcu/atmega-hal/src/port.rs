#[cfg(feature = "atmega328p")]
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
