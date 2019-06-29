#![no_std]

extern crate avr_hal_generic as avr_hal;

pub use avr_device::atmega32u4;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;
pub mod usart;

pub mod prelude {
    pub use crate::avr_hal::prelude::*;
    pub use crate::port::PortExt as _atmega_PortExt;
}

pub mod i2c {
    use crate::port::portd;
    pub use avr_hal::i2c::*;

    avr_hal::impl_twi_i2c! {
        pub struct I2c {
            peripheral: crate::atmega32u4::TWI,
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
