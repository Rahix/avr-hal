#![no_std]

extern crate avr_hal_generic as avr_hal;

pub use avr_device::atmega328p;

pub use avr_hal::clock;
pub use avr_hal::delay;

pub mod port;
pub mod spi;

pub mod prelude {
    pub use crate::avr_hal::prelude::*;
    pub use crate::port::PortExt as _atmega_PortExt;
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
            registers: {
                control_a: ucsr0a {
                    data_empty: udre0,
                    recv_complete: rxc0,
                },
                control_b: ucsr0b {
                    tx_enable: txen0,
                    rx_enable: rxen0,
                },
                control_c: ucsr0c {
                    mode: umsel0,
                    char_size: ucsz0,
                    stop_bits: usbs0,
                    parity: upm0,
                },
                baud: ubrr0,
                data: udr0,
            },
        }
    }
}
