//! Support for PWM pins
//!
//! The 6 timers of ATmega2560 can be used for PWM on certain pins.
//! The PWM methods are from `embedded_hal::PwmPin`;
//!
//! # Example
//! ```
//! let mut portd = dp.PORTD.split();
//! let mut timer0 = Timer0Pwm::new(dp.TC0, pwm::Prescaler::Prescale64);
//!
//! let mut pb7 = portb.pb7.into_output(&mut portb.ddr).into_pwm(&mut timer0);
//!
//! pb7.set_duty(128);
//! pb7.enable();
//! ```
//!
//! Here is an overview of pins and which timer they work with:
//!
//! | Pin | Conversion Method | Alternate Conversion Method |
//! | --- | --- | --- |
//! | `PB4` | `.into_pwm(&mut timer2)` | |
//! | `PB5` | `.into_pwm(&mut timer1)` | |
//! | `PB6` | `.into_pwm(&mut timer1)` | |
//! | `PB7` | `.into_pwm(&mut timer0)` | `.into_pwm1(&mut timer1)` |
//! | `PE3` | `.into_pwm(&mut timer3)` | |
//! | `PE4` | `.into_pwm(&mut timer3)` | |
//! | `PE5` | `.into_pwm(&mut timer3)` | |
//! | `PG5` | `.into_pwm(&mut timer0)` | |
//! | `PH3` | `.into_pwm(&mut timer4)` | |
//! | `PH4` | `.into_pwm(&mut timer4)` | |
//! | `PH5` | `.into_pwm(&mut timer4)` | |
//! | `PH6` | `.into_pwm(&mut timer2)` | |
//! | `PL3` | `.into_pwm(&mut timer5)` | |
//! | `PL4` | `.into_pwm(&mut timer5)` | |
//! | `PL5` | `.into_pwm(&mut timer5)` | |

use crate::port::{portb, porte, portg, porth, portl};
pub use avr_hal::pwm::*;

avr_hal::impl_pwm! {
    /// Use `TC0` for PWM (pins `PB7`, `PG5`)
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut portg = dp.PORTG.split();
    /// let mut timer0 = Timer0Pwm::new(dp.TC0, pwm::Prescaler::Prescale64);
    ///
    /// let mut pb7 = portb.pb7.into_output(&mut portb.ddr).into_pwm(&mut timer0);
    /// let mut pg5 = portg.pg5.into_output(&mut portg.ddr).into_pwm(&mut timer0);
    ///
    /// pb7.set_duty(128);
    /// pb7.enable();
    /// ```
    pub struct Timer0Pwm {
        timer: crate::atmega2560::TC0,
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
            portg::PG5: {
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
    /// let mut timer1 = Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale64);
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
        timer: crate::atmega2560::TC1,
        init: |tim, prescaler| {
            tim.tccr1a.modify(|_, w| w.wgm1().bits(0b01));
            tim.tccr1b.modify(|_, w|  {
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
    /// Use `TC2` for PWM (pins `PB4`, `PH6`)
    ///
    /// # Example
    /// ```
    /// let mut portb = dp.PORTB.split();
    /// let mut porth = dp.PORTH.split();
    /// let mut timer2 = Timer2Pwm::new(dp.TC2, pwm::Prescaler::Prescale64);
    ///
    /// let mut pb4 = portb.into_output(&mut portb.ddr).into_pwm(&mut timer2);
    /// let mut ph6 = porth.into_output(&mut porth.ddr).into_pwm(&mut timer2);
    ///
    /// pb4.set_duty(128);
    /// pb4.enable();
    /// ```
    pub struct Timer2Pwm {
        timer: crate::atmega2560::TC2,
        init: |tim, prescaler| {
            tim.tccr2a.modify(|_, w| w.wgm2().bits(0b01));
            tim.tccr2b.modify(|_, w| {
                w.wgm22().clear_bit();
                match prescaler {
                    Prescaler::Direct => w.cs2().direct(),
                    Prescaler::Prescale8 => w.cs2().prescale_8(),
                    Prescaler::Prescale64 => w.cs2().prescale_64(),
                    Prescaler::Prescale256 => w.cs2().prescale_256(),
                    Prescaler::Prescale1024 => w.cs2().prescale_1024(),
                }
            });
        },
        pins: {
            portb::PB4: {
                ocr: ocr2a,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_, w| w.com2a().match_clear());
                } else {
                    tim.tccr2a.modify(|_, w| w.com2a().disconnected());
                },
            },
            porth::PH6: {
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

avr_hal::impl_pwm! {
    /// Use `TC3` for PWM (pins `PE3`, `PE4`, `PE5`)
    ///
    /// # Example
    /// ```
    /// let mut porte = dp.PORTE.split();
    /// let mut timer3 = Timer3Pwm::new(dp.TC3, pwm::Prescaler::Prescale64);
    ///
    /// let mut pe3 = porte.pe3.into_output(&mut porte.ddr).into_pwm(&mut timer3);
    /// let mut pe4 = porte.pe4.into_output(&mut porte.ddr).into_pwm(&mut timer3);
    /// let mut pe5 = porte.pe5.into_output(&mut porte.ddr).into_pwm(&mut timer3);
    ///
    /// pe3.set_duty(128);
    /// pe3.enable();
    /// ```
    pub struct Timer3Pwm {
        timer: crate::atmega2560::TC3,
        init: |tim, prescaler| {
            tim.tccr3a.modify(|_, w| w.wgm3().bits(0b01));
            tim.tccr3b.modify(|_, w| {
                w.wgm3().bits(0b01);
                match prescaler {
                    Prescaler::Direct => w.cs3().direct(),
                    Prescaler::Prescale8 => w.cs3().prescale_8(),
                    Prescaler::Prescale64 => w.cs3().prescale_64(),
                    Prescaler::Prescale256 => w.cs3().prescale_256(),
                    Prescaler::Prescale1024 => w.cs3().prescale_1024(),
                }
            });
        },
        pins: {
            porte::PE3: {
                ocr: ocr3a,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_, w| w.com3a().match_clear());
                } else {
                    tim.tccr3a.modify(|_, w| w.com3a().disconnected());
                },
            },
            porte::PE4: {
                ocr: ocr3b,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_, w| w.com3b().match_clear());
                } else {
                    tim.tccr3a.modify(|_, w| w.com3b().disconnected());
                },
            },
            porte::PE5: {
                ocr: ocr3c,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_, w| w.com3c().match_clear());
                } else {
                    tim.tccr3a.modify(|_, w| w.com3c().disconnected());
                },
            },
        },
    }
}

avr_hal::impl_pwm! {
    /// Use `TC4` for PWM (pins `PH3`, `PH4`, `PH5`)
    ///
    /// # Example
    /// ```
    /// let mut porth = dp.PORTH.split();
    /// let mut timer4 = Timer4Pwm::new(dp.TC4, pwm::Prescaler::Prescale64);
    ///
    /// let mut ph3 = porth.ph3.into_output(&mut porth.ddr).into_pwm(&mut timer4);
    /// let mut ph4 = porth.ph4.into_output(&mut porth.ddr).into_pwm(&mut timer4);
    /// let mut ph5 = porth.ph5.into_output(&mut porth.ddr).into_pwm(&mut timer4);
    ///
    /// ph3.set_duty(128);
    /// ph3.enable();
    /// ```
    pub struct Timer4Pwm {
        timer: crate::atmega2560::TC4,
        init: |tim, prescaler| {
            tim.tccr4a.modify(|_, w| w.wgm4().bits(0b01));
            tim.tccr4b.modify(|_, w| {
                w.wgm4().bits(0b01);
                match prescaler {
                    Prescaler::Direct => w.cs4().direct(),
                    Prescaler::Prescale8 => w.cs4().prescale_8(),
                    Prescaler::Prescale64 => w.cs4().prescale_64(),
                    Prescaler::Prescale256 => w.cs4().prescale_256(),
                    Prescaler::Prescale1024 => w.cs4().prescale_1024(),
                }
            });
        },
        pins: {
            porth::PH3: {
                ocr: ocr4a,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_, w| w.com4a().match_clear());
                } else {
                    tim.tccr4a.modify(|_, w| w.com4a().disconnected());
                },
            },
            porth::PH4: {
                ocr: ocr4b,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_, w| w.com4b().match_clear());
                } else {
                    tim.tccr4a.modify(|_, w| w.com4b().disconnected());
                },
            },
            porth::PH5: {
                ocr: ocr4c,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_, w| w.com4c().match_clear());
                } else {
                    tim.tccr4a.modify(|_, w| w.com4c().disconnected());
                },
            },
        },
    }
}

avr_hal::impl_pwm! {
    /// Use `TC5` for PWM (pins `PL3`, `PL4`, `PL5`)
    ///
    /// # Example
    /// ```
    /// let mut portl = dp.PORTL.split();
    /// let mut timer5 = Timer5Pwm::new(dp.TC5, pwm::Prescaler::Prescale64);
    ///
    /// let mut pl3 = portl.pl3.into_output(&mut portl.ddr).into_pwm(&mut timer5);
    /// let mut pl4 = portl.pl4.into_output(&mut portl.ddr).into_pwm(&mut timer5);
    /// let mut pl5 = portl.pl5.into_output(&mut portl.ddr).into_pwm(&mut timer5);
    ///
    /// pl3.set_duty(128);
    /// pl3.enable();
    /// ```
    pub struct Timer5Pwm {
        timer: crate::atmega2560::TC5,
        init: |tim, prescaler| {
            tim.tccr5a.modify(|_, w| w.wgm5().bits(0b01));
            tim.tccr5b.modify(|_, w| {
                w.wgm5().bits(0b01);
                match prescaler {
                    Prescaler::Direct => w.cs5().direct(),
                    Prescaler::Prescale8 => w.cs5().prescale_8(),
                    Prescaler::Prescale64 => w.cs5().prescale_64(),
                    Prescaler::Prescale256 => w.cs5().prescale_256(),
                    Prescaler::Prescale1024 => w.cs5().prescale_1024(),
                }
            });
        },
        pins: {
            portl::PL3: {
                ocr: ocr5a,
                into_pwm: |tim| if enable {
                    tim.tccr5a.modify(|_, w| w.com5a().match_clear());
                } else {
                    tim.tccr5a.modify(|_, w| w.com5a().disconnected());
                },
            },
            portl::PL4: {
                ocr: ocr5b,
                into_pwm: |tim| if enable {
                    tim.tccr5a.modify(|_, w| w.com5b().match_clear());
                } else {
                    tim.tccr5a.modify(|_, w| w.com5b().disconnected());
                },
            },
            portl::PL5: {
                ocr: ocr5c,
                into_pwm: |tim| if enable {
                    tim.tccr5a.modify(|_, w| w.com5c().match_clear());
                } else {
                    tim.tccr5a.modify(|_, w| w.com5c().disconnected());
                },
            },
        },
    }
}
