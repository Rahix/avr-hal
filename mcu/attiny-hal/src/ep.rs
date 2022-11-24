#[cfg(feature = "attiny2313")]
avr_hal_generic::impl_attiny_eeprom! {
    name: Eeprom,
    peripheral: crate::pac::EEPROM,
    capacity: 128,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address as u8));
    },
}

#[cfg(any(feature = "attiny167", feature = "attiny85"))]
avr_hal_generic::impl_attiny_eeprom! {
    name: Eeprom,
    peripheral: crate::pac::EEPROM,
    capacity: 512,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(feature = "attiny88")]
avr_hal_generic::impl_attiny_eeprom! {
    name: Eeprom,
    peripheral: crate::pac::EEPROM,
    capacity: 64,
    set_address: |peripheral, address| {
        peripheral.eearl.write(|w| w.bits(address as u8));
    },
}
