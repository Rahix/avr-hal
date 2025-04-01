use crate::r#impl::avr_hal;

avr_hal! {
    device: attiny88,
    eeprom: {
        capacity: 64,
        addr_width: u8,
        addr_reg: eearl,
    },
    port: {
        ports: {
            A: [0, 1, 2, 3],
            B: [0, 1, 2, 3, 4, 5, 6, 7],
            C: [0, 1, 2, 3, 4, 5, 6, 7],
            D: [0, 1, 2, 3, 4, 5, 6, 7],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },
    pwm: {
        timers: {
            Timer1Pwm: {
                peripheral: TC1,
                tccr: tccr1,
                pins: {
                    PB1: {
                        ocr: ocr1a,
                        com: com1a,
                    },
                    PB2: {
                        ocr: ocr1b,
                        com: com1b,
                    },
                },
                impl!: crate::r#impl::timer_16bit_impl,
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
                impl!: avr_hal_generic::impl_spi,
            },
        },
    },
    adc: {
        references: {
            /// Default reference voltage (default).
            AVcc: |peripheral| {
                peripheral.admux.write(|w| w.refs0().avcc())
            },
            /// Internal 1.1V reference.
            Internal1_1: |peripheral| {
                peripheral.admux.write(|w| w.refs0().internal())
            },
        },
        pins: {
            PC0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            PC1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            PC2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            PC3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            PC4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            PC5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
            PA0: (hal::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            PA1: (hal::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        },
        channels: {
            Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
            Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
        },
    },
    wdt: {
        wdtcsr_name: wdtcsr,
    },
}
