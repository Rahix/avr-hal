use crate::r#impl::avr_hal;

avr_hal! {
    device: attiny85,
    eeprom: {
        capacity: 512,
        addr_width: u16,
        addr_reg: eear,
    },
    port: {
        ports: {
            B: [0, 1, 2, 3, 4, 5],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },
    pwm: {
        impl: {
            pub use avr_hal_generic::simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps};
            avr_hal_generic::impl_simple_pwm! {
                /// Use `TC0` for PWM (pins `PB0`, `PB1`)
                ///
                /// # Example
                /// ```
                /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
                ///
                /// let mut d0 = pins.d0.into_output().into_pwm(&mut timer0);
                /// let mut d1 = pins.d1.into_output().into_pwm(&mut timer0);
                ///
                /// d0.set_duty(128);
                /// d0.enable();
                /// ```
                pub struct Timer0Pwm {
                    timer: crate::attiny85::pac::TC0,
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
                        hal::port::PB0: {
                            ocr: ocr0a,
                            into_pwm: |tim| if enable {
                                tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                            } else {
                                tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                            },
                        },

                        hal::port::PB1: {
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
                /// Use `TC1` for PWM (pins `PB4`)
                ///
                /// # Example
                /// ```
                /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
                ///
                /// let mut d4 = pins.d4.into_output().into_pwm(&mut timer1);
                ///
                /// d4.set_duty(128);
                /// d4.enable();
                /// ```
                pub struct Timer1Pwm {
                    timer: crate::attiny85::pac::TC1,
                    init: |tim, prescaler| {
                        tim.gtccr.modify(|_, w| w.pwm1b().bit(true));

                        tim.tccr1.modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs1().direct(),
                            Prescaler::Prescale8 => w.cs1().prescale_8(),
                            Prescaler::Prescale64 => w.cs1().prescale_64(),
                            Prescaler::Prescale256 => w.cs1().prescale_256(),
                            Prescaler::Prescale1024 => w.cs1().prescale_1024(),
                        });
                    },
                    pins: {
                        hal::port::PB4: {
                            ocr: ocr1b,
                            into_pwm: |tim| if enable {
                                tim.gtccr.modify(|_, w| w.com1b().bits(0b10));
                            } else {
                                tim.gtccr.modify(|_, w| w.com1b().disconnected());
                            },
                        },
                    },
                }
            }
        },
    },
    adc: {
        references: {
            /// Voltage applied to AREF pin.
            Aref: |peripheral| {
                peripheral.admux.write(|w| w.refs().aref())
            },
            /// Default reference voltage (default).
            AVcc: |peripheral| {
                peripheral.admux.write(|w| w.refs().vcc())
            },
            /// Internal 1.1V reference.
            Internal1_1: |peripheral| {
                peripheral.admux.write(|w| w.refs().internal().refs2().clear_bit())
            },
            /// Internal 2.56V reference.
            Internal2_56: |peripheral| {
                peripheral.admux.write(|w| w.refs().internal().refs2().set_bit())
            },
        },
        pins: {
            PB5: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            PB2: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            PB4: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            PB3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        },
        channels: {
            Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
            Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
        },
    },
    wdt: {
        wdtcsr_name: wdtcr,
    },
}
