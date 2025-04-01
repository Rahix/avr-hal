use crate::r#impl::avr_hal;

avr_hal! {
    device: attiny84,
    eeprom: {
        capacity: 512,
        addr_width: u16,
        addr_reg: eear,
    },
    port: {
        ports: {
            A: [0, 1, 2, 3, 4, 5, 6, 7],
            B: [0, 1, 2, 3],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },
    pwm: {
        timers: {
            Timer0Pwm: {
                peripheral: TC0,
                tccr: tccr0,
                pins: {
                    PB2: {
                        ocr: ocr0a,
                        com: com0a,
                    },
                    PA7: {
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
                    PA6: {
                        ocr: ocr1a,
                        com: com1a,
                    },
                    PA5: {
                        ocr: ocr1b,
                        com: com1b,
                    },
                },
                impl!: crate::r#impl::timer_16bit_impl,
            },
        },
    },
    wdt: {
        wdtcsr_name: wdtcsr,
    },
}
