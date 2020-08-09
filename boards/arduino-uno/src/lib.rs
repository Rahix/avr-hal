#![no_std]

pub extern crate atmega328p_hal as hal;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use hal::entry;

mod pins;

pub use atmega328p_hal::atmega328p;
pub use crate::atmega328p::Peripherals;
pub use atmega328p_hal::prelude;
pub use atmega328p_hal::spi;
pub use atmega328p_hal::adc;

pub mod pwm {
    //! Support for PWM pins
    //!
    //! The 3 timers of ATmega328P can be used for PWM on certain pins.
    //! The PWM methods are from `embedded_hal::PwmPin`.
    //!
    //! # Example
    //! ```
    //! let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    //!
    //! let mut timer1 = arduino_uno::pwm::Timer1Pwm::new(dp.TC1);
    //!
    //! let mut pin = pins.d9.into_output(&mut pins.ddr).into_pwm(&mut timer1);
    //!
    //! pin.set_duty(128);
    //! pin.enable();
    //! ```
    //!
    //! Here is an overview of pins and which timer they work with:
    //!
    //! | Pin | Conversion Method |
    //! | --- | --- |
    //! | `pins.d3` | `.into_pwm(&mut timer2)` |
    //! | `pins.d5` | `.into_pwm(&mut timer0)` |
    //! | `pins.d6` | `.into_pwm(&mut timer0)` |
    //! | `pins.d9` | `.into_pwm(&mut timer1)` |
    //! | `pins.d10` | `.into_pwm(&mut timer1)` |
    //! | `pins.d11` | `.into_pwm(&mut timer2)` |

    pub use atmega328p_hal::pwm::*;
}

pub use crate::pins::*;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = hal::usart::Usart0<hal::clock::MHz16, IMODE>;
pub type I2c<M> = hal::i2c::I2c<hal::clock::MHz16, M>;
