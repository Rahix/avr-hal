macro_rules! impl_mod_simple_pwm {
    (
        hal: crate::$hal:ident,
        timers: {
            $(
                $timer:ident: {
                    peripheral: $peripheral:ident,
                    tccr: $tccr:ident,
                    pins: {
                        $(
                            $pin:ident: {
                                ocr: $ocr:ident,
                                com: $com:ident,
                            },
                        )*
                    },
                    impl!: $($impl_macro:ident)::+,
                },
            )*
        },
    ) => {
        pub mod simple_pwm {
            use crate::$hal as hal;
            pub use avr_hal_generic::simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps};
            use avr_hal_generic::paste::paste;

            $(
                $($impl_macro)::+! {
                    hal: crate::$hal,
                    timer: $timer,
                    peripheral: $peripheral,
                    tccr: $tccr,
                    pins: {
                        $(
                            $pin: {
                                ocr: $ocr,
                                com: $com,
                            },
                        )*
                    },
                }
            )*
        }
    }
}
pub(crate) use impl_mod_simple_pwm;

#[allow(unused_macros)]
macro_rules! timer_8bit_impl {
    (
        hal: crate::$hal:ident,
        timer: $timer:ident,
        peripheral: $peripheral:ident,
        tccr: $tccr:ident,
        pins: {
            $(
                $pin:ident: {
                    ocr: $ocr:ident,
                    com: $com:ident,
                },
            )*
        },
    ) => {
        paste! {
            avr_hal_generic::impl_simple_pwm! {
                #[doc = concat!("Use `", stringify!($peripheral), "` for PWM.")]
                ///
                /// # Example
                /// ```no_run
                #[doc = concat!("use attiny_hal::", stringify!($hal), " as hal;")]
                /// use hal::simple_pwm::{IntoPwmPin,Timer0Pwm,Prescaler};
                ///
                /// let dp = hal::Peripherals::take().unwrap();
                /// let pins = hal::pins!(dp);
                #[doc = concat!("let mut timer0 = ", stringify!($timer), "::new(dp.", stringify!($peripheral), ", Prescaler::Prescale64);")]
                ///
                $(
                    #[doc = paste!{ concat!(
                        "let mut ", stringify!([< $pin:lower >]), " = pins.", stringify!([< $pin:lower >]), ".into_output().into_pwm(&mut timer0);\n",
                        stringify!([< $pin:lower >]), ".set_duty(128);\n",
                        stringify!([< $pin:lower >]), ".enable();\n",
                        "\n",
                    ) }]
                )+
                /// ```
                pub struct $timer {
                    timer: crate::$hal::pac::$peripheral,
                    init: |tim, prescaler| {
                        tim.[<$tccr a>].modify(|_r, w| w.wgm0().pwm_fast());
                        tim.[<$tccr b>].modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs0().direct(),
                            Prescaler::Prescale8 => w.cs0().prescale_8(),
                            Prescaler::Prescale64 => w.cs0().prescale_64(),
                            Prescaler::Prescale256 => w.cs0().prescale_256(),
                            Prescaler::Prescale1024 => w.cs0().prescale_1024(),
                        });
                    },
                    pins: {
                        $(
                            hal::port::$pin: {
                                ocr: $ocr,
                                into_pwm: |tim| if enable {
                                    tim.[<$tccr a>].modify(|_r, w| w.$com().match_clear());
                                } else {
                                    tim.[<$tccr a>].modify(|_r, w| w.$com().disconnected());
                                },
                            },
                        )*
                    },
                }
            }
        }
    }
}
#[allow(unused_imports)]
pub(crate) use timer_8bit_impl;

#[allow(unused_macros)]
macro_rules! timer_8bit_separate_prescale {
    (
        hal: crate::$hal:ident,
        timer: $timer:ident,
        peripheral: $peripheral:ident,
        tccr: $tccr:ident,
        pins: {
            $(
                $pin:ident: {
                    ocr: $ocr:ident,
                    com: $com:ident,
                },
            )*
        },
    ) => {
        paste! {
            avr_hal_generic::impl_simple_pwm! {
                #[doc = concat!("Use `", stringify!($peripheral), "` for PWM.")]
                ///
                /// # Example
                /// ```no_run
                #[doc = concat!("use attiny_hal::", stringify!($hal), " as hal;")]
                /// use hal::simple_pwm::{IntoPwmPin,Timer1Pwm,Prescaler};
                ///
                /// let dp = hal::Peripherals::take().unwrap();
                /// let pins = hal::pins!(dp);
                #[doc = concat!("let mut timer1 = ", stringify!($timer), "::new(dp.", stringify!($peripheral), ", Prescaler::Prescale64);")]
                ///
                $(
                    #[doc = paste!{ concat!(
                        "let mut ", stringify!([< $pin:lower >]), " = pins.", stringify!([< $pin:lower >]), ".into_output().into_pwm(&mut timer1);\n",
                        stringify!([< $pin:lower >]), ".set_duty(128);\n",
                        stringify!([< $pin:lower >]), ".enable();\n",
                        "\n",
                    ) }]
                )+
                /// ```
                pub struct $timer {
                    timer: crate::$hal::pac::$peripheral,
                    init: |tim, prescaler| {
                        tim.gtccr.modify(|_, w| w.pwm1b().bit(true));

                        tim.$tccr.modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs1().direct(),
                            Prescaler::Prescale8 => w.cs1().prescale_8(),
                            Prescaler::Prescale64 => w.cs1().prescale_64(),
                            Prescaler::Prescale256 => w.cs1().prescale_256(),
                            Prescaler::Prescale1024 => w.cs1().prescale_1024(),
                        });
                    },
                    pins: {
                        $(
                            hal::port::$pin: {
                                ocr: $ocr,
                                into_pwm: |tim| if enable {
                                    tim.gtccr.modify(|_r, w| w.$com().bits(0b10));
                                } else {
                                    tim.gtccr.modify(|_r, w| w.$com().disconnected());
                                },
                            },
                        )*
                    },
                }
            }
        }
    }
}
#[allow(unused_imports)]
pub(crate) use timer_8bit_separate_prescale;

#[allow(unused_macros)]
macro_rules! timer_16bit_impl {
    (
        hal: crate::$hal:ident,
        timer: $timer:ident,
        peripheral: $peripheral:ident,
        tccr: $tccr:ident,
        pins: {
            $(
                $pin:ident: {
                    ocr: $ocr:ident,
                    com: $com:ident,
                },
            )*
        },
    ) => {
        paste! {
            avr_hal_generic::impl_simple_pwm! {
                #[doc = concat!("Use `", stringify!($peripheral), "` for PWM.")]
                ///
                /// # Example
                /// ```no_run
                #[doc = concat!("use attiny_hal::", stringify!($hal), " as hal;")]
                /// use hal::simple_pwm::{IntoPwmPin,Timer1Pwm,Prescaler};
                ///
                /// let dp = hal::Peripherals::take().unwrap();
                /// let pins = hal::pins!(dp);
                #[doc = concat!("let mut timer1 = ", stringify!($timer), "::new(dp.", stringify!($peripheral), ", Prescaler::Prescale64);")]
                ///
                $(
                    #[doc = paste!{ concat!(
                        "let mut ", stringify!([< $pin:lower >]), " = pins.", stringify!([< $pin:lower >]), ".into_output().into_pwm(&mut timer1);\n",
                        stringify!([< $pin:lower >]), ".set_duty(128);\n",
                        stringify!([< $pin:lower >]), ".enable();\n",
                        "\n",
                    ) }]
                )+
                /// ```
                pub struct $timer {
                    timer: crate::$hal::pac::$peripheral,
                    init: |tim, prescaler| {
                        tim.[<$tccr a>].modify(|_, w| w.wgm1().bits(0b01));
                        tim.[<$tccr b>].modify(|_, w| w.wgm1().bits(0b01));

                        tim.[<$tccr b>].modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs1().direct(),
                            Prescaler::Prescale8 => w.cs1().prescale_8(),
                            Prescaler::Prescale64 => w.cs1().prescale_64(),
                            Prescaler::Prescale256 => w.cs1().prescale_256(),
                            Prescaler::Prescale1024 => w.cs1().prescale_1024(),
                        });
                    },
                    pins: {
                        $(
                            hal::port::$pin: {
                                ocr: $ocr,
                                into_pwm: |tim| if enable {
                                    tim.[<$tccr a>].modify(|_r, w| w.$com().bits(0b10));
                                } else {
                                    tim.[<$tccr a>].modify(|_r, w| w.$com().disconnected());
                                },
                            },
                        )*
                    },
                }
            }
        }
    }
}
#[allow(unused_imports)]
pub(crate) use timer_16bit_impl;
