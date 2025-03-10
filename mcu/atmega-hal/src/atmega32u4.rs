use crate::r#impl::avr_hal;

avr_hal! {
    device: atmega32u4,

    eeprom: {
        capacity: 1024,
        addr_width: u16,
        addr_reg: eear,
        impl!: avr_hal_generic::impl_eeprom_atmega,
    },

    port: {
        ports: {
            B: [0, 1, 2, 3, 4, 5, 6, 7],
            C: [6, 7],
            D: [0, 1, 2, 3, 4, 5, 6, 7],
            E: [2, 6],
            F: [0, 1, 4, 5, 6, 7],
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
                        PD0: {
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
            Timer3Pwm: {
                peripheral: TC3,
                impl!: timer_16bit_impl {
                    tccr: tccr3,
                    wgm: wgm3,
                    cs: cs3,
                    pins: {
                        PC6: {
                            ocr: ocr3a,
                            com: com3a,
                        },
                    },
                },
            },
            Timer4Pwm: {
                peripheral: TC4,
                impl!: timer_10bit_impl {
                    tccr: tccr4,
                    pwm: pwm4,
                    cs: cs4,
                    pins: {
                        PC7: {
                            pin_tccr: tccr4a,
                            ocr: ocr4a,
                            com: com4a,
                        },
                        PB6: {
                            pin_tccr: tccr4a,
                            ocr: ocr4b,
                            com: com4b,
                        },
                        PD7: {
                            pin_tccr: tccr4c,
                            ocr: ocr4d,
                            com: com4d,
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
    },

   wdt: {
        impl!: impl_wdt_peripheral_ms8000 {
            mcusr: hal::pac::cpu::MCUSR,
            wdtcsr_name: wdtcsr,
        },
    },
}
