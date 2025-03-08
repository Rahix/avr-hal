use crate::r#impl::avr_hal;

avr_hal! {
    device: attiny85,
    eeprom: {
        capacity: 512,
        addr_width: u16,
        addr_reg: eear,
    },
    port: {
        ports: {
            B: [0, 1, 2, 3, 4, 5],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },
    pwm: {
        timers: {
            Timer0Pwm: {
                peripheral: TC0,
                tccr: tccr0,
                pins: {
                    PB0: {
                        ocr: ocr0a,
                        com: com0a,
                    },
                    PB1: {
                        ocr: ocr0b,
                        com: com0b,
                    },
                },
                impl!: crate::r#impl::timer_8bit_impl,
            },
            Timer1Pwm: {
                peripheral: TC1,
                tccr: tccr1,
                pins: {
                    PB4: {
                        ocr: ocr1b,
                        com: com1b,
                    },
                },
                impl!: crate::r#impl::timer_8bit_separate_prescale,
            },
        },
    },
    adc: {
        references: {
            /// Voltage applied to AREF pin.
            Aref: |peripheral| {
                peripheral.admux.write(|w| w.refs().aref())
            },
            /// Default reference voltage (default).
            AVcc: |peripheral| {
                peripheral.admux.write(|w| w.refs().vcc())
            },
            /// Internal 1.1V reference.
            Internal1_1: |peripheral| {
                peripheral.admux.write(|w| w.refs().internal().refs2().clear_bit())
            },
            /// Internal 2.56V reference.
            Internal2_56: |peripheral| {
                peripheral.admux.write(|w| w.refs().internal().refs2().set_bit())
            },
        },
        pins: {
            PB5: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            PB2: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            PB4: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            PB3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        },
        channels: {
            Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
            Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
        },
    },
    wdt: {
        wdtcsr_name: wdtcr,
    },
}
