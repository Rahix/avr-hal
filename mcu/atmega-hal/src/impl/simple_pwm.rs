#![allow(unused_macros)]

macro_rules! impl_mod_simple_pwm {
    (
        hal: crate::$hal:ident,
        timers: {
            $(
                $timer:ident: {
                    peripheral: $peripheral:ident,
                    impl!: $($impl_macro:ident)::* $({
                        $($arg:tt)*
                    })?,
                },
            )*
        },
    ) => {
        pub mod simple_pwm {
            pub use avr_hal_generic::simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps};
            use avr_hal_generic::paste::paste;

            #[allow(unused_imports)]
            use crate::r#impl::{
                timer0_8bit_impl,
                timer_10bit_impl,
                timer_16bit_impl,
                timer_8bit_1wf_with_async,
                timer_8bit_2wf_with_async,
            };

            $(
                $($impl_macro)::+! {
                    hal: crate::$hal,
                    timer: $timer,
                    peripheral: $peripheral,
                    $($($arg)*)?
                }
            )*
        }
    }
}
pub(crate) use impl_mod_simple_pwm;

#[allow(unused_macros)]
macro_rules! timer0_8bit_impl {
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
                #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
                #[doc = concat!("use hal::simple_pwm::{IntoPwmPin,", stringify!($timer), ",Prescaler};")]
                ///
                /// let dp = hal::Peripherals::take().unwrap();
                /// let pins = hal::pins!(dp);
                #[doc = concat!("let mut timer = ", stringify!($timer), "::new(dp.", stringify!($peripheral), ", Prescaler::Prescale64);")]
                ///
                $(
                    #[doc = paste!{ concat!(
                        "let mut ", stringify!([< $pin:lower >]), " = pins.", stringify!([< $pin:lower >]), ".into_output().into_pwm(&mut timer);\n",
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
                            crate::$hal::port::$pin: {
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
pub(crate) use timer0_8bit_impl;

#[allow(unused_macros)]
macro_rules! timer1_8bit_separate_prescale {
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
                #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
                #[doc = concat!("use hal::simple_pwm::{IntoPwmPin,", stringify!($timer), ",Prescaler};")]
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
                            crate::$hal::port::$pin: {
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
pub(crate) use timer1_8bit_separate_prescale;

#[allow(unused_macros)]
macro_rules! timer_10bit_impl {
    (
        hal: crate::$hal:ident,
        timer: $timer:ident,
        peripheral: $peripheral:ident,
        tccr: $tccr:ident,
        pwm: $pwm:ident,
        cs: $cs:ident,
        pins: {
            $(
                $pin:ident: {
                    pin_tccr: $pin_tccr:ident,
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
                #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
                #[doc = concat!("use hal::simple_pwm::{IntoPwmPin,", stringify!($timer), ",Prescaler};")]
                ///
                /// let dp = hal::Peripherals::take().unwrap();
                /// let pins = hal::pins!(dp);
                #[doc = concat!("let mut timer = ", stringify!($timer), "::new(dp.", stringify!($peripheral), ", Prescaler::Prescale64);")]
                ///
                $(
                    #[doc = paste!{ concat!(
                        "let mut ", stringify!([< $pin:lower >]), " = pins.", stringify!([< $pin:lower >]), ".into_output().into_pwm(&mut timer);\n",
                        stringify!([< $pin:lower >]), ".set_duty(128);\n",
                        stringify!([< $pin:lower >]), ".enable();\n",
                        "\n",
                    ) }]
                )+
                /// ```
                pub struct $timer {
                    timer: crate::$hal::pac::$peripheral,
                    init: |tim, prescaler| {
                        tim.[<$tccr a>].modify(|_r, w| w.[<$pwm a>]().set_bit());
                        tim.[<$tccr a>].modify(|_r, w| w.[<$pwm b>]().set_bit());
                        tim.[<$tccr c>].modify(|_r, w| w.[<$pwm d>]().set_bit());
                        tim.[<$tccr b>].modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.$cs().direct(),
                            Prescaler::Prescale8 => w.$cs().prescale_8(),
                            Prescaler::Prescale64 => w.$cs().prescale_64(),
                            Prescaler::Prescale256 => w.$cs().prescale_256(),
                            Prescaler::Prescale1024 => w.$cs().prescale_1024(),
                        });
                    },
                    pins: {
                        $(
                            crate::$hal::port::$pin: {
                                ocr: $ocr,
                                into_pwm: |tim| if enable {
                                    tim.$pin_tccr.modify(|_r, w| w.$com().match_clear());
                                } else {
                                    tim.$pin_tccr.modify(|_r, w| w.$com().disconnected());
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
pub(crate) use timer_10bit_impl;

#[allow(unused_macros)]
macro_rules! timer_16bit_impl {
    (
        hal: crate::$hal:ident,
        timer: $timer:ident,
        peripheral: $peripheral:ident,
        tccr: $tccr:ident,
        wgm: $wgm:ident,
        $(tccr_b_wgm: $tccr_b_wgm:ident,)?
        cs: $cs:ident,
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
                #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
                #[doc = concat!("use hal::simple_pwm::{IntoPwmPin,", stringify!($timer), ",Prescaler};")]
                ///
                /// let dp = hal::Peripherals::take().unwrap();
                /// let pins = hal::pins!(dp);
                #[doc = concat!("let mut timer = ", stringify!($timer), "::new(dp.", stringify!($peripheral), ", Prescaler::Prescale64);")]
                ///
                $(
                    #[doc = paste!{ concat!(
                        "let mut ", stringify!([< $pin:lower >]), " = pins.", stringify!([< $pin:lower >]), ".into_output().into_pwm(&mut timer);\n",
                        stringify!([< $pin:lower >]), ".set_duty(128);\n",
                        stringify!([< $pin:lower >]), ".enable();\n",
                        "\n",
                    ) }]
                )+
                /// ```
                pub struct $timer {
                    timer: crate::$hal::pac::$peripheral,
                    init: |tim, prescaler| {
                        tim.[<$tccr a>].modify(|_, w| w.$wgm().bits(0b01));
                        tim.[<$tccr b>].modify(|_, w| $($tccr_b_wgm)? { w.$wgm().bits(0b01) });

                        tim.[<$tccr b>].modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.$cs().direct(),
                            Prescaler::Prescale8 => w.$cs().prescale_8(),
                            Prescaler::Prescale64 => w.$cs().prescale_64(),
                            Prescaler::Prescale256 => w.$cs().prescale_256(),
                            Prescaler::Prescale1024 => w.$cs().prescale_1024(),
                        });
                    },
                    pins: {
                        $(
                            crate::$hal::port::$pin: {
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
pub(crate) use timer_16bit_impl;

#[allow(unused_macros)]
macro_rules! timer_8bit_1wf_with_async {
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
                #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
                #[doc = concat!("use hal::simple_pwm::{IntoPwmPin,", stringify!($timer), ",Prescaler};")]
                ///
                /// let dp = hal::Peripherals::take().unwrap();
                /// let pins = hal::pins!(dp);
                #[doc = concat!("let mut timer = ", stringify!($timer), "::new(dp.", stringify!($peripheral), ", Prescaler::Prescale64);")]
                ///
                $(
                    #[doc = paste!{ concat!(
                        "let mut ", stringify!([< $pin:lower >]), " = pins.", stringify!([< $pin:lower >]), ".into_output().into_pwm(&mut timer);\n",
                        stringify!([< $pin:lower >]), ".set_duty(128);\n",
                        stringify!([< $pin:lower >]), ".enable();\n",
                        "\n",
                    ) }]
                )+
                /// ```
                pub struct $timer {
                    timer: crate::$hal::pac::$peripheral,
                    init: |tim, prescaler| {
                        tim.tccr2.modify(|_r, w| w.wgm20().set_bit().wgm21().set_bit());
                        tim.tccr2.modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs2().direct(),
                            Prescaler::Prescale8 => w.cs2().prescale_8(),
                            Prescaler::Prescale64 => w.cs2().prescale_64(),
                            Prescaler::Prescale256 => w.cs2().prescale_256(),
                            Prescaler::Prescale1024 => w.cs2().prescale_1024(),
                        });
                    },
                    pins: {
                        $(
                            crate::$hal::port::$pin: {
                                ocr: $ocr,
                                into_pwm: |tim| if enable {
                                    tim.tccr2.modify(|_r, w| w.$com().match_clear());
                                } else {
                                    tim.tccr2.modify(|_r, w| w.$com().disconnected());
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
pub(crate) use timer_8bit_1wf_with_async;

#[allow(unused_macros)]
macro_rules! timer_8bit_2wf_with_async {
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
                #[doc = concat!("use atmega_hal::", stringify!($hal), " as hal;")]
                #[doc = concat!("use hal::simple_pwm::{IntoPwmPin,", stringify!($timer), ",Prescaler};")]
                ///
                /// let dp = hal::Peripherals::take().unwrap();
                /// let pins = hal::pins!(dp);
                #[doc = concat!("let mut timer = ", stringify!($timer), "::new(dp.", stringify!($peripheral), ", Prescaler::Prescale64);")]
                ///
                $(
                    #[doc = paste!{ concat!(
                        "let mut ", stringify!([< $pin:lower >]), " = pins.", stringify!([< $pin:lower >]), ".into_output().into_pwm(&mut timer);\n",
                        stringify!([< $pin:lower >]), ".set_duty(128);\n",
                        stringify!([< $pin:lower >]), ".enable();\n",
                        "\n",
                    ) }]
                )+
                /// ```
                pub struct $timer {
                    timer: crate::$hal::pac::$peripheral,
                    init: |tim, prescaler| {
                        tim.[<$tccr a>].modify(|_r, w| w.wgm2().pwm_fast());
                        tim.[<$tccr b>].modify(|_r, w| w.wgm22().clear_bit());
                        tim.[<$tccr b>].modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs2().direct(),
                            Prescaler::Prescale8 => w.cs2().prescale_8(),
                            Prescaler::Prescale64 => w.cs2().prescale_64(),
                            Prescaler::Prescale256 => w.cs2().prescale_256(),
                            Prescaler::Prescale1024 => w.cs2().prescale_1024(),
                        });
                    },
                    pins: {
                        $(
                            crate::$hal::port::$pin: {
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
pub(crate) use timer_8bit_2wf_with_async;
