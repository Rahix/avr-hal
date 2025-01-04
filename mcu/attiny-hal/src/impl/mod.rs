#[cfg(feature = "_peripheral-adc")]
mod adc;
#[cfg(feature = "_peripheral-adc")]
pub(crate) use adc::*;

mod eeprom;
pub(crate) use eeprom::*;

mod port;
pub(crate) use port::*;

#[cfg(feature = "_peripheral-simple-pwm")]
mod simple_pwm;
#[cfg(feature = "_peripheral-simple-pwm")]
pub(crate) use simple_pwm::*;

#[cfg(feature = "_peripheral-spi")]
mod spi;
#[cfg(feature = "_peripheral-spi")]
pub(crate) use spi::*;

mod wdt;
pub(crate) use wdt::*;
