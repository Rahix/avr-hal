
#[cfg(feature = "atmega48p")]
avr_hal_generic::impl_eeprom_traditional! {Eeprom,crate::pac::EEPROM,256}
#[cfg(feature = "atmega168")]
avr_hal_generic::impl_eeprom_traditional! {Eeprom,crate::pac::EEPROM,512}
#[cfg(any(feature = "atmega328pb", feature = "atmega328p"))]
avr_hal_generic::impl_eeprom_traditional! {Eeprom,crate::pac::EEPROM,1024}
#[cfg(feature = "atmega32u4")]
avr_hal_generic::impl_eeprom_traditional! {Eeprom,crate::pac::EEPROM,1024}
#[cfg(feature = "atmega2560")]
avr_hal_generic::impl_eeprom_traditional! {Eeprom,crate::pac::EEPROM,4096}
#[cfg(feature = "atmega1280")]
avr_hal_generic::impl_eeprom_traditional! {Eeprom,crate::pac::EEPROM,4096}
#[cfg(feature = "atmega1284p")]
avr_hal_generic::impl_eeprom_traditional! {Eeprom,crate::pac::EEPROM,4096}
