pub use avr_device::atmega32u4 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega32u4,
    pins: {
        PF0: (0b000000, didr0::adc0d),
        PF1: (0b000001, didr0::adc1d),
        PF4: (0b000100, didr0::adc4d),
        PF5: (0b000101, didr0::adc5d),
        PF6: (0b000110, didr0::adc6d),
        PF7: (0b000111, didr0::adc7d),
        PD4: (0b100000, didr2::adc8d),
        PD6: (0b100001, didr2::adc9d),
        PD7: (0b100010, didr2::adc10d),
        PB4: (0b100011, didr2::adc11d),
        PB5: (0b100100, didr2::adc12d),
        PB6: (0b100101, didr2::adc13d),
    },
    channels: {
        Vbg: 0b011110,
        Gnd: 0b011111,
        Temperature: 0b100111,
    },
    impl!: impl_adc_admux_adcsrb,
}

impl_mod_eeprom! {
    hal: crate::atmega32u4,
    capacity: 1024,
    addr_width: u16,
    addr_reg: eear,
    impl!: avr_hal_generic::impl_eeprom_atmega,
}

impl_mod_i2c! {
    hal: crate::atmega32u4,
    interfaces: {
        I2c: {
            peripheral: TWI,
            sda: PD1,
            scl: PD0,
        },
    },
}

impl_mod_port! {
    use crate::atmega32u4 as hal;

    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            C: hal::pac::PORTC = [6, 7],
            D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
            E: hal::pac::PORTE = [2, 6],
            F: hal::pac::PORTF = [0, 1, 4, 5, 6, 7],
        }
    }

    #[macro_export]
    macro_rules! atmega32u4_pins {
        ($p:expr) => {
            $crate::atmega32u4::Pins::new($p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF)
        };
    }

    pub use atmega32u4_pins as pins;
}

impl_mod_simple_pwm! {
    hal: crate::atmega32u4,
    impl: {
        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC0` for PWM (pins `PB7`, `PD0`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega32u4 as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer0Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
            ///
            /// let mut pb7 = pins.pb7.into_output().into_pwm(&mut timer0);
            /// let mut pd0 = pins.pd0.into_output().into_pwm(&mut timer0);
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

                    hal::port::PD0: {
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
            /// use atmega_hal::atmega32u4 as hal;
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
            /// Use `TC3` for PWM (pins `PC6`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega32u4 as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer3Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);
            ///
            /// let mut pc6 = pins.pc6.into_output().into_pwm(&mut timer3);
            ///
            /// pc6.set_duty(128);
            /// pc6.enable();
            /// ```
            pub struct Timer3Pwm {
                timer: hal::pac::TC3,
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
                    hal::port::PC6: {
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

        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC4` for PWM (pins `PB6`, `PC7`, `PD7`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega32u4 as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer4Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer4 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);
            ///
            /// let mut pb6 = pins.pb6.into_output().into_pwm(&mut timer4);
            /// let mut pc7 = pins.pc7.into_output().into_pwm(&mut timer4);
            /// let mut pd7 = pins.pd7.into_output().into_pwm(&mut timer4);
            ///
            /// pb6.set_duty(128);
            /// pb6.enable();
            /// ```
            pub struct Timer4Pwm {
                timer: hal::pac::TC4,
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
                    hal::port::PB6: {
                        ocr: ocr4b,
                        into_pwm: |tim| if enable {
                            tim.tccr4a.modify(|_r, w| w.com4b().match_clear());
                        } else {
                            tim.tccr4a.modify(|_r, w| w.com4b().disconnected());
                        },
                    },

                    hal::port::PC7: {
                        ocr: ocr4a,
                        into_pwm: |tim| if enable {
                            tim.tccr4a.modify(|_r, w| w.com4a().match_clear());
                        } else {
                            tim.tccr4a.modify(|_r, w| w.com4a().disconnected());
                        },
                    },

                    hal::port::PD7: {
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
    },
}

impl_mod_spi! {
    hal: crate::atmega32u4,
    interfaces: {
        Spi: {
            peripheral: SPI,
            sclk: PB1,
            mosi: PB2,
            miso: PB3,
            cs: PB0,
        },
    },
}

impl_mod_usart! {
    hal: crate::atmega32u4,
    interfaces: {
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
    use crate::atmega32u4 as hal;

    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}

