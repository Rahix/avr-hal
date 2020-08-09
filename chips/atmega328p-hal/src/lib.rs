#![no_std]

extern crate avr_hal_generic as avr_hal;

pub use avr_device::atmega328p;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;

pub mod adc;
pub mod pwm;

pub mod prelude {
    pub use crate::avr_hal::prelude::*;
    pub use crate::port::PortExt as _;
}

/// I2C Bus
pub mod i2c {
    use crate::port::portc;
    pub use avr_hal::i2c::*;

    avr_hal::impl_twi_i2c! {
        /// I2C based on ATmega328P's TWI peripheral
        pub struct I2c {
            peripheral: crate::atmega328p::TWI,
            pins: {
                sda: portc::PC4,
                scl: portc::PC5,
            },
            registers: {
                control: twcr {
                    enable: twen,
                    ack: twea,
                    int: twint,
                    start: twsta,
                    stop: twsto,
                },
                status: twsr {
                    prescaler: twps,
                    status: tws,
                },
                bitrate: twbr,
                data: twdr,
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
    //! pins.d10.into_output(&mut pins.ddr);// SS must be set to output mode
    //! // create SPI interface
    //! let mut spi = Spi::new(
    //!     dp.SPI,// SPI peripheral
    //!     pins.d13.into_output(&mut pins.ddr),// SCLK
    //!     pins.d11.into_output(&mut pins.ddr),// MOSI output pin
    //!     pins.d12.into_pull_up_input(&mut pins.ddr),// MISO input pin
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
    pub use avr_hal::spi::*;

    avr_hal::impl_spi! {
        pub struct Spi {
            peripheral: crate::atmega328p::SPI,
            pins: {
                sclk: portb::PB5,
                mosi: portb::PB3,
                miso: portb::PB4,
            }
        }
    }
}

/// Serial interface using USART
pub mod usart {
    use crate::port::portd;
    pub use avr_hal::serial::*;

    crate::avr_hal::impl_usart! {
        /// Serial interface based on ATmega328P's USART0 peripheral
        ///
        /// Maximum baudrate seems to be 57600
        pub struct Usart0 {
            peripheral: crate::atmega328p::USART0,
            pins: {
                rx: portd::PD0,
                tx: portd::PD1,
            },
            register_suffix: 0,
        }
    }
}
