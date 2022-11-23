#[cfg(any(feature = "attiny84", feature = "attiny85", feature = "attiny167"))]
avr_hal_generic::impl_eeprom_traditional! {Eeprom,crate::pac::EEPROM,512}
#[cfg(any(feature = "attiny88"))]
impl_eeprom_traditional! {Eeprom,arduino_hal::hal::pac::EEPROM,64}
#[cfg(any(feature = "attiny2313"))]
impl_eeprom_traditional! {Eeprom,arduino_hal::hal::pac::EEPROM,128}
