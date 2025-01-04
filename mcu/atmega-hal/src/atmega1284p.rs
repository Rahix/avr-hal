pub use avr_device::atmega1284p as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::atmega1284p as hal;
    impl_adc_channels_extra!();
    impl_adc!();

    avr_hal_generic::impl_adc! {
        hal: hal::Hal,
        peripheral: hal::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
        channel_id: hal::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            hal::port::PA0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            hal::port::PA1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            hal::port::PA2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            hal::port::PA3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            hal::port::PA4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            hal::port::PA5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
        },
        channels: {
            #[cfg(feature = "enable-extra-adc")]
            channel::ADC6: hal::pac::adc::admux::MUX_A::ADC6,
            #[cfg(feature = "enable-extra-adc")]
            channel::ADC7: hal::pac::adc::admux::MUX_A::ADC7,
            channel::Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::atmega1284p,
    capacity: 4096,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega,
}

impl_mod_i2c! {
    use crate::atmega1284p as hal;
    impl_i2c_peripheral! {
        i2c_type: I2c,
        peripheral: hal::pac::TWI,
        sda: hal::port::PC1,
        scl: hal::port::PC0,
    }
}

impl_mod_port! {
    use crate::atmega1284p as hal;
    impl_port_peripheral_a8_b8_c8_d8! {
    }

    #[macro_export]
    macro_rules! atmega1284_pins {
        ($p:expr) => {
            $crate::atmega1284::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
        };
    }

    pub use atmega1284_pins as pins;
}

impl_mod_simple_pwm! {
    use crate::atmega1284p as hal;

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
}

impl_mod_spi! {
    use crate::atmega1284p as hal;
    impl_spi_peripheral! {
        spi: Spi,
        peripheral: hal::pac::SPI,
        sclk: hal::port::PB7,
        mosi: hal::port::PB5,
        miso: hal::port::PB6,
        cs: hal::port::PB4,
    }
}

impl_mod_usart! {
    use crate::atmega1284p as hal;
    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART0,
        register_suffix: 0,
        rx: hal::port::PD0,
        tx: hal::port::PD1,
        usart_type: Usart0,
    }

    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART1,
        register_suffix: 1,
        rx: hal::port::PD2,
        tx: hal::port::PD3,
        usart_type: Usart1,
    }
}

impl_mod_wdt! {
    use crate::atmega1284p as hal;
    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}
