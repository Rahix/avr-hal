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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer0Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
            ///
            /// let mut pd5 = pins.pd5.into_output().into_pwm(&mut timer0);
            /// let mut pd6 = pins.pd6.into_output().into_pwm(&mut timer0);
            ///
            /// pd5.set_duty(128);
            /// pd5.enable();
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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer1Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
            ///
            /// let mut pb1 = pins.pb1.into_output().into_pwm(&mut timer1);
            /// let mut pb2 = pins.pb2.into_output().into_pwm(&mut timer1);
            ///
            /// pb1.set_duty(128);
            /// pb1.enable();
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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer2Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
            ///
            /// let mut pb3 = pins.pb3.into_output().into_pwm(&mut timer2);
            /// let mut pd3 = pins.pd3.into_output().into_pwm(&mut timer2);
            ///
            /// pb3.set_duty(128);
            /// pb3.enable();
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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer0Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
            ///
            /// let mut pb7 = pins.pb7.into_output().into_pwm(&mut timer0);
            /// let mut pg5 = pins.pg5.into_output().into_pwm(&mut timer0);
            ///
            /// pb7.set_duty(128);
            /// pb7.enable();
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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer1Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
            ///
            /// let mut pb5 = pins.pb5.into_output().into_pwm(&mut timer1);
            /// let mut pb6 = pins.pb6.into_output().into_pwm(&mut timer1);
            /// let mut pb7 = pins.pb7.into_output().into_pwm(&mut timer1);
            ///
            /// pb5.set_duty(128);
            /// pb5.enable();
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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer2Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
            ///
            /// let mut pb4 = pins.pb4.into_output().into_pwm(&mut timer2);
            /// let mut ph6 = pins.ph6.into_output().into_pwm(&mut timer2);
            ///
            /// pb4.set_duty(128);
            /// pb4.enable();
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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer3Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);
            ///
            /// let mut pe3 = pins.pe3.into_output().into_pwm(&mut timer3);
            /// let mut pe4 = pins.pe4.into_output().into_pwm(&mut timer3);
            /// let mut pe5 = pins.pe5.into_output().into_pwm(&mut timer3);
            ///
            /// pe3.set_duty(128);
            /// pe3.enable();
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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer4Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer4 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);
            ///
            /// let mut ph3 = pins.ph3.into_output().into_pwm(&mut timer4);
            /// let mut ph4 = pins.ph4.into_output().into_pwm(&mut timer4);
            /// let mut ph5 = pins.ph5.into_output().into_pwm(&mut timer4);
            ///
            /// ph3.set_duty(128);
            /// ph3.enable();
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
            /// ```no_run
            #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
            /// use hal::simple_pwm::{IntoPwmPin,Timer5Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer5 = Timer5Pwm::new(dp.TC5, Prescaler::Prescale64);
            ///
            /// let mut pl3 = pins.pl3.into_output().into_pwm(&mut timer5);
            /// let mut pl4 = pins.pl4.into_output().into_pwm(&mut timer5);
            /// let mut pl5 = pins.pl5.into_output().into_pwm(&mut timer5);
            ///
            /// pl3.set_duty(128);
            /// pl3.enable();
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
