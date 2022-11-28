pub use avr_hal_generic::eeprom::{EepromOps, OutOfBoundsError};

pub type Eeprom = avr_hal_generic::eeprom::Eeprom<crate::Attiny, crate::pac::EEPROM>;

#[cfg(feature = "attiny2313")]
avr_hal_generic::impl_eeprom_attiny! {
    hal: crate::Attiny,
    peripheral: crate::pac::EEPROM,
    capacity: 128,
    addr_width: u8,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(any(feature = "attiny167", feature = "attiny85"))]
avr_hal_generic::impl_eeprom_attiny! {
    hal: crate::Attiny,
    peripheral: crate::pac::EEPROM,
    capacity: 512,
    addr_width: u16,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(feature = "attiny88")]
avr_hal_generic::impl_eeprom_attiny! {
    hal: crate::Attiny,
    peripheral: crate::pac::EEPROM,
    capacity: 64,
    addr_width: u8,
    set_address: |peripheral, address| {
        peripheral.eearl.write(|w| w.bits(address));
    },
}
