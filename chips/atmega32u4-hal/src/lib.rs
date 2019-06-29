#![no_std]

extern crate avr_hal_generic as avr_hal;

pub use avr_device::atmega32u4;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;

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

pub mod usart {
    use crate::port::portd;
    pub use avr_hal::serial::*;

    crate::avr_hal::impl_usart! {
        pub struct Usart1 {
            peripheral: crate::atmega32u4::USART1,
            pins: {
                rx: portd::PD2,
                tx: portd::PD3,
            },
            registers: {
                control_a: ucsr1a {
                    data_empty: udre1,
                    recv_complete: rxc1,
                },
                control_b: ucsr1b {
                    tx_enable: txen1,
                    rx_enable: rxen1,
                },
                control_c: ucsr1c {
                    mode: umsel1,
                    char_size: ucsz1,
                    stop_bits: usbs1,
                    parity: upm1,
                },
                baud: ubrr1,
                data: udr1,
            },
        }
    }
}
