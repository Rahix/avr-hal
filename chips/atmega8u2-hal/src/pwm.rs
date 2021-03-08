//! Support for PWM pins
//!
//! The 2 timers of ATmega8U2 can be used for PWM on certain pins.
//! The PWM methods are from `embedded_hal::PwmPin`.
//!
//! # Example
//! ```
//! let mut portb = dp.PORTB.split();
//! let mut timer1 = Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale64);
//!
//! let mut pb7 = portb.pb7.into_output(&mut portb.ddr).into_pwm(&mut timer1);
//!
//! pb7.set_duty(128);
//! pb7.enable();
//! ```
//!
//! Here is an overview of pins and which timer they work with:
//!
//! | Pin | Conversion Method | Alternate Conversion Method |
//! | --- | --- | --- |
//! | `PB7` | `.into_pwm(&mut timer0)` | `.into_pwm(&mut timer1)` |

use crate::port::portb;
pub use avr_hal_generic::pwm::*;

avr_hal_generic::impl_pwm! {
    /// Use `TC0` for PWM
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut timer0 = Timer0Pwm::new(dp.TC0, pwm::Prescaler::Prescale64);
    ///
    /// let mut pb7 = portb.pb7.into_output(&mut portb.ddr).into_pwm(&mut timer0);
    ///
    /// pb7.set_duty(128);
    /// pb7.enable();
    /// ```
    pub struct Timer0Pwm {
        timer: crate::pac::TC0,
        init: |tim, prescaler| {
            tim.tccr0a.modify(|_, w| w.wgm0().pwm_fast());
            tim.tccr0b.modify(|_, w| match prescaler {
                Prescaler::Direct => w.cs0().direct(),
                Prescaler::Prescale8 => w.cs0().prescale_8(),
                Prescaler::Prescale64 => w.cs0().prescale_64(),
                Prescaler::Prescale256 => w.cs0().prescale_256(),
                Prescaler::Prescale1024 => w.cs0().prescale_1024(),
            });
        },
        pins: {
            portb::PB7: {
                ocr: ocr0a,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_, w| w.com0a().match_clear());
                } else {
                    tim.tccr0a.modify(|_, w| w.com0a().disconnected());
                },
            },
        },
    }
}

avr_hal_generic::impl_pwm! {
    /// Use `TC1` for PWM
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut timer1 = Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale64);
    ///
    /// let mut pb7 = portb.pb7.into_output(&mut portb.ddr).into_pwm1(&mut timer1);
    ///
    /// pb7.set_duty(128);
    /// pb7.enable();
    /// ```
    pub struct Timer1Pwm {
        timer: crate::pac::TC1,
        init: |tim, prescaler| {
            tim.tccr1a.modify(|_, w| w.wgm1().bits(0b01));
            tim.tccr1b.modify(|_, w| {
                w.wgm1().bits(0b01);
                match prescaler {
                    Prescaler::Direct => w.cs1().direct(),
                    Prescaler::Prescale8 => w.cs1().prescale_8(),
                    Prescaler::Prescale64 => w.cs1().prescale_64(),
                    Prescaler::Prescale256 => w.cs1().prescale_256(),
                    Prescaler::Prescale1024 => w.cs1().prescale_1024(),
                }
            });
        },
        pins: {
            portb::PB7: {
                ocr: ocr1c,
                into_pwm1: |tim| if enable {
                    tim.tccr1a.modify(|_, w| w.com1c().match_clear());
                } else {
                    tim.tccr1a.modify(|_, w| w.com1c().disconnected());
                },
            },
        },
    }
}
