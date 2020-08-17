//! Support for PWM pins
//!
//! The 3 timers of ATmega48P can be used for PWM on certain pins.
//! The PWM methods are from `embedded_hal::PwmPin`.
//!
//! # Example
//! ```
//! let mut portd = dp.PORTD.split();
//! let mut timer0 = Timer0Pwm::new(dp.TC0, pwm::Prescaler::Prescale64);
//!
//! let mut pd5 = portd.pd5.into_output(&mut portd.ddr).into_pwm(&mut timer0);
//!
//! pd5.set_duty(128);
//! pd5.enable();
//! ```
//!
//! Here is an overview of pins and which timer they work with:
//!
//! | Pin | Conversion Method |
//! | --- | --- |
//! | `PB1` | `.into_pwm(&mut timer1)` |
//! | `PB2` | `.into_pwm(&mut timer1)` |
//! | `PB3` | `.into_pwm(&mut timer2)` |
//! | `PD3` | `.into_pwm(&mut timer2)` |
//! | `PD5` | `.into_pwm(&mut timer0)` |
//! | `PD6` | `.into_pwm(&mut timer0)` |

use crate::port::{portb, portd};
pub use avr_hal::pwm::*;

avr_hal::impl_pwm! {
    /// Use `TC0` for PWM (pins `PD5`, `PD6`)
    ///
    /// # Example
    /// ```
    /// let mut portd = dp.PORTD.split();
    /// let mut timer0 = Timer0Pwm::new(dp.TC0, pwm::Prescaler::Prescale64);
    ///
    /// let mut pd5 = portd.pd5.into_output(&mut portd.ddr).into_pwm(&mut timer0);
    /// let mut pd6 = portd.pd6.into_output(&mut portd.ddr).into_pwm(&mut timer0);
    ///
    /// pd5.set_duty(128);
    /// pd5.enable();
    /// ```
    pub struct Timer0Pwm {
        timer: crate::atmega48p::TC0,
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
            portd::PD6: {
                ocr: ocr0a,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_, w| w.com0a().match_clear());
                } else {
                    tim.tccr0a.modify(|_, w| w.com0a().disconnected());
                },
            },
            portd::PD5: {
                ocr: ocr0b,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_, w| w.com0b().match_clear());
                } else {
                    tim.tccr0a.modify(|_, w| w.com0b().disconnected());
                },
            },
        },
    }
}

avr_hal::impl_pwm! {
    /// Use `TC1` for PWM (pins `PB1`, `PB2`)
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut timer1 = Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale64);
    ///
    /// let mut pb1 = portb.pb1.into_output(&mut portb.ddr).into_pwm(&mut timer1);
    /// let mut pb2 = portb.pb2.into_output(&mut portb.ddr).into_pwm(&mut timer1);
    ///
    /// pb1.set_duty(128);
    /// pb1.enable();
    /// ```
    pub struct Timer1Pwm {
        timer: crate::atmega48p::TC1,
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
            portb::PB1: {
                ocr: ocr1a,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_, w| w.com1a().match_clear());
                } else {
                    tim.tccr1a.modify(|_, w| w.com1a().disconnected());
                },
            },
            portb::PB2: {
                ocr: ocr1b,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_, w| w.com1b().match_clear());
                } else {
                    tim.tccr1a.modify(|_, w| w.com1b().disconnected());
                },
            },
        },
    }
}

avr_hal::impl_pwm! {
    /// Use `TC2` for PWM (pins `PB3`, `PD3`)
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut portd = dp.PORTD.split();
    /// let mut timer2 = Timer2Pwm::new(dp.TC2, pwm::Prescaler::Prescale64);
    ///
    /// let mut pb3 = portb.pb3.into_output(&mut portb.ddr).into_pwm(&mut timer2);
    /// let mut pd3 = portd.pd3.into_output(&mut portd.ddr).into_pwm(&mut timer2);
    ///
    /// pb3.set_duty(128);
    /// pb3.enable();
    /// ```
    pub struct Timer2Pwm {
        timer: crate::atmega48p::TC2,
        init: |tim, prescaler| {
            tim.tccr2a.modify(|_, w| w.wgm2().pwm_fast());
            tim.tccr2b.modify(|_, w| match prescaler {
                Prescaler::Direct => w.cs2().direct(),
                Prescaler::Prescale8 => w.cs2().prescale_8(),
                Prescaler::Prescale64 => w.cs2().prescale_64(),
                Prescaler::Prescale256 => w.cs2().prescale_256(),
                Prescaler::Prescale1024 => w.cs2().prescale_1024(),
            });
        },
        pins: {
            portb::PB3: {
                ocr: ocr2a,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_, w| w.com2a().match_clear());
                } else {
                    tim.tccr2a.modify(|_, w| w.com2a().disconnected());
                },
            },
            portd::PD3: {
                ocr: ocr2b,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_, w| w.com2b().match_clear());
                } else {
                    tim.tccr2a.modify(|_, w| w.com2b().disconnected());
                },
            },
        },
    }
}
