#![no_std]

extern crate avr_hal_generic as avr_hal;

pub use avr_device::atmega48p;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;

pub mod adc;
pub mod pwm;

pub mod spi;

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
            peripheral: crate::atmega48p::TWI,
            pins: {
                sda: portc::PC4,
                scl: portc::PC5,
            },
        }
    }
}

/// Serial interface using USART
pub mod usart {
    use crate::port::portd;
    pub use avr_hal::serial::*;

    crate::avr_hal::impl_usart! {
        /// Serial interface based on ATmega48P's USART0 peripheral
        ///
        /// Maximum baudrate seems to be 57600
        pub struct Usart0 {
            peripheral: crate::atmega48p::USART0,
            pins: {
                rx: portd::PD0,
                tx: portd::PD1,
            },
            register_suffix: 0,
        }
    }
}
