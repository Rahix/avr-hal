//! # Notes
//! The ATmega48/V/88/V/168/V is an older family that is replaced with the ATmega48A/PA/88A/PA/168A/PA/328/P family.
//!
//! Examples for [atmega328p-hal](../atmega328p_hal/index.html) can in most cases be used.
//!
#![no_std]

pub use avr_device::atmega168 as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;

pub mod adc;
pub mod port;
pub mod pwm;

pub mod prelude {
    pub use avr_hal_generic::prelude::*;
    pub use crate::port::PortExt as _;
}

/// I2C Bus
pub mod i2c {
    use crate::port::portc;
    pub use avr_hal_generic::i2c::*;

    avr_hal_generic::impl_twi_i2c! {
        /// I2C based on ATmega168's TWI peripheral
        pub struct I2c {
            peripheral: crate::pac::TWI,
            pins: {
                sda: portc::PC4,
                scl: portc::PC5,
            },
        }
    }
}

pub mod spi {
    //! Implementation of the Rust Embedded-HAL SPI FullDuplex trait for AVR.
    //!
    //! The interface can be instantiated with the `new` method, and used directly
    //! or passed into a driver.  Example usage:
    //!
    //! ```
    //! // create SPI interface
    //! let (mut spi, mut cs) = Spi::new(
    //!     dp.SPI,// SPI peripheral
    //!     pins.d13.into_output(&mut pins.ddr),// SCLK
    //!     pins.d11.into_output(&mut pins.ddr),// MOSI output pin
    //!     pins.d12.into_pull_up_input(&mut pins.ddr),// MISO input pin
    //!     pins.d10.into_output(&mut pins.ddr),// CS pin
    //!     Settings::default(),
    //! );
    //!
    //! // Send a byte
    //! let sent = 0b10101010;
    //! spi.send(sent).unwrap();
    //! let response = spi.read().unwrap();
    //! ```
    //! In the example above, all of the settings are left at the default.  You can
    //! also instantiate a Settings object with the other options available.

    use crate::port::portb;
    pub use avr_hal_generic::spi::*;

    avr_hal_generic::impl_spi! {
        pub struct Spi {
            peripheral: crate::pac::SPI,
            pins: {
                sclk: portb::PB5,
                mosi: portb::PB3,
                miso: portb::PB4,
                cs: portb::PB2,
            }
        }
    }
}

/// Serial interface using USART
pub mod usart {
    use crate::port::portd;
    pub use avr_hal_generic::usart::*;

    avr_hal_generic::impl_usart! {
        /// Serial interface based on ATmega168's USART0 peripheral
        ///
        /// Maximum baudrate seems to be 57600
        pub struct Usart0 {
            peripheral: crate::pac::USART0,
            pins: {
                rx: portd::PD0,
                tx: portd::PD1,
            },
            register_suffix: 0,
        }
    }
}
