#![allow(unused_macros)]

macro_rules! impl_mod_simple_pwm {
    (
        hal: crate::$hal:ident,
        $(impl!: $($impl_macro:ident)::* $({
            $($arg_name:ident: $arg_value:expr,)*
        })?,)?
        $(impl: {
            $($impl:item)*
        },)?
    ) => {
        pub mod simple_pwm {
            pub use avr_hal_generic::simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps};

            #[allow(unused_imports)]
            use crate::r#impl::{impl_simple_pwm_48p_168_328p_328pb, impl_simple_pwm_1280_2560};

            #[allow(unused_imports)]
            use crate::$hal as hal;

            $(
                $($impl_macro)::+! {
                    hal: crate::$hal,
                    $($($arg_name: $arg_value,)*)?
                }
            )?

            $($($impl)*)?
        }
    }
}
pub(crate) use impl_mod_simple_pwm;

macro_rules! impl_simple_pwm_48p_168_328p_328pb {
    (
        hal: crate::$hal:ident,
    ) => {
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
                timer: hal::pac::TC0,
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
                    hal::port::PD6: {
                        ocr: ocr0a,
                        into_pwm: |tim| if enable {
                            tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                        } else {
                            tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                        },
                    },

                    hal::port::PD5: {
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
                timer: hal::pac::TC1,
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
                    hal::port::PB1: {
                        ocr: ocr1a,
                        into_pwm: |tim| if enable {
                            tim.tccr1a.modify(|_r, w| w.com1a().match_clear());
                        } else {
                            tim.tccr1a.modify(|_r, w| w.com1a().disconnected());
                        },
                    },

                    hal::port::PB2: {
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
                timer: hal::pac::TC2,
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
                    hal::port::PB3: {
                        ocr: ocr2a,
                        into_pwm: |tim| if enable {
                            tim.tccr2a.modify(|_r, w| w.com2a().match_clear());
                        } else {
                            tim.tccr2a.modify(|_r, w| w.com2a().disconnected());
                        },
                    },

                    hal::port::PD3: {
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
    };
}
pub(crate) use impl_simple_pwm_48p_168_328p_328pb;

macro_rules! impl_simple_pwm_1280_2560 {
    (
        hal: crate::$hal:ident,
    ) => {
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
                timer: hal::pac::TC0,
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
                    hal::port::PB7: {
                        ocr: ocr0a,
                        into_pwm: |tim| if enable {
                            tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                        } else {
                            tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                        },
                    },

                    hal::port::PG5: {
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
                timer: hal::pac::TC1,
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
                    hal::port::PB5: {
                        ocr: ocr1a,
                        into_pwm: |tim| if enable {
                            tim.tccr1a.modify(|_r, w| w.com1a().match_clear());
                        } else {
                            tim.tccr1a.modify(|_r, w| w.com1a().disconnected());
                        },
                    },

                    hal::port::PB6: {
                        ocr: ocr1b,
                        into_pwm: |tim| if enable {
                            tim.tccr1a.modify(|_r, w| w.com1b().match_clear());
                        } else {
                            tim.tccr1a.modify(|_r, w| w.com1b().disconnected());
                        },
                    },

                    hal::port::PB7: {
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
                timer: hal::pac::TC2,
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
                    hal::port::PB4: {
                        ocr: ocr2a,
                        into_pwm: |tim| if enable {
                            tim.tccr2a.modify(|_r, w| w.com2a().match_clear());
                        } else {
                            tim.tccr2a.modify(|_r, w| w.com2a().disconnected());
                        },
                    },

                    hal::port::PH6: {
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
                timer: hal::pac::TC3,
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
                    hal::port::PE3: {
                        ocr: ocr3a,
                        into_pwm: |tim| if enable {
                            tim.tccr3a.modify(|_r, w| w.com3a().match_clear());
                        } else {
                            tim.tccr3a.modify(|_r, w| w.com3a().disconnected());
                        },
                    },

                    hal::port::PE4: {
                        ocr: ocr3b,
                        into_pwm: |tim| if enable {
                            tim.tccr3a.modify(|_r, w| w.com3b().match_clear());
                        } else {
                            tim.tccr3a.modify(|_r, w| w.com3b().disconnected());
                        },
                    },

                    hal::port::PE5: {
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
                timer: hal::pac::TC4,
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
                    hal::port::PH3: {
                        ocr: ocr4a,
                        into_pwm: |tim| if enable {
                            tim.tccr4a.modify(|_r, w| w.com4a().match_clear());
                        } else {
                            tim.tccr4a.modify(|_r, w| w.com4a().disconnected());
                        },
                    },

                    hal::port::PH4: {
                        ocr: ocr4b,
                        into_pwm: |tim| if enable {
                            tim.tccr4a.modify(|_r, w| w.com4b().match_clear());
                        } else {
                            tim.tccr4a.modify(|_r, w| w.com4b().disconnected());
                        },
                    },

                    hal::port::PH5: {
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
                timer: hal::pac::TC5,
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
                    hal::port::PL3: {
                        ocr: ocr5a,
                        into_pwm: |tim| if enable {
                            tim.tccr5a.modify(|_r, w| w.com5a().match_clear());
                        } else {
                            tim.tccr5a.modify(|_r, w| w.com5a().disconnected());
                        },
                    },

                    hal::port::PL4: {
                        ocr: ocr5b,
                        into_pwm: |tim| if enable {
                            tim.tccr5a.modify(|_r, w| w.com5b().match_clear());
                        } else {
                            tim.tccr5a.modify(|_r, w| w.com5b().disconnected());
                        },
                    },

                    hal::port::PL5: {
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

    }
}
pub(crate) use impl_simple_pwm_1280_2560;
