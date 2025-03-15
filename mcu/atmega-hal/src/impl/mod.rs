#![allow(unused_imports)]

mod adc;
pub(crate) use adc::*;

mod eeprom;
pub(crate) use eeprom::*;

mod i2c;
pub(crate) use i2c::*;

mod port;
pub(crate) use port::*;

mod simple_pwm;
pub(crate) use simple_pwm::*;

mod spi;
pub(crate) use spi::*;

mod usart;
pub(crate) use usart::*;

mod wdt;
pub(crate) use wdt::*;

macro_rules! avr_hal {
    (
        device: $device:ident,
        eeprom: { $($eeprom:tt)* },
        port: { $($port:tt)* },
        $(pwm: { $($pwm:tt)* },)?
        $(i2c: { $($i2c:tt)* },)?
        $(spi: { $($spi:tt)* },)?
        $(usart: { $($usart:tt)* },)?
        $(adc: { $($adc:tt)* },)?
        wdt: { $($wdt:tt)* },
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
        $(impl_mod_i2c! {
            hal: crate::$device,
            $($i2c)*
        })?
        $(impl_mod_spi! {
            hal: crate::$device,
            $($spi)*
        })?
        $(impl_mod_usart! {
            hal: crate::$device,
            $($usart)*
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
