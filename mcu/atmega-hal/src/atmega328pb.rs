pub use avr_device::atmega328pb as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega328pb,
    pins: {
        PC0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        PC1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        PC2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        PC3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        PC4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        PC5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
    },
    channels: {
        #[cfg(feature = "enable-extra-adc")]
        ADC6: hal::pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        ADC7: hal::pac::adc::admux::MUX_A::ADC7,
        Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
        Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
    },
    impl!: impl_adc_admux,
}

impl_mod_eeprom! {
    hal: crate::atmega328pb,
    capacity: 1024,
    addr_width: u16,
    addr_reg: eear,
    impl!: avr_hal_generic::impl_eeprom_atmega,
}

impl_mod_i2c! {
    hal: crate::atmega328pb,
    interfaces: {
        I2c0: {
            peripheral: TWI0,
            sda: PC4,
            scl: PC5,
        },
        I2c1: {
            peripheral: TWI1,
            sda: PE0,
            scl: PE1,
        },
    },
}

impl_mod_port! {
    use crate::atmega328pb as hal;
    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            C: hal::pac::PORTC = [0, 1, 2, 3, 4, 5, 6],
            D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
            E: hal::pac::PORTE = [0, 1, 2, 3],
        }
    }

    #[macro_export]
    macro_rules! atmega328pb_pins {
        ($p:expr) => {
            $crate::atmega328pb::Pins::new($p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE)
        };
    }

    pub use atmega328pb_pins as pins;
}

impl_mod_simple_pwm! {
    hal: crate::atmega328pb,
    impl!: impl_simple_pwm_48p_168_328p_328pb,
    impl: {
        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC3` for PWM (pins `PD0`, `PD2`)
            pub struct Timer3Pwm {
                timer: hal::pac::TC3,
                init: |tim, prescaler| {
                    tim.tccr3a.modify(|_r, w| w.wgm3().bits(0b01));
                    tim.tccr3b.modify(|_r, w| {
                        unsafe { w.wgm3().bits(0b01) };
    
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
                    hal::port::PD0: {
                        ocr: ocr3a,
                        into_pwm: |tim| if enable {
                            tim.tccr3a.modify(|_r, w| w.com3a().match_clear());
                        } else {
                            tim.tccr3a.modify(|_r, w| w.com3a().disconnected());
                        },
                    },
    
                    hal::port::PD2: {
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
    
        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC4` for PWM (pins `PD1`, `PD2`)
            pub struct Timer4Pwm {
                timer: hal::pac::TC4,
                init: |tim, prescaler| {
                    tim.tccr4a.modify(|_r, w| w.wgm4().bits(0b01));
                    tim.tccr4b.modify(|_r, w| {
                        unsafe { w.wgm4().bits(0b01) };
    
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
                    hal::port::PD1: {
                        ocr: ocr4a,
                        into_pwm: |tim| if enable {
                            tim.tccr4a.modify(|_r, w| w.com4a().match_clear());
                        } else {
                            tim.tccr4a.modify(|_r, w| w.com4a().disconnected());
                        },
                    },
    
                    hal::port::PD2: {
                        ocr: ocr4b,
                        into_pwm: |tim| if enable {
                            tim.tccr4a.modify(|_r, w| w.com4b().match_clear());
                        } else {
                            tim.tccr4a.modify(|_r, w| w.com4b().disconnected());
                        },
                    },
                },
            }
        }
    },
}

impl_mod_spi! {
    hal: crate::atmega328pb,
    interfaces: {
        Spi0: {
            peripheral: SPI0,
            sclk: PB5,
            mosi: PB3,
            miso: PB4,
            cs: PB2,
        },
        Spi1: {
            peripheral: SPI1,
            sclk: PC1,
            mosi: PE3,
            miso: PC0,
            cs: PE2,
        },
    },
}

impl_mod_usart! {
    hal: crate::atmega328pb,
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
            rx: PB4,
            tx: PB3,
            impl!: crate::r#impl::impl_usart_traditional {
                register_suffix: 1,
            },
        },
    },
}

impl_mod_wdt! {
    use crate::atmega328pb as hal;

    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}

