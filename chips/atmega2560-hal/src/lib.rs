#![no_std]

/// Reexport of `atmega2560` from `avr-device`
pub use avr_device::atmega2560 as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;

pub mod adc;
pub mod port;
pub mod pwm;
pub mod spi;
pub mod usart;

pub mod prelude {
    pub use avr_hal_generic::prelude::*;
    pub use crate::port::PortExt as _;
}

pub mod i2c {
    use crate::port::portd;
    pub use avr_hal_generic::i2c::*;

    avr_hal_generic::impl_twi_i2c! {
        pub struct I2c {
            peripheral: crate::pac::TWI,
            pins: {
                sda: portd::PD1,
                scl: portd::PD0,
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
