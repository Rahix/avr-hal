//! I2C
//!
//! # Example
//!
//! Complete example source code can be found in the repository:
//! [`atmega2560-i2cdetect.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-i2cdetect.rs)
//!
//! ```
//! let dp = atmega_hal::Peripherals::take().unwrap();
//! let pins = atmega_hal::pins!(dp);
//!
//! let mut i2c = I2c::new(
//!     dp.TWI,
//!     pins.pd1.into_pull_up_input(),
//!     pins.pd0.into_pull_up_input(),
//!     50_000,
//! );
//!
//! i2c.i2cdetect(&mut serial, atmega_hal::i2c::Direction::Read).unwrap();
//! ```

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
