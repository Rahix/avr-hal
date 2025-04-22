pub mod adc {
    pub use crate::periphals::adc::*;

    // Fixme: Implement ADC for ATtiny84.
}

pub mod eeprom {
    pub use crate::periphals::eeprom::*;

    // Fixme: Implement EEPROM for ATtiny84.
}

pub mod port {
    pub use crate::periphals::port::*;

    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
            B: crate::pac::PORTB = [0, 1, 2, 3],
        }
    }
}
