use crate::r#impl::avr_hal;

avr_hal! {
    device: atmega32a,

    eeprom: {
        capacity: 1024,
        addr_width: u16,
        addr_reg: eear,
        impl!: avr_hal_generic::impl_eeprom_atmega_old,
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
                peripheral: USART,
                rx: PD0,
                tx: PD1,
                impl!: crate::r#impl::impl_usart_ubrrh_ucsrc,
            },
        },
    },

    adc: {
        pins: {
            PA0: (hal::pac::adc::admux::MUX_A::ADC0),
            PA1: (hal::pac::adc::admux::MUX_A::ADC1),
            PA2: (hal::pac::adc::admux::MUX_A::ADC2),
            PA3: (hal::pac::adc::admux::MUX_A::ADC3),
            PA4: (hal::pac::adc::admux::MUX_A::ADC4),
            PA5: (hal::pac::adc::admux::MUX_A::ADC5),
            PA6: (hal::pac::adc::admux::MUX_A::ADC6),
            PA7: (hal::pac::adc::admux::MUX_A::ADC7),
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
