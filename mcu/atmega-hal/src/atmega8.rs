use crate::r#impl::avr_hal;

avr_hal! {
    device: atmega8,

    eeprom: {
        capacity: 512,
        addr_width: u16,
        addr_reg: eear,
        impl!: avr_hal_generic::impl_eeprom_atmega_old,
    },

    port: {
        ports: {
            B: [0, 1, 2, 3, 4, 5, 6, 7],
            C: [0, 1, 2, 3, 4, 5, 6],
            D: [0, 1, 2, 3, 4, 5, 6, 7],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },

    pwm: {
        timers: {
            Timer1Pwm: {
                peripheral: TC1,
                impl!: timer_16bit_impl {
                    tccr: tccr1,
                    wgm: wgm1,
                    cs: cs1,
                    pins: {
                        PB1: {
                            ocr: ocr1a,
                            com: com1a,
                        },
                        PB2: {
                            ocr: ocr1a,
                            com: com1a,
                        },
                    },
                },
            },
            Timer2Pwm: {
                peripheral: TC2,
                impl!: timer_8bit_1wf_with_async {
                    tccr: tccr2,
                    pins: {
                        PB1: {
                            ocr: ocr2,
                            com: com2,
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
                sda: PC4,
                scl: PC5,
            },
        },
    },

    spi: {
        interfaces: {
            Spi: {
                peripheral: SPI,
                sclk: PB5,
                mosi: PB3,
                miso: PB4,
                cs: PB2,
            },
        },
    },

    usart: {
        interfaces: {
            Usart0: {
                peripheral: USART,
                rx: PD0,
                tx: PD1,
                impl!: crate::r#impl::impl_usart_ubrrh_ucsrc,
            },
        },
    },

    adc: {
        pins: {
            PC0: (hal::pac::adc::admux::MUX_A::ADC0),
            PC1: (hal::pac::adc::admux::MUX_A::ADC1),
            PC2: (hal::pac::adc::admux::MUX_A::ADC2),
            PC3: (hal::pac::adc::admux::MUX_A::ADC3),
            PC4: (hal::pac::adc::admux::MUX_A::ADC4),
            PC5: (hal::pac::adc::admux::MUX_A::ADC5),
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
        impl!: impl_wdt_peripheral_ms2000 {
            mcusr: hal::pac::cpu::MCUCSR,
            wdtcsr_name: wdtcr,
        },
    },
}
