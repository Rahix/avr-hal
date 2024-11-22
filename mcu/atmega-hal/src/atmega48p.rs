pub use avr_device::atmega48p as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega48p,
    pins: {
        PC0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        PC1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        PC2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        PC3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        PC4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        PC5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
    },
    channels: {
        #[cfg(feature = "enable-extra-adc")]
        ADC6: hal::pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        ADC7: hal::pac::adc::admux::MUX_A::ADC7,
        Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
        Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
    },
    impl!: impl_adc_admux,
}

impl_mod_eeprom! {
    hal: crate::atmega48p,
    capacity: 256,
    addr_width: u8,
    addr_reg: eearl,
    impl!: avr_hal_generic::impl_eeprom_atmega,
}

impl_mod_i2c! {
    hal: crate::atmega48p,
    interfaces: {
        I2c: {
            peripheral: TWI,
            sda: PC4,
            scl: PC5,
        },
    },
}

impl_mod_port! {
    use crate::atmega48p as hal;
    impl_port_peripheral_b8_c7_d8! {
    }

    #[macro_export]
    macro_rules! atmega48p_pins {
        ($p:expr) => {
            $crate::atmega48p::Pins::new($p.PORTB, $p.PORTC, $p.PORTD)
        };
    }

    pub use atmega48p_pins as pins;
}

impl_mod_simple_pwm! {
    hal: crate::atmega48p,
    impl!: impl_simple_pwm_48p_168_328p_328pb,
}

impl_mod_spi! {
    hal: crate::atmega48p,
    interfaces: {
        Spi: {
            peripheral: SPI,
            sclk: PB5,
            mosi: PB3,
            miso: PB4,
            cs: PB2,
        },
    },
}

impl_mod_wdt! {
    use crate::atmega48p as hal;
    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}

