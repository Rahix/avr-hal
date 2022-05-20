//! Simple PWM output for supported Pins.
//!
//! This module implements simple (FastPWM) PWM output for supported Pins.
//!
//! Check the documentation for each of the TimerXPwm-structs for usage
//! examples.

pub use avr_hal_generic::simple_pwm::IntoPwmPin;
pub use avr_hal_generic::simple_pwm::Prescaler;

#[cfg(feature = "mcu-atmega")]
pub use atmega_hal::simple_pwm::*;

#[cfg(feature = "mcu-attiny")]
pub use attiny_hal::simple_pwm::*;