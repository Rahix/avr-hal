#[cfg(feature = "atmega48p")]
avr_hal_generic::impl_atmega_eeprom! {
    name: Eeprom,
    peripheral: crate::pac::EEPROM,
    capacity: 256,
    set_address: |peripheral, address| {
        peripheral.eearl.write(|w| w.bits(address as u8));
    },
}

#[cfg(feature = "atmega168")]
avr_hal_generic::impl_atmega_eeprom! {
    name: Eeprom,
    peripheral: crate::pac::EEPROM,
    capacity: 512,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(any(
    feature = "atmega328pb",
    feature = "atmega328p",
    feature = "atmega32u4"
))]
avr_hal_generic::impl_atmega_eeprom! {
    name: Eeprom,
    peripheral: crate::pac::EEPROM,
    capacity: 1024,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(any(
    feature = "atmega2560",
    feature = "atmega1280",
    feature = "atmega1284p"
))]
avr_hal_generic::impl_atmega_eeprom! {
    name: Eeprom,
    peripheral: crate::pac::EEPROM,
    capacity: 4096,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}
