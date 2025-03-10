use crate::r#impl::avr_hal;

avr_hal! {
    device: atmega164pa,

    eeprom: {
        capacity: 512,
        addr_width: u16,
        addr_reg: eear,
        impl!: avr_hal_generic::impl_eeprom_atmega,
    },

    port: {
        ports: {
            A: [0, 1, 2, 3, 4, 5, 6 ,7],
            B: [0, 1, 2, 3, 4, 5, 6 ,7],
            C: [0, 1, 2, 3, 4, 5, 6 ,7],
            D: [0, 1, 2, 3, 4, 5, 6 ,7],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },

    pwm: {
        timers: {
            Timer0Pwm: {
                peripheral: TC0,
                impl!: crate::atmega164pa::atmega164pa_timer_8bit_impl {
                    tccr: tccr0,
                    pins: {
                        PB3: {
                            ocr: ocr0a,
                            com: com0a,
                        },
                    },
                },
            },
            Timer1Pwm: {
                peripheral: TC1,
                impl!: crate::atmega164pa::atmega164pa_timer_16bit_impl {
                    tccr: tccr1,
                    pins: {
                        PD5: {
                            ocr: ocr1a,
                            com: com1a,
                        },
                        PD4: {
                            ocr: ocr1b,
                            com: com1b,
                        },
                    },
                },
            },
        },
    },

    i2c: {
        interfaces: {
            I2c: {
                peripheral: TWI,
                sda: PC1,
                scl: PC0,
            },
        },
    },

    usart: {
        interfaces: {
            Usart0: {
                peripheral: USART0,
                rx: PD0,
                tx: PD1,
                impl!: crate::r#impl::impl_usart_traditional {
                    register_suffix: 0,
                },
            },
            Usart1: {
                peripheral: USART1,
                rx: PD2,
                tx: PD3,
                impl!: crate::r#impl::impl_usart_traditional {
                    register_suffix: 1,
                },
            },
        },
    },

    adc: {
        pins: {
            PA0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            PA1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            PA2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            PA3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            PA4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            PA5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
            PA6: (hal::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            PA7: (hal::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        },
        channels: {
            Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        },
        impl!: impl_adc_admux,
    },

   wdt: {
        impl!: impl_wdt_peripheral_ms8000 {
            mcusr: hal::pac::cpu::MCUSR,
            wdtcsr_name: wdtcsr,
        },
    },
}

macro_rules! atmega164pa_timer_8bit_impl {
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
                        tim.[<$tccr a>].modify(|_r, w| w.wgm0().bits(0b11));
                        tim.[<$tccr a>].modify(|_r, w| w.com0a().bits(0b00));

                        tim.[<$tccr b>].modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs0().running_no_prescaling(),
                            Prescaler::Prescale8 => w.cs0().running_clk_8(),
                            Prescaler::Prescale64 => w.cs0().running_clk_64(),
                            Prescaler::Prescale256 => w.cs0().running_clk_256(),
                            Prescaler::Prescale1024 => w.cs0().running_clk_1024(),
                        });
                    },
                    pins: {
                        $(
                            crate::$hal::port::$pin: {
                                ocr: $ocr,
                                into_pwm: |tim| if enable {
                                    tim.[<$tccr a>].modify(|_r, w| w.$com().bits(0b11));
                                } else {
                                    tim.[<$tccr a>].modify(|_r, w| w.$com().bits(0b00));
                                },
                            },
                        )*
                    },
                }
            }
        }
    }
}
pub(crate) use atmega164pa_timer_8bit_impl;

macro_rules! atmega164pa_timer_16bit_impl {
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
                        tim.[<$tccr a>].modify(|_r, w| w.wgm1().bits(0b01));
                        tim.[<$tccr a>].modify(|_r, w| w.com1a().bits(0b00));
                        tim.[<$tccr a>].modify(|_r, w| w.com1b().bits(0b00));
                        tim.[<$tccr b>].modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs1().running_no_prescaling(),
                            Prescaler::Prescale8 => w.cs1().running_clk_8(),
                            Prescaler::Prescale64 => w.cs1().running_clk_64(),
                            Prescaler::Prescale256 => w.cs1().running_clk_256(),
                            Prescaler::Prescale1024 => w.cs1().running_clk_1024(),
                        });
                    },
                    pins: {
                        $(
                            crate::$hal::port::$pin: {
                                ocr: $ocr,
                                into_pwm: |tim| if enable {
                                    tim.[<$tccr a>].modify(|_r, w| w.$com().bits(0b11));
                                } else {
                                    tim.[<$tccr a>].modify(|_r, w| w.$com().bits(0b00));
                                },
                            },
                        )*
                    },
                }
            }
        }
    }
}
pub(crate) use atmega164pa_timer_16bit_impl;
