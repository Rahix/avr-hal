#![no_std]

pub extern crate atmega32u4_hal as hal;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use hal::entry;

mod pins;

pub use atmega32u4_hal::atmega32u4;
pub use crate::atmega32u4::Peripherals;
pub use atmega32u4_hal::prelude;
pub use atmega32u4_hal::spi;
pub use atmega32u4_hal::adc;

pub mod pwm {
    //! Support for PWM pins
    //!
    //! The 4 timers of ATmega32U4 can be used for PWM on certain pins.
    //! The PWM methods are from `embedded_hal::PwmPin`.
    //!
    //! # Example
    //! ```
    //! let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);
    //! let mut timer1 = Timer1Pwm::new(dp.TC1);
    //!
    //! let mut d3 = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer0);
    //!
    //! d3.set_duty(128);
    //! d3.enable();
    //! ```
    //!
    //! Here is an overview of pins and which timer they work with:
    //!
    //! | Pin | Conversion Method | Alternate Conversion Method |
    //! | --- | --- | --- |
    //! | `pins.d3` | `.into_pwm(&mut timer0)` | |
    //! | `pins.d5` | `.into_pwm(&mut timer3)` | |
    //! | `pins.d6` | `.into_pwm(&mut timer4)` | |
    //! | `pins.d9` | `.into_pwm(&mut timer1)` | |
    //! | `pins.d10` | `.into_pwm(&mut timer1)` | `.into_pwm4(&mut timer4)` |
    //! | `pins.d11` | `.into_pwm(&mut timer0)` | `.into_pwm1(&mut timer1)` |
    //! | `pins.d13` | `.into_pwm(&mut timer4)` | |

    pub use atmega32u4_hal::pwm::*;
}

pub use crate::pins::*;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;
pub type Serial<IMODE> = hal::usart::Usart1<hal::clock::MHz16, IMODE>;
pub type I2c<M> = hal::i2c::I2c<hal::clock::MHz16, M>;
