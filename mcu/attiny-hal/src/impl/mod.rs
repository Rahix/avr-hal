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

macro_rules! avr_hal {
    (
        device: $device:ident,
        eeprom: { $($eeprom:tt)* },
        port: {$ ($port:tt)* },
        $(pwm: {$ ($pwm:tt)* },)?
        $(spi: {$ ($spi:tt)* },)?
        $(adc: {$ ($adc:tt)* },)?
        wdt: {$ ($wdt:tt)* },
    ) => {
        pub use avr_device::$device as pac;
        pub struct Hal;
        use crate::r#impl::*;

        impl_mod_eeprom! {
            hal: crate::$device,
            $($eeprom)*
        }
        impl_mod_port! {
            hal: crate::$device,
            $($port)*
        }
        $(impl_mod_simple_pwm! {
            hal: crate::$device,
            $($pwm)*
        })?
        $(impl_mod_spi! {
            hal: crate::$device,
            $($spi)*
        })?
        $(impl_mod_adc! {
            hal: crate::$device,
            $($adc)*
        })?
        impl_mod_wdt! {
            hal: crate::$device,
            $($wdt)*
        }
    };
}

pub(crate) use avr_hal;
