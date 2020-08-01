//! Support for PWM pins
//!
//! The 4 timers of ATmega32U4 can be used for PWM on certain pins.
//! The PWM methods are from `embedded_hal::PwmPin`.
//!
//! # Example
//! ```
//! let mut portb = dp.PORTB.split();
//! let mut timer1 = Timer1Pwm::new(dp.TC1);
//!
//! let mut pb5 = portb.pb5.into_output(&mut portb.ddr).into_pwm(&mut timer1);
//!
//! pb5.set_duty(128);
//! pb5.enable();
//! ```
//!
//! Here is an overview of pins and which timer they work with:
//!
//! | Pin | Conversion Method | Alternate Conversion Method |
//! | --- | --- | --- |
//! | `PB5` | `.into_pwm(&mut timer1)` | |
//! | `PB6` | `.into_pwm(&mut timer1)` | `.into_pwm4(&mut timer4)` |
//! | `PB7` | `.into_pwm(&mut timer0)` | `.into_pwm1(&mut timer1)` |
//! | `PC6` | `.into_pwm(&mut timer3)` | |
//! | `PC7` | `.into_pwm(&mut timer4)` | |
//! | `PD0` | `.into_pwm(&mut timer0)` | |
//! | `PD7` | `.into_pwm(&mut timer4)` | |

use crate::port::{portb, portc, portd};

avr_hal::impl_pwm! {
    /// Use `TC0` for PWM (pins `PB7`, `PD0`)
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut portd = dp.PORTD.split();
    /// let mut timer0 = Timer0Pwm::new(dp.TC0);
    ///
    /// let mut pb7 = portb.pb7.into_output(&mut portb.ddr).into_pwm(&mut timer0);
    /// let mut pd0 = portd.pd0.into_output(&mut portd.ddr).into_pwm(&mut timer0);
    ///
    /// pb7.set_duty(128);
    /// pb7.enable();
    /// ```
    pub struct Timer0Pwm {
        timer: crate::atmega32u4::TC0,
        init: |tim| {
            tim.tccr0a.modify(|_, w| w.wgm0().pwm_fast());
            tim.tccr0b.modify(|_, w| w.cs0().prescale_64());
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
            portd::PD0: {
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
    /// Use `TC1` for PWM (pins `PB5`, `PB6`, `PB7`)
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut timer1 = Timer1Pwm::new(dp.TC1);
    ///
    /// let mut pb5 = portb.pb5.into_output(&mut portb.ddr).into_pwm(&mut timer1);
    /// let mut pb6 = portb.pb6.into_output(&mut portb.ddr).into_pwm(&mut timer1);
    /// let mut pb7 = portb.pb7.into_output(&mut portb.ddr).into_pwm1(&mut timer1);
    ///
    /// pb5.set_duty(128);
    /// pb5.enable();
    /// ```
    ///
    /// **Note**: For `PB7` the method is called `into_pwm1()`!
    pub struct Timer1Pwm {
        timer: crate::atmega32u4::TC1,
        init: |tim| {
            tim.tccr1a.modify(|_, w| w.wgm1().bits(0b01));
            tim.tccr1b.modify(|_, w| w.wgm1().bits(0b01).cs1().prescale_64());
        },
        pins: {
            portb::PB5: {
                ocr: ocr1a,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_, w| w.com1a().match_clear());
                } else {
                    tim.tccr1a.modify(|_, w| w.com1a().disconnected());
                },
            },
            portb::PB6: {
                ocr: ocr1b,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_, w| w.com1b().match_clear());
                } else {
                    tim.tccr1a.modify(|_, w| w.com1b().disconnected());
                },
            },
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

avr_hal::impl_pwm! {
    /// Use `TC3` for PWM (pin `PC6`)
    ///
    /// # Example
    /// ```
    /// let mut portc = dp.PORTC.split();
    /// let mut timer3 = Timer3Pwm::new(dp.TC3);
    ///
    /// let mut pc6 = portc.pc6.into_output(&mut portc.ddr).into_pwm(&mut timer3);
    ///
    /// pc6.set_duty(128);
    /// pc6.enable();
    /// ```
    pub struct Timer3Pwm {
        timer: crate::atmega32u4::TC3,
        init: |tim| {
            tim.tccr3a.modify(|_, w| w.wgm3().bits(0b01));
            tim.tccr3b.modify(|_, w| w.wgm3().bits(0b01).cs3().prescale_64());
        },
        pins: {
            portc::PC6: {
                ocr: ocr3a,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_, w| w.com3a().match_clear());
                } else {
                    tim.tccr3a.modify(|_, w| w.com3a().disconnected());
                },
            },
        },
    }
}

avr_hal::impl_pwm! {
    /// Use `TC4` for PWM (pins `PB6`, `PC7`, `PD7`)
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut portc = dp.PORTC.split();
    /// let mut portd = dp.PORTD.split();
    /// let mut timer4 = Timer4Pwm::new(dp.TC4);
    ///
    /// let pb6 = portb.pb6.into_output(&mut portb.ddr).into_pwm4(&mut timer4);
    /// let pc7 = portc.pc7.into_output(&mut portc.ddr).into_pwm(&mut timer4);
    /// let pd7 = portd.pd7.into_output(&mut portd.ddr).into_pwm(&mut timer4);
    ///
    /// pb6.set_duty(128);
    /// pb6.enable();
    /// ```
    ///
    /// **Note**: For `PB6` the method is called `into_pwm6()`!
    pub struct Timer4Pwm {
        timer: crate::atmega32u4::TC4,
        init: |tim| {
            tim.tccr4b.modify(|_, w| w.cs4().prescale_64());
            tim.tccr4d.modify(|_, w| w.wgm4().pwm_correct());
        },
        pins: {
            portc::PC7: {
                ocr: ocr4a,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_, w| w.com4a().match_clear().pwm4a().set_bit());
                } else {
                    tim.tccr4a.modify(|_, w| w.com4a().disconnected().pwm4a().clear_bit());
                },
            },
            portb::PB6: {
                ocr: ocr4b,
                into_pwm4: |tim| if enable {
                    tim.tccr4a.modify(|_, w| w.com4b().match_clear().pwm4b().set_bit());
                } else {
                    tim.tccr4a.modify(|_, w| w.com4b().disconnected().pwm4b().clear_bit());
                },
            },
            portd::PD7: {
                ocr: ocr4d,
                into_pwm: |tim| if enable {
                    tim.tccr4c.modify(|_, w| w.com4d().match_clear().pwm4d().set_bit());
                } else {
                    tim.tccr4c.modify(|_, w| w.com4d().disconnected().pwm4d().clear_bit());
                },
            },
        },
    }
}
