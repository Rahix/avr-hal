pub use avr_device::atmega328p as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::atmega328p as hal;
    impl_adc_channels_extra_temp!();
    impl_adc!();

    avr_hal_generic::impl_adc! {
        hal: hal::Hal,
        peripheral: hal::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
        channel_id: hal::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            hal::port::PC0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            hal::port::PC1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            hal::port::PC2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            hal::port::PC3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            hal::port::PC4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            hal::port::PC5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
        },
        channels: {
            #[cfg(feature = "enable-extra-adc")]
            channel::ADC6: hal::pac::adc::admux::MUX_A::ADC6,
            #[cfg(feature = "enable-extra-adc")]
            channel::ADC7: hal::pac::adc::admux::MUX_A::ADC7,
            channel::Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
            channel::Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::atmega328p,
    capacity: 1024,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega,
}

impl_mod_i2c! {
    use crate::atmega328p as hal;
    impl_i2c_peripheral! {
        i2c_type: I2c,
        peripheral: hal::pac::TWI,
        sda: hal::port::PC4,
        scl: hal::port::PC5,
    }
}

impl_mod_port! {
    use crate::atmega328p as hal;
    impl_port_peripheral_b8_c7_d8! {
    }

    #[macro_export]
    macro_rules! atmega328p_pins {
        ($p:expr) => {
            $crate::atmega328p::Pins::new($p.PORTB, $p.PORTC, $p.PORTD)
        };
    }

    pub use atmega328p_pins as pins;
}

impl_mod_simple_pwm! {
    use crate::atmega328p as hal;
    impl_simple_pwm_peripheral_48p_168_328p_328pb! {
    }
}

impl_mod_spi! {
    use crate::atmega328p as hal;
    impl_spi_peripheral! {
        spi: Spi,
        peripheral: hal::pac::SPI,
        sclk: hal::port::PB5,
        mosi: hal::port::PB3,
        miso: hal::port::PB4,
        cs: hal::port::PB2,
    }
}

impl_mod_usart! {
    use crate::atmega328p as hal;
    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART0,
        register_suffix: 0,
        rx: hal::port::PD0,
        tx: hal::port::PD1,
        usart_type: Usart0,
    }
}

impl_mod_wdt! {
    use crate::atmega328p as hal;
    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}
