pub(crate) mod delay;
pub(crate) use delay::*;

#[cfg(feature = "_mcu-atmega")]
pub(crate) mod adc;
#[cfg(feature = "_mcu-atmega")]
pub(crate) use adc::*;

#[cfg(feature = "_mcu-atmega")]
pub(crate) mod i2c;
#[cfg(feature = "_mcu-atmega")]
pub(crate) use i2c::*;

#[cfg(feature = "_mcu-atmega")]
pub(crate) mod spi;
#[cfg(feature = "_mcu-atmega")]
pub(crate) use spi::*;

#[cfg(feature = "_mcu-atmega")]
pub(crate) mod usart;
#[cfg(feature = "_mcu-atmega")]
pub(crate) use usart::*;

pub(crate) mod eeprom;
pub(crate) use eeprom::*;

pub(crate) mod simple_pwm;
pub(crate) use simple_pwm::*;
