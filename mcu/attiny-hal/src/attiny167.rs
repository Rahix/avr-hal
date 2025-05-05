use crate::r#impl::avr_hal;

avr_hal! {
    device: attiny167,
    eeprom: {
        capacity: 512,
        addr_width: u16,
        addr_reg: eear,
    },
    port: {
        ports: {
            A: [0, 1, 2, 3, 4, 5, 6, 7],
            B: [0, 1, 2, 3, 4, 5, 6, 7],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },
    spi: {
        interfaces: {
            Spi: {
                peripheral: SPI,
                sclk: PA5,
                mosi: PA4,
                miso: PA2,
                cs: PA6,
                impl!: avr_hal_generic::impl_spi,
            },
        },
    },
    adc: {
        references: {
            /// Voltage applied to AREF pin.
            Aref: |peripheral| {
                peripheral.amiscr.write(|w| w.arefen().set_bit());
                peripheral.admux.write(|w| w.refs().avcc());
            },
            /// Default reference voltage (default).
            AVcc: |peripheral| {
                peripheral.amiscr.write(|w| w.arefen().clear_bit());
                peripheral.admux.write(|w| w.refs().avcc());
            },
            /// Internal 1.1V reference.
            Internal1_1: |peripheral| {
                peripheral.amiscr.write(|w| w.arefen().clear_bit());
                peripheral.admux.write(|w| w.refs().internal_11());
            },
            /// Internal 2.56V reference.
            Internal2_56: |peripheral| {
                peripheral.amiscr.write(|w| w.arefen().clear_bit());
                peripheral.admux.write(|w| w.refs().internal_256());
            },
        },
        pins: {
            PA0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            PA1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            PA2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            PA3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            PA4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            PA5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
            PA6: (hal::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            PA7: (hal::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
            PB5: (hal::pac::adc::admux::MUX_A::ADC8, didr1::adc8d),
            PB6: (hal::pac::adc::admux::MUX_A::ADC9, didr1::adc9d),
            PB7: (hal::pac::adc::admux::MUX_A::ADC10, didr1::adc10d),
        },
        channels: {
            AVcc_4: hal::pac::adc::admux::MUX_A::ADC_AVCC_4,
            Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
            Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
        },
    },
    wdt: {
        wdtcsr_name: wdtcr,
    },
}
