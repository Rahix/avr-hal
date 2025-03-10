use crate::r#impl::avr_hal;

avr_hal! {
    device: atmega2560,

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
            E: [0, 1, 2, 3, 4, 5, 6, 7],
            F: [0, 1, 2, 3, 4, 5, 6, 7],
            G: [0, 1, 2, 3, 4, 5],
            H: [0, 1, 2, 3, 4, 5, 6, 7],
            J: [0, 1, 2, 3, 4, 5, 6, 7],
            K: [0, 1, 2, 3, 4, 5, 6, 7],
            L: [0, 1, 2, 3, 4, 5, 6, 7],
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
                        PB7: {
                            ocr: ocr0a,
                            com: com0a,
                        },
                        PG5: {
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
                        PB5: {
                            ocr: ocr1a,
                            com: com1a,
                        },
                        PB6: {
                            ocr: ocr1b,
                            com: com1b,
                        },
                        PB7: {
                            ocr: ocr1c,
                            com: com1c,
                        },
                    },
                },
            },
            Timer2Pwm: {
                peripheral: TC2,
                impl!: timer_8bit_2wf_with_async {
                    tccr: tccr2,
                    pins: {
                        PB4: {
                            ocr: ocr2a,
                            com: com2a,
                        },
                        PH6: {
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
                        PE3: {
                            ocr: ocr3a,
                            com: com3a,
                        },
                        PE4: {
                            ocr: ocr3b,
                            com: com3b,
                        },
                        PE5: {
                            ocr: ocr3c,
                            com: com3c,
                        },
                    },
                },
            },
            Timer4Pwm: {
                peripheral: TC4,
                impl!: timer_16bit_impl {
                    tccr: tccr4,
                    wgm: wgm4,
                    cs: cs4,
                    pins: {
                        PH3: {
                            ocr: ocr4a,
                            com: com4a,
                        },
                        PH4: {
                            ocr: ocr4b,
                            com: com4b,
                        },
                        PH5: {
                            ocr: ocr4c,
                            com: com4c,
                        },
                    },
                },
            },
            Timer5Pwm: {
                peripheral: TC5,
                impl!: timer_16bit_impl {
                    tccr: tccr5,
                    wgm: wgm5,
                    cs: cs5,
                    pins: {
                        PL3: {
                            ocr: ocr5a,
                            com: com5a,
                        },
                        PL4: {
                            ocr: ocr5b,
                            com: com5b,
                        },
                        PL5: {
                            ocr: ocr5c,
                            com: com5c,
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
                sda: PD1,
                scl: PD0,
            },
        },
    },

   spi: {
        interfaces: {
            Spi: {
                peripheral: SPI,
                sclk: PB1,
                mosi: PB2,
                miso: PB3,
                cs: PB0,
            },
        },
    },

    usart: {
        interfaces: {
            Usart0: {
                peripheral: USART0,
                rx: PE0,
                tx: PE1,
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
            PF0: (0b000000, didr0::adc0d),
            PF1: (0b000001, didr0::adc1d),
            PF2: (0b000010, didr0::adc2d),
            PF3: (0b000011, didr0::adc3d),
            PF4: (0b000100, didr0::adc4d),
            PF5: (0b000101, didr0::adc5d),
            PF6: (0b000110, didr0::adc6d),
            PF7: (0b000111, didr0::adc7d),
            PK0: (0b100000, didr2::adc8d),
            PK1: (0b100001, didr2::adc9d),
            PK2: (0b100010, didr2::adc10d),
            PK3: (0b100011, didr2::adc11d),
            PK4: (0b100100, didr2::adc12d),
            PK5: (0b100101, didr2::adc13d),
            PK6: (0b100110, didr2::adc14d),
            PK7: (0b100111, didr2::adc15d),
        },
        channels: {
            Vbg: 0b011110,
            Gnd: 0b011111,
        },
        impl!: impl_adc_admux_adcsrb,
    },

   wdt: {
        impl!: impl_wdt_peripheral_ms8000 {
            mcusr: hal::pac::cpu::MCUSR,
            wdtcsr_name: wdtcsr,
        },
    },
}
