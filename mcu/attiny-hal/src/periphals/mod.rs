// ATtiny2313 does not have ADC and will not compile with this module
#[cfg(not(feature = "attiny2313"))]
pub mod adc;
pub mod eeprom;
