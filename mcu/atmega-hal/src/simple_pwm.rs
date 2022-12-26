pub use avr_hal_generic::simple_pwm::{PwmPinOps, Prescaler};

use crate::port::*;

#[cfg(any(
    feature = "atmega48p",
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega328pb"
))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC0` for PWM (pins `PD5`, `PD6`)
    ///
    /// # Example
    /// ```
    /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    ///
    /// let mut d5 = pins.d5.into_output().into_pwm(&mut timer0);
    /// let mut d6 = pins.d6.into_output().into_pwm(&mut timer0);
    ///
    /// d5.set_duty(128);
    /// d5.enable();
    /// ```
    pub struct Timer0Pwm {
        timer: crate::pac::TC0,
        init: |tim, prescaler| {
            tim.tccr0a.modify(|_r, w| w.wgm0().pwm_fast());
            tim.tccr0b.modify(|_r, w| match prescaler {
                Prescaler::Direct => w.cs0().direct(),
                Prescaler::Prescale8 => w.cs0().prescale_8(),
                Prescaler::Prescale64 => w.cs0().prescale_64(),
                Prescaler::Prescale256 => w.cs0().prescale_256(),
                Prescaler::Prescale1024 => w.cs0().prescale_1024(),
            });
        },
        pins: {
            PD6: {
                ocr: ocr0a,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                } else {
                    tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                },
            },

            PD5: {
                ocr: ocr0b,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_r, w| w.com0b().match_clear());
                } else {
                    tim.tccr0a.modify(|_r, w| w.com0b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(
    feature = "atmega48p",
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega328pb"
))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC1` for PWM (pins `PB1`, `PB2`)
    ///
    /// # Example
    /// ```
    /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    ///
    /// let mut d9 = pins.d9.into_output().into_pwm(&mut timer1);
    /// let mut d10 = pins.d10.into_output().into_pwm(&mut timer1);
    ///
    /// d9.set_duty(128);
    /// d9.enable();
    /// ```
    pub struct Timer1Pwm {
        timer: crate::pac::TC1,
        init: |tim, prescaler| {
            tim.tccr1a.modify(|_r, w| w.wgm1().bits(0b01));
            tim.tccr1b.modify(|_r, w| {
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
            PB1: {
                ocr: ocr1a,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1a().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1a().disconnected());
                },
            },

            PB2: {
                ocr: ocr1b,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1b().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(
    feature = "atmega48p",
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega328pb"
))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC2` for PWM (pins `PB3`, `PD3`)
    ///
    /// # Example
    /// ```
    /// let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    ///
    /// let mut d11 = pins.d11.into_output().into_pwm(&mut timer2);
    /// let mut d3 = pins.d3.into_output().into_pwm(&mut timer2);
    ///
    /// d11.set_duty(128);
    /// d11.enable();
    /// ```
    pub struct Timer2Pwm {
        timer: crate::pac::TC2,
        init: |tim, prescaler| {
            tim.tccr2a.modify(|_r, w| w.wgm2().pwm_fast());
            tim.tccr2b.modify(|_r, w| match prescaler {
                    Prescaler::Direct => w.cs2().direct(),
                    Prescaler::Prescale8 => w.cs2().prescale_8(),
                    Prescaler::Prescale64 => w.cs2().prescale_64(),
                    Prescaler::Prescale256 => w.cs2().prescale_256(),
                    Prescaler::Prescale1024 => w.cs2().prescale_1024(),
            });
        },
        pins: {
            PB3: {
                ocr: ocr2a,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_r, w| w.com2a().match_clear());
                } else {
                    tim.tccr2a.modify(|_r, w| w.com2a().disconnected());
                },
            },

            PD3: {
                ocr: ocr2b,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_r, w| w.com2b().match_clear());
                } else {
                    tim.tccr2a.modify(|_r, w| w.com2b().disconnected());
                },
            },
        },
    }
}

#[cfg(feature = "atmega328pb")]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC3` for PWM (pins `PD0`, `PD2`)
    pub struct Timer3Pwm {
        timer: crate::pac::TC3,
        init: |tim, prescaler| {
            tim.tccr3a.modify(|_r, w| w.wgm3().bits(0b01));
            tim.tccr3b.modify(|_r, w| {
                unsafe { w.wgm3().bits(0b01) };

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
            PD0: {
                ocr: ocr3a,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_r, w| w.com3a().match_clear());
                } else {
                    tim.tccr3a.modify(|_r, w| w.com3a().disconnected());
                },
            },

            PD2: {
                ocr: ocr3b,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_r, w| w.com3b().match_clear());
                } else {
                    tim.tccr3a.modify(|_r, w| w.com3b().disconnected());
                },
            },
        },
    }
}

#[cfg(feature = "atmega328pb")]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC4` for PWM (pins `PD1`, `PD2`)
    pub struct Timer4Pwm {
        timer: crate::pac::TC4,
        init: |tim, prescaler| {
            tim.tccr4a.modify(|_r, w| w.wgm4().bits(0b01));
            tim.tccr4b.modify(|_r, w| {
                unsafe { w.wgm4().bits(0b01) };

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
            PD1: {
                ocr: ocr4a,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_r, w| w.com4a().match_clear());
                } else {
                    tim.tccr4a.modify(|_r, w| w.com4a().disconnected());
                },
            },

            PD2: {
                ocr: ocr4b,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_r, w| w.com4b().match_clear());
                } else {
                    tim.tccr4a.modify(|_r, w| w.com4b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC0` for PWM (pins `PB7`, `PG5`)
    ///
    /// # Example
    /// ```
    /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    ///
    /// let mut d13 = pins.d13.into_output().into_pwm(&mut timer0);
    /// let mut d4 = pins.d4.into_output().into_pwm(&mut timer0);
    ///
    /// d13.set_duty(128);
    /// d13.enable();
    /// ```
    pub struct Timer0Pwm {
        timer: crate::pac::TC0,
        init: |tim, prescaler| {
            tim.tccr0a.modify(|_r, w| w.wgm0().pwm_fast());
            tim.tccr0b.modify(|_r, w| match prescaler {
                Prescaler::Direct => w.cs0().direct(),
                Prescaler::Prescale8 => w.cs0().prescale_8(),
                Prescaler::Prescale64 => w.cs0().prescale_64(),
                Prescaler::Prescale256 => w.cs0().prescale_256(),
                Prescaler::Prescale1024 => w.cs0().prescale_1024(),
            });
        },
        pins: {
            PB7: {
                ocr: ocr0a,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                } else {
                    tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                },
            },

            PG5: {
                ocr: ocr0b,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_r, w| w.com0b().match_clear());
                } else {
                    tim.tccr0a.modify(|_r, w| w.com0b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC1` for PWM (pins `PB5`, `PB6`, `PB7`)
    ///
    /// # Example
    /// ```
    /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    ///
    /// let mut d11 = pins.d11.into_output().into_pwm(&mut timer1);
    /// let mut d12 = pins.d12.into_output().into_pwm(&mut timer1);
    /// let mut d13 = pins.d13.into_output().into_pwm(&mut timer1);
    ///
    /// d11.set_duty(128);
    /// d11.enable();
    /// ```
    pub struct Timer1Pwm {
        timer: crate::pac::TC1,
        init: |tim, prescaler| {
            tim.tccr1a.modify(|_r, w| w.wgm1().bits(0b01));
            tim.tccr1b.modify(|_r, w| match prescaler {
                Prescaler::Direct => w.cs1().direct(),
                Prescaler::Prescale8 => w.cs1().prescale_8(),
                Prescaler::Prescale64 => w.cs1().prescale_64(),
                Prescaler::Prescale256 => w.cs1().prescale_256(),
                Prescaler::Prescale1024 => w.cs1().prescale_1024(),
            });
        },
        pins: {
            PB5: {
                ocr: ocr1a,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1a().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1a().disconnected());
                },
            },

            PB6: {
                ocr: ocr1b,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1b().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1b().disconnected());
                },
            },

            PB7: {
                ocr: ocr1c,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1c().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1c().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC2` for PWM (pins `PB4`, `PH6`)
    ///
    /// # Example
    /// ```
    /// let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    ///
    /// let mut d10 = pins.d10.into_output().into_pwm(&mut timer2);
    /// let mut d9 = pins.d9.into_output().into_pwm(&mut timer2);
    ///
    /// d10.set_duty(128);
    /// d10.enable();
    /// ```

    pub struct Timer2Pwm {
        timer: crate::pac::TC2,
        init: |tim, prescaler| {
            tim.tccr2a.modify(|_r, w| w.wgm2().bits(0b01));
            tim.tccr2b.modify(|_r, w| {
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
            PB4: {
                ocr: ocr2a,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_r, w| w.com2a().match_clear());
                } else {
                    tim.tccr2a.modify(|_r, w| w.com2a().disconnected());
                },
            },

            PH6: {
                ocr: ocr2b,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_r, w| w.com2b().match_clear());
                } else {
                    tim.tccr2a.modify(|_r, w| w.com2b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC3` for PWM (pins `PE3`, `PE4`, `PE5`)
    ///
    /// # Example
    /// ```
    /// let mut timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);
    ///
    /// let mut d5 = pins.d5.into_output().into_pwm(&mut timer3);
    /// let mut d2 = pins.d2.into_output().into_pwm(&mut timer3);
    /// let mut d3 = pins.d3.into_output().into_pwm(&mut timer3);
    ///
    /// d5.set_duty(128);
    /// d5.enable();
    /// ```
    pub struct Timer3Pwm {
        timer: crate::pac::TC3,
        init: |tim, prescaler| {
            tim.tccr3a.modify(|_r, w| w.wgm3().bits(0b01));
            tim.tccr3b.modify(|_r, w| {
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
            PE3: {
                ocr: ocr3a,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_r, w| w.com3a().match_clear());
                } else {
                    tim.tccr3a.modify(|_r, w| w.com3a().disconnected());
                },
            },

            PE4: {
                ocr: ocr3b,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_r, w| w.com3b().match_clear());
                } else {
                    tim.tccr3a.modify(|_r, w| w.com3b().disconnected());
                },
            },

            PE5: {
                ocr: ocr3c,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_r, w| w.com3c().match_clear());
                } else {
                    tim.tccr3a.modify(|_r, w| w.com3c().disconnected());
                },
            },

        },
    }
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC4` for PWM (pins `PH3`, `PH4`, `PH5`)
    ///
    /// # Example
    /// ```
    /// let mut timer4 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);
    ///
    /// let mut d6 = pins.d6.into_output().into_pwm(&mut timer4);
    /// let mut d7 = pins.d7.into_output().into_pwm(&mut timer4);
    /// let mut d8 = pins.d8.into_output().into_pwm(&mut timer4);
    ///
    /// d6.set_duty(128);
    /// d6.enable();
    /// ```
    pub struct Timer4Pwm {
        timer: crate::pac::TC4,
        init: |tim, prescaler| {
            tim.tccr4a.modify(|_r, w| w.wgm4().bits(0b01));
            tim.tccr4b.modify(|_r, w| {
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
            PH3: {
                ocr: ocr4a,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_r, w| w.com4a().match_clear());
                } else {
                    tim.tccr4a.modify(|_r, w| w.com4a().disconnected());
                },
            },

            PH4: {
                ocr: ocr4b,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_r, w| w.com4b().match_clear());
                } else {
                    tim.tccr4a.modify(|_r, w| w.com4b().disconnected());
                },
            },

            PH5: {
                ocr: ocr4c,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_r, w| w.com4c().match_clear());
                } else {
                    tim.tccr4a.modify(|_r, w| w.com4c().disconnected());
                },
            },

        },
    }
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC5` for PWM (pins `PL3`, `PL4`, `PL5`)
    ///
    /// # Example
    /// ```
    /// let mut timer5 = Timer5Pwm::new(dp.TC5, Prescaler::Prescale64);
    ///
    /// let mut d46 = pins.d46.into_output().into_pwm(&mut timer5);
    /// let mut d45 = pins.d45.into_output().into_pwm(&mut timer5);
    /// let mut d44 = pins.d44.into_output().into_pwm(&mut timer5);
    ///
    /// d46.set_duty(128);
    /// d46.enable();
    /// ```
    pub struct Timer5Pwm {
        timer: crate::pac::TC5,
        init: |tim, prescaler| {
            tim.tccr5a.modify(|_r, w| w.wgm5().bits(0b01));
            tim.tccr5b.modify(|_r, w| {
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
            PL3: {
                ocr: ocr5a,
                into_pwm: |tim| if enable {
                    tim.tccr5a.modify(|_r, w| w.com5a().match_clear());
                } else {
                    tim.tccr5a.modify(|_r, w| w.com5a().disconnected());
                },
            },

            PL4: {
                ocr: ocr5b,
                into_pwm: |tim| if enable {
                    tim.tccr5a.modify(|_r, w| w.com5b().match_clear());
                } else {
                    tim.tccr5a.modify(|_r, w| w.com5b().disconnected());
                },
            },

            PL5: {
                ocr: ocr5c,
                into_pwm: |tim| if enable {
                    tim.tccr5a.modify(|_r, w| w.com5c().match_clear());
                } else {
                    tim.tccr5a.modify(|_r, w| w.com5c().disconnected());
                },
            },

        },
    }
}

#[cfg(any(feature = "atmega32u4"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC0` for PWM (pins `PB7`, `PD0`)
    ///
    /// # Example
    /// ```
    /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    ///
    /// let mut d11 = pins.d11.into_output().into_pwm(&mut timer0);
    /// let mut d3 = pins.d3.into_output().into_pwm(&mut timer0);
    ///
    /// d11.set_duty(128);
    /// d11.enable();
    /// ```
    pub struct Timer0Pwm {
        timer: crate::pac::TC0,
        init: |tim, prescaler| {
            tim.tccr0a.modify(|_r, w| w.wgm0().pwm_fast());
            tim.tccr0b.modify(|_r, w| match prescaler {
                Prescaler::Direct => w.cs0().direct(),
                Prescaler::Prescale8 => w.cs0().prescale_8(),
                Prescaler::Prescale64 => w.cs0().prescale_64(),
                Prescaler::Prescale256 => w.cs0().prescale_256(),
                Prescaler::Prescale1024 => w.cs0().prescale_1024(),
            });
        },
        pins: {
            PB7: {
                ocr: ocr0a,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                } else {
                    tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                },
            },

            PD0: {
                ocr: ocr0b,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_r, w| w.com0b().match_clear());
                } else {
                    tim.tccr0a.modify(|_r, w| w.com0b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega32u4"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC1` for PWM (pins `PB5`, `PB6`, `PB7`)
    ///
    /// # Example
    /// ```
    /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    ///
    /// let mut d9 = pins.d9.into_output().into_pwm(&mut timer1);
    /// let mut d10 = pins.d10.into_output().into_pwm(&mut timer1);
    /// let mut d11 = pins.d11.into_output().into_pwm(&mut timer1);
    ///
    /// d9.set_duty(128);
    /// d9.enable();
    /// ```
    pub struct Timer1Pwm {
        timer: crate::pac::TC1,
        init: |tim, prescaler| {
            tim.tccr1a.modify(|_r, w| w.wgm1().bits(0b01));
            tim.tccr1b.modify(|_r, w| w.wgm1().bits(0b01));

            tim.tccr1b.modify(|_r, w| match prescaler {
                Prescaler::Direct => w.cs1().direct(),
                Prescaler::Prescale8 => w.cs1().prescale_8(),
                Prescaler::Prescale64 => w.cs1().prescale_64(),
                Prescaler::Prescale256 => w.cs1().prescale_256(),
                Prescaler::Prescale1024 => w.cs1().prescale_1024(),
            });
        },
        pins: {
            PB5: {
                ocr: ocr1a,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1a().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1a().disconnected());
                },
            },

            PB6: {
                ocr: ocr1b,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1b().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1b().disconnected());
                },
            },

            PB7: {
                ocr: ocr1c,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1c().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1c().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega32u4"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC3` for PWM (pins `PC6`)
    ///
    /// # Example
    /// ```
    /// let mut timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);
    ///
    /// let mut d5 = pins.d5.into_output().into_pwm(&mut timer3);
    ///
    /// d5.set_duty(128);
    /// d5.enable();
    /// ```
    pub struct Timer3Pwm {
        timer: crate::pac::TC3,
        init: |tim, prescaler| {
            tim.tccr3a.modify(|_r, w| w.wgm3().bits(0b01));
            tim.tccr3b.modify(|_r, w| w.wgm3().bits(0b01));

            tim.tccr3b.modify(|_r, w| match prescaler {
                Prescaler::Direct => w.cs3().direct(),
                Prescaler::Prescale8 => w.cs3().prescale_8(),
                Prescaler::Prescale64 => w.cs3().prescale_64(),
                Prescaler::Prescale256 => w.cs3().prescale_256(),
                Prescaler::Prescale1024 => w.cs3().prescale_1024(),
            });
        },
        pins: {
            PC6: {
                ocr: ocr3a,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_r, w| w.com3a().match_clear());
                } else {
                    tim.tccr3a.modify(|_r, w| w.com3a().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega32u4"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC4` for PWM (pins `PB6`, `PC7`, `PD7`)
    ///
    /// # Example
    /// ```
    /// let mut timer4 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);
    ///
    /// let mut d6 = pins.d6.into_output().into_pwm(&mut timer4);
    /// let mut d10 = pins.d10.into_output().into_pwm(&mut timer4);
    /// let mut d13 = pins.d13.into_output().into_pwm(&mut timer4);
    ///
    /// d6.set_duty(128);
    /// d6.enable();
    /// ```
    pub struct Timer4Pwm {
        timer: crate::pac::TC4,
        init: |tim, prescaler| {
            tim.tccr4a.modify(|_r, w| w.pwm4a().set_bit());
            tim.tccr4a.modify(|_r, w| w.pwm4b().set_bit());
            tim.tccr4c.modify(|_r, w| w.pwm4d().set_bit());

            tim.tccr4b.modify(|_r, w| match prescaler {
                Prescaler::Direct => w.cs4().direct(),
                Prescaler::Prescale8 => w.cs4().prescale_8(),
                Prescaler::Prescale64 => w.cs4().prescale_64(),
                Prescaler::Prescale256 => w.cs4().prescale_256(),
                Prescaler::Prescale1024 => w.cs4().prescale_1024(),
            });
        },
        pins: {
            PB6: {
                ocr: ocr4b,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_r, w| w.com4b().match_clear());
                } else {
                    tim.tccr4a.modify(|_r, w| w.com4b().disconnected());
                },
            },

            PC7: {
                ocr: ocr4a,
                into_pwm: |tim| if enable {
                    tim.tccr4a.modify(|_r, w| w.com4a().match_clear());
                } else {
                    tim.tccr4a.modify(|_r, w| w.com4a().disconnected());
                },
            },

            PD7: {
                ocr: ocr4d,
                into_pwm: |tim| if enable {
                    tim.tccr4c.modify(|_r, w| w.com4d().match_clear());
                } else {
                    tim.tccr4c.modify(|_r, w| w.com4d().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega1284p"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC0` for PWM (pins `PB3`, `PB4`)
    ///
    /// # Example
    /// ```
    /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    ///
    /// let mut b3 = pins.b3.into_output().into_pwm(&mut timer0);
    /// let mut b4 = pins.b4.into_output().into_pwm(&mut timer0);
    ///
    /// b3.set_duty(128);
    /// b4.enable();
    /// ```
    pub struct Timer0Pwm {
        timer: crate::pac::TC0,
        init: |tim, prescaler| {
            tim.tccr0a.modify(|_r, w| w.wgm0().pwm_fast());
            tim.tccr0b.modify(|_r, w| match prescaler {
                Prescaler::Direct => w.cs0().direct(),
                Prescaler::Prescale8 => w.cs0().prescale_8(),
                Prescaler::Prescale64 => w.cs0().prescale_64(),
                Prescaler::Prescale256 => w.cs0().prescale_256(),
                Prescaler::Prescale1024 => w.cs0().prescale_1024(),
            });
        },
        pins: {
            PB3: {
                ocr: ocr0a,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                } else {
                    tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                },
            },

            PB4: {
                ocr: ocr0b,
                into_pwm: |tim| if enable {
                    tim.tccr0a.modify(|_r, w| w.com0b().match_clear());
                } else {
                    tim.tccr0a.modify(|_r, w| w.com0b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega1284p"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC1` for PWM (pins `PD5`, `PD4`)
    ///
    /// # Example
    /// ```
    /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    ///
    /// let mut d5 = pins.d5.into_output().into_pwm(&mut timer1);
    /// let mut d4 = pins.d4.into_output().into_pwm(&mut timer1);
    ///
    /// d5.set_duty(128);
    /// d5.enable();
    /// ```
    pub struct Timer1Pwm {
        timer: crate::pac::TC1,
        init: |tim, prescaler| {
            tim.tccr1a.modify(|_r, w| w.wgm1().bits(0b01));
            tim.tccr1b.modify(|_r, w| {
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
            PD5: {
                ocr: ocr1a,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1a().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1a().disconnected());
                },
            },

            PD4: {
                ocr: ocr1b,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_r, w| w.com1b().match_clear());
                } else {
                    tim.tccr1a.modify(|_r, w| w.com1b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega1284p"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC2` for PWM (pins `PD7`, `PD6`)
    ///
    /// # Example
    /// ```
    /// let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    ///
    /// let mut d7 = pins.d7.into_output().into_pwm(&mut timer2);
    /// let mut d6 = pins.d6.into_output().into_pwm(&mut timer2);
    ///
    /// d7.set_duty(128);
    /// d7.enable();
    /// ```
    pub struct Timer2Pwm {
        timer: crate::pac::TC2,
        init: |tim, prescaler| {
            tim.tccr2a.modify(|_r, w| w.wgm2().pwm_fast());
            tim.tccr2b.modify(|_r, w| match prescaler {
                    Prescaler::Direct => w.cs2().direct(),
                    Prescaler::Prescale8 => w.cs2().prescale_8(),
                    Prescaler::Prescale64 => w.cs2().prescale_64(),
                    Prescaler::Prescale256 => w.cs2().prescale_256(),
                    Prescaler::Prescale1024 => w.cs2().prescale_1024(),
            });
        },
        pins: {
            PD7: {
                ocr: ocr2a,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_r, w| w.com2a().match_clear());
                } else {
                    tim.tccr2a.modify(|_r, w| w.com2a().disconnected());
                },
            },

            PD6: {
                ocr: ocr2b,
                into_pwm: |tim| if enable {
                    tim.tccr2a.modify(|_r, w| w.com2b().match_clear());
                } else {
                    tim.tccr2a.modify(|_r, w| w.com2b().disconnected());
                },
            },
        },
    }
}

#[cfg(any(feature = "atmega1284p"))]
avr_hal_generic::impl_simple_pwm! {
    /// Use `TC3` for PWM (pins `PB6`, `PB7`)
    pub struct Timer3Pwm {
        timer: crate::pac::TC3,
        init: |tim, prescaler| {
            tim.tccr3a.modify(|_r, w| w.wgm3().bits(0b01));
            tim.tccr3b.modify(|_r, w| {
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
            PB6: {
                ocr: ocr3a,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_r, w| w.com3a().match_clear());
                } else {
                    tim.tccr3a.modify(|_r, w| w.com3a().disconnected());
                },
            },

            PB7: {
                ocr: ocr3b,
                into_pwm: |tim| if enable {
                    tim.tccr3a.modify(|_r, w| w.com3b().match_clear());
                } else {
                    tim.tccr3a.modify(|_r, w| w.com3b().disconnected());
                },
            },
        },
    }
}
