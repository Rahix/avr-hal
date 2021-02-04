#![no_std]

/// Reexport of `atmega48p` from `avr-device`
pub use avr_device::atmega48p as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;

pub mod port;

pub mod adc;
pub mod pwm;

pub mod spi;

pub mod prelude {
    pub use avr_hal_generic::prelude::*;
    pub use crate::port::PortExt as _;
}

/// I2C Bus
pub mod i2c {
    use crate::port::portc;
    pub use avr_hal_generic::i2c::*;

    avr_hal_generic::impl_twi_i2c! {
        /// I2C based on ATmega328P's TWI peripheral
        pub struct I2c {
            peripheral: crate::pac::TWI,
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
    pub use avr_hal_generic::usart::*;

    pub type Usart0<CLOCK, IMODE> = Usart<
        crate::pac::USART0,
        portd::PD0<crate::port::mode::Input<IMODE>>,
        portd::PD1<crate::port::mode::Output>,
        CLOCK,
    >;

    avr_hal_generic::impl_usart_traditional! {
        peripheral: crate::pac::USART0,
        register_suffix: 0,
        rx: portd::PD0,
        tx: portd::PD1,
    }
}
