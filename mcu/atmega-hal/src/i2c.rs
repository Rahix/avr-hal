#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::i2c::*;

#[cfg(any(
    feature = "atmega128a",
    feature = "atmega1280",
    feature = "atmega2560",
    feature = "atmega32u4"
))]
pub type I2c<CLOCK> = avr_hal_generic::i2c::I2c<
    crate::Atmega,
    crate::pac::TWI,
    port::Pin<port::mode::Input, port::PD1>,
    port::Pin<port::mode::Input, port::PD0>,
    CLOCK,
>;
#[cfg(any(
    feature = "atmega128a",
    feature = "atmega1280",
    feature = "atmega2560",
    feature = "atmega32u4"
))]
avr_hal_generic::impl_i2c_twi! {
    hal: crate::Atmega,
    peripheral: crate::pac::TWI,
    sda: port::PD1,
    scl: port::PD0,
}

#[cfg(any(feature = "atmega164pa"))]
pub type I2c<CLOCK> = avr_hal_generic::i2c::I2c<
    crate::Atmega,
    crate::pac::TWI,
    port::Pin<port::mode::Input, port::PC1>,
    port::Pin<port::mode::Input, port::PC0>,
    CLOCK,
>;
#[cfg(any(feature = "atmega164pa"))]
avr_hal_generic::impl_i2c_twi! {
    hal: crate::Atmega,
    peripheral: crate::pac::TWI,
    sda: port::PC1,
    scl: port::PC0,
}

#[cfg(any(
    feature = "atmega328p",
    feature = "atmega168",
    feature = "atmega48p",
    feature = "atmega8"
))]
pub type I2c<CLOCK> = avr_hal_generic::i2c::I2c<
    crate::Atmega,
    crate::pac::TWI,
    port::Pin<port::mode::Input, port::PC4>,
    port::Pin<port::mode::Input, port::PC5>,
    CLOCK,
>;
#[cfg(any(
    feature = "atmega328p",
    feature = "atmega168",
    feature = "atmega48p",
    feature = "atmega8"
))]
avr_hal_generic::impl_i2c_twi! {
    hal: crate::Atmega,
    peripheral: crate::pac::TWI,
    sda: port::PC4,
    scl: port::PC5,
}

#[cfg(any(feature = "atmega328pb"))]
pub type I2c0<CLOCK> = avr_hal_generic::i2c::I2c<
    crate::Atmega,
    crate::pac::TWI0,
    port::Pin<port::mode::Input, port::PC4>,
    port::Pin<port::mode::Input, port::PC5>,
    CLOCK,
>;
#[cfg(any(feature = "atmega328pb"))]
avr_hal_generic::impl_i2c_twi! {
    hal: crate::Atmega,
    peripheral: crate::pac::TWI0,
    sda: port::PC4,
    scl: port::PC5,
}
#[cfg(any(feature = "atmega328pb"))]
pub type I2c1<CLOCK> = avr_hal_generic::i2c::I2c<
    crate::Atmega,
    crate::pac::TWI1,
    port::Pin<port::mode::Input, port::PE0>,
    port::Pin<port::mode::Input, port::PE1>,
    CLOCK,
>;
#[cfg(any(feature = "atmega328pb"))]
avr_hal_generic::impl_i2c_twi! {
    hal: crate::Atmega,
    peripheral: crate::pac::TWI1,
    sda: port::PE0,
    scl: port::PE1,
}

#[cfg(any(feature = "atmega1284p", feature = "atmega32a"))]
pub type I2c<CLOCK> = avr_hal_generic::i2c::I2c<
    crate::Atmega,
    crate::pac::TWI,
    port::Pin<port::mode::Input, port::PC1>,
    port::Pin<port::mode::Input, port::PC0>,
    CLOCK,
>;
#[cfg(any(feature = "atmega1284p", feature = "atmega32a"))]
avr_hal_generic::impl_i2c_twi! {
    hal: crate::Atmega,
    peripheral: crate::pac::TWI,
    sda: port::PC1,
    scl: port::PC0,
}
