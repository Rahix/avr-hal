pub use avr_device::atmega1284p as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega1284p,
    pins: {
        PA0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        PA1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        PA2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        PA3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        PA4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        PA5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
    },
    channels: {
        #[cfg(feature = "enable-extra-adc")]
        ADC6: hal::pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        ADC7: hal::pac::adc::admux::MUX_A::ADC7,
        Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
        Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
    },
    impl!: impl_adc_admux,
}

impl_mod_eeprom! {
    hal: crate::atmega1284p,
    capacity: 4096,
    addr_width: u16,
    addr_reg: eear,
    impl!: avr_hal_generic::impl_eeprom_atmega,
}

impl_mod_i2c! {
    hal: crate::atmega1284p,
    interfaces: {
        I2c: {
            peripheral: TWI,
            sda: PC1,
            scl: PC0,
        },
    },
}

impl_mod_port! {
    use crate::atmega1284p as hal;
    impl_port_peripheral_a8_b8_c8_d8! {
    }

    #[macro_export]
    macro_rules! atmega1284_pins {
        ($p:expr) => {
            $crate::atmega1284p::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
        };
    }

    pub use atmega1284_pins as pins;
}

impl_mod_simple_pwm! {
    hal: crate::atmega1284p,
    impl: {
        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC0` for PWM (pins `PB3`, `PB4`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega1284p as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer0Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
            ///
            /// let mut pb3 = pins.pb3.into_output().into_pwm(&mut timer0);
            /// let mut pb4 = pins.pb4.into_output().into_pwm(&mut timer0);
            ///
            /// pb3.set_duty(128);
            /// pb4.enable();
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
                    hal::port::PB3: {
                        ocr: ocr0a,
                        into_pwm: |tim| if enable {
                            tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                        } else {
                            tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                        },
                    },
    
                    hal::port::PB4: {
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
            /// Use `TC1` for PWM (pins `PD5`, `PD4`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega1284p as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer1Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
            ///
            /// let mut pd5 = pins.pd5.into_output().into_pwm(&mut timer1);
            /// let mut pd4 = pins.pd4.into_output().into_pwm(&mut timer1);
            ///
            /// pd5.set_duty(128);
            /// pd5.enable();
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
                    hal::port::PD5: {
                        ocr: ocr1a,
                        into_pwm: |tim| if enable {
                            tim.tccr1a.modify(|_r, w| w.com1a().match_clear());
                        } else {
                            tim.tccr1a.modify(|_r, w| w.com1a().disconnected());
                        },
                    },
    
                    hal::port::PD4: {
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
            /// Use `TC2` for PWM (pins `PD7`, `PD6`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega1284p as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer2Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
            ///
            /// let mut pd7 = pins.pd7.into_output().into_pwm(&mut timer2);
            /// let mut pd6 = pins.pd6.into_output().into_pwm(&mut timer2);
            ///
            /// pd7.set_duty(128);
            /// pd7.enable();
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
                    hal::port::PD7: {
                        ocr: ocr2a,
                        into_pwm: |tim| if enable {
                            tim.tccr2a.modify(|_r, w| w.com2a().match_clear());
                        } else {
                            tim.tccr2a.modify(|_r, w| w.com2a().disconnected());
                        },
                    },
    
                    hal::port::PD6: {
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
            /// Use `TC3` for PWM (pins `PB6`, `PB7`)
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
                    hal::port::PB6: {
                        ocr: ocr3a,
                        into_pwm: |tim| if enable {
                            tim.tccr3a.modify(|_r, w| w.com3a().match_clear());
                        } else {
                            tim.tccr3a.modify(|_r, w| w.com3a().disconnected());
                        },
                    },
    
                    hal::port::PB7: {
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
    },
}

impl_mod_spi! {
    hal: crate::atmega1284p,
    interfaces: {
        Spi: {
            peripheral: SPI,
            sclk: PB7,
            mosi: PB5,
            miso: PB6,
            cs: PB4,
        },
    },
}

impl_mod_usart! {
    hal: crate::atmega1284p,
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
}

impl_mod_wdt! {
    use crate::atmega1284p as hal;
    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}
