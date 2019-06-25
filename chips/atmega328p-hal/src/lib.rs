#![no_std]

extern crate avr_hal_generic as avr_hal;
pub extern crate avr_device as atmega328p;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;
pub mod usart;

pub mod prelude {
    pub use crate::avr_hal::prelude::*;
    pub use crate::port::PortExt as _atmega_PortExt;
}

pub mod i2c {
    use crate::port::portc;
    pub use avr_hal::i2c::*;

    avr_hal::impl_twi_i2c! {
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
