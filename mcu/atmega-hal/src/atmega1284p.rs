use crate::r#impl::avr_hal;

avr_hal! {
    device: atmega1284p,

    eeprom: {
        capacity: 4096,
        addr_width: u16,
        addr_reg: eear,
        impl!: avr_hal_generic::impl_eeprom_atmega,
    },

    port: {
        ports: {
            A: [0, 1, 2, 3, 4, 5, 6, 7],
            B: [0, 1, 2, 3, 4, 5, 6, 7],
            C: [0, 1, 2, 3, 4, 5, 6, 7],
            D: [0, 1, 2, 3, 4, 5, 6, 7],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },

    pwm: {
        timers: {
            Timer0Pwm: {
                peripheral: TC0,
                impl!: timer0_8bit_impl {
                    tccr: tccr0,
                    pins: {
                        PB3: {
                            ocr: ocr0a,
                            com: com0a,
                        },
                        PB4: {
                            ocr: ocr0b,
                            com: com0b,
                        },
                    },
                },
            },
            Timer1Pwm: {
                peripheral: TC1,
                impl!: timer_16bit_impl {
                    tccr: tccr1,
                    wgm: wgm1,
                    cs: cs1,
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
            Timer2Pwm: {
                peripheral: TC2,
                impl!: timer_8bit_2wf_with_async {
                    tccr: tccr2,
                    pins: {
                        PD7: {
                            ocr: ocr2a,
                            com: com2a,
                        },
                        PD6: {
                            ocr: ocr2b,
                            com: com2b,
                        },
                    },
                },
            },
            Timer3Pwm: {
                peripheral: TC3,
                impl!: timer_16bit_impl {
                    tccr: tccr3,
                    wgm: wgm3,
                    cs: cs3,
                    pins: {
                        PB6: {
                            ocr: ocr3a,
                            com: com3a,
                        },
                        PB7: {
                            ocr: ocr3b,
                            com: com3b,
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

    spi: {
        interfaces: {
            Spi: {
                peripheral: SPI,
                sclk: PB7,
                mosi: PB5,
                miso: PB6,
                cs: PB4,
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
    },

    wdt: {
        impl!: impl_wdt_peripheral_ms8000 {
            mcusr: hal::pac::cpu::MCUSR,
            wdtcsr_name: wdtcsr,
        },
    },
}
