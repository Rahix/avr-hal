pub mod eeprom {
    pub use crate::periphals::eeprom::*;

    avr_hal_generic::impl_eeprom_attiny! {
        hal: crate::Attiny,
        peripheral: crate::pac::EEPROM,
        capacity: 128,
        addr_width: u8,
        set_address: |peripheral, address| {
            peripheral.eear.write(|w| w.bits(address));
        },
    }
}

pub mod port {
    pub use crate::periphals::port::*;

    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: crate::pac::PORTA = [0, 1, 2],
            B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6],
        }
    }
}
