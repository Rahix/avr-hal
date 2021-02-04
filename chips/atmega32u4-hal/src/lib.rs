#![no_std]

/// Reexport of `atmega32u4` from `avr-device`
pub use avr_device::atmega32u4 as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;

pub mod adc;
pub mod port;
pub mod pwm;
pub mod wdt;

pub mod prelude {
    pub use avr_hal_generic::prelude::*;
    pub use crate::port::PortExt as _;
}

/// I2C Bus
pub mod i2c {
    use crate::port::portd;
    pub use avr_hal_generic::i2c::*;

    avr_hal_generic::impl_twi_i2c! {
        /// I2C based on ATmega32U4's TWI peripheral
        pub struct I2c {
            peripheral: crate::pac::TWI,
            pins: {
                sda: portd::PD1,
                scl: portd::PD0,
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

    pub use avr_hal_generic::spi::*;
    use crate::port::portb;

    avr_hal_generic::impl_spi! {
        pub struct Spi {
            peripheral: crate::pac::SPI,
            pins: {
                sclk: portb::PB1,
                mosi: portb::PB2,
                miso: portb::PB3,
                cs: portb::PB0,
            }
        }
    }
}

/// Serial interface using USART
pub mod usart {
    use crate::port::portd;
    pub use avr_hal_generic::usart::*;

    /// Serial interface based on ATmega32U4's USART1 peripheral
    pub type Usart1<CLOCK, IMODE> = Usart<
        crate::pac::USART1,
        portd::PD2<crate::port::mode::Input<IMODE>>,
        portd::PD3<crate::port::mode::Output>,
        CLOCK,
    >;

    avr_hal_generic::impl_usart_traditional! {
        peripheral: crate::pac::USART1,
        register_suffix: 1,
        rx: portd::PD2,
        tx: portd::PD3,
    }
}
