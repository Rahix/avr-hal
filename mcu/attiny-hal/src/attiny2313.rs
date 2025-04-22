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
