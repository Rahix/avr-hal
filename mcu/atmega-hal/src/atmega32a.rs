pub use avr_device::atmega32a as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::atmega32a as hal;

    impl_adc_channels_extra!();
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
            hal::port::PA0: (hal::pac::adc::admux::MUX_A::ADC0),
            hal::port::PA1: (hal::pac::adc::admux::MUX_A::ADC1),
            hal::port::PA2: (hal::pac::adc::admux::MUX_A::ADC2),
            hal::port::PA3: (hal::pac::adc::admux::MUX_A::ADC3),
            hal::port::PA4: (hal::pac::adc::admux::MUX_A::ADC4),
            hal::port::PA5: (hal::pac::adc::admux::MUX_A::ADC5),
            hal::port::PA6: (hal::pac::adc::admux::MUX_A::ADC6),
            hal::port::PA7: (hal::pac::adc::admux::MUX_A::ADC7),
        },
        channels: {
            channel::Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::atmega32a,
    capacity: 1024,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega_old,
}

impl_mod_i2c! {
    use crate::atmega32a as hal;

    impl_i2c_peripheral! {
        i2c_type: I2c,
        peripheral: hal::pac::TWI,
        sda: hal::port::PC1,
        scl: hal::port::PC0,
    }
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
    use crate::atmega32a as hal;
    impl_spi_peripheral! {
        spi: Spi,
        peripheral: hal::pac::SPI,
        sclk: hal::port::PB7,
        mosi: hal::port::PB5,
        miso: hal::port::PB6,
        cs: hal::port::PB4,
    }
}

impl_mod_usart! {
    use crate::atmega32a as hal;
    impl_usart_peripheral_ubrrh_ucsrc! {
        peripheral: hal::pac::USART,
        rx: hal::port::PD0,
        tx: hal::port::PD1,
        usart_type: Usart0,
    }
}

impl_mod_wdt! {
    use crate::atmega32a as hal;

    impl_wdt_peripheral_ms2000! {
        mcusr: hal::pac::cpu::MCUCSR,
        wdtcsr_name: wdtcr,
    }
}

