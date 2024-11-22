pub use avr_device::atmega32a as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega32a,
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
}


impl_mod_eeprom! {
    hal: crate::atmega32a,
    capacity: 1024,
    addr_width: u16,
    addr_reg: eear,
    impl!: avr_hal_generic::impl_eeprom_atmega_old,
}

impl_mod_i2c! {
    hal: crate::atmega32a,
    interfaces: {
        I2c: {
            peripheral: TWI,
            sda: PC1,
            scl: PC0,
        },
    },
}

impl_mod_port! {
    use crate::atmega32a as hal;

    impl_port_peripheral_a8_b8_c8_d8! {
    }

    #[macro_export]
    macro_rules! atmega32a_pins {
        ($p:expr) => {
            $crate::atmega32a::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
        };
    }

    pub use atmega32a_pins as pins;
}

impl_mod_spi! {
    hal: crate::atmega32a,
    interfaces: {
        Spi: {
            peripheral: SPI,
            sclk: PB7,
            mosi: PB5,
            miso: PB6,
            cs: PB4,
        },
    },
}

impl_mod_usart! {
    hal: crate::atmega32a,
    interfaces: {
        Usart0: {
            peripheral: USART,
            rx: PD0,
            tx: PD1,
            impl!: crate::r#impl::impl_usart_ubrrh_ucsrc,
        },
    },
}

impl_mod_wdt! {
    use crate::atmega32a as hal;

    impl_wdt_peripheral_ms2000! {
        mcusr: hal::pac::cpu::MCUCSR,
        wdtcsr_name: wdtcr,
    }
}

