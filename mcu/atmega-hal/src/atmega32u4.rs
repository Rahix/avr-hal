pub use avr_device::atmega32u4 as pac;

pub struct Hal;

use crate::r#impl::*;
impl_mod_adc! {
    use crate::atmega32u4 as hal;

    impl_adc_channels_temp!();
    impl_adc!();

    avr_hal_generic::impl_adc! {
        hal: hal::Hal,
        peripheral: hal::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
        channel_id: u8,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().bits(id & 0x1f));
            peripheral.adcsrb.modify(|_, w| w.mux5().bit(id & 0x20 != 0));
        },
        pins: {
            hal::port::PF0: (0b000000, didr0::adc0d),
            hal::port::PF1: (0b000001, didr0::adc1d),
            hal::port::PF4: (0b000100, didr0::adc4d),
            hal::port::PF5: (0b000101, didr0::adc5d),
            hal::port::PF6: (0b000110, didr0::adc6d),
            hal::port::PF7: (0b000111, didr0::adc7d),
            hal::port::PD4: (0b100000, didr2::adc8d),
            hal::port::PD6: (0b100001, didr2::adc9d),
            hal::port::PD7: (0b100010, didr2::adc10d),
            hal::port::PB4: (0b100011, didr2::adc11d),
            hal::port::PB5: (0b100100, didr2::adc12d),
            hal::port::PB6: (0b100101, didr2::adc13d),
        },
        channels: {
            channel::Vbg: 0b011110,
            channel::Gnd: 0b011111,
            channel::Temperature: 0b100111,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::atmega32u4,
    capacity: 1024,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega,
}

impl_mod_i2c! {
    use crate::atmega32u4 as hal;

    impl_i2c_peripheral! {
        i2c_type: I2c,
        peripheral: hal::pac::TWI,
        sda: hal::port::PD1,
        scl: hal::port::PD0,
    }
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
    use crate::atmega32u4 as hal;

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
        /// ```
        /// let mut timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);
        ///
        /// let mut d5 = pins.d5.into_output().into_pwm(&mut timer3);
        ///
        /// d5.set_duty(128);
        /// d5.enable();
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
}

impl_mod_spi! {
    use crate::atmega32u4 as hal;
 
    impl_spi_peripheral! {
        spi: Spi,
        peripheral: hal::pac::SPI,
        sclk: hal::port::PB1,
        mosi: hal::port::PB2,
        miso: hal::port::PB3,
        cs: hal::port::PB0,
    }
}

impl_mod_usart! {
    use crate::atmega32u4 as hal;
    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART1,
        register_suffix: 1,
        rx: hal::port::PD2,
        tx: hal::port::PD3,
        usart_type: Usart1,
    }
}

impl_mod_wdt! {
    use crate::atmega32u4 as hal;

    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}

