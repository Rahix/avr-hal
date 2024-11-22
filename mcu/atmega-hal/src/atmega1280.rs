pub use avr_device::atmega1280 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::atmega1280 as hal;
    impl_adc_channels!();
    impl_adc!();

    avr_hal_generic::impl_adc! {
        hal: hal::Hal,
        peripheral: hal::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
        channel_id: u8,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().bits(id & 0x1f));
            peripheral.adcsrb.modify(|_, w| w.mux5().bit(id & 0x20 != 0));
        },
        pins: {
            hal::port::PF0: (0b000000, didr0::adc0d),
            hal::port::PF1: (0b000001, didr0::adc1d),
            hal::port::PF2: (0b000010, didr0::adc2d),
            hal::port::PF3: (0b000011, didr0::adc3d),
            hal::port::PF4: (0b000100, didr0::adc4d),
            hal::port::PF5: (0b000101, didr0::adc5d),
            hal::port::PF6: (0b000110, didr0::adc6d),
            hal::port::PF7: (0b000111, didr0::adc7d),
            hal::port::PK0: (0b100000, didr2::adc8d),
            hal::port::PK1: (0b100001, didr2::adc9d),
            hal::port::PK2: (0b100010, didr2::adc10d),
            hal::port::PK3: (0b100011, didr2::adc11d),
            hal::port::PK4: (0b100100, didr2::adc12d),
            hal::port::PK5: (0b100101, didr2::adc13d),
            hal::port::PK6: (0b100110, didr2::adc14d),
            hal::port::PK7: (0b100111, didr2::adc15d),
        },
        channels: {
            channel::Vbg: 0b011110,
            channel::Gnd: 0b011111,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::atmega1280,
    capacity: 4096,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega,
}

impl_mod_i2c! {
    use crate::atmega1280 as hal;
    impl_i2c_peripheral! {
        i2c_type: I2c,
        peripheral: hal::pac::TWI,
        sda: hal::port::PD1,
        scl: hal::port::PD0,
    }
}

impl_mod_port! {
    use crate::atmega1280 as hal;
    impl_port_peripheral_a8_b8_c8_d8_e8_f8_g6_h8_j8_k8_l8! {
    }

    #[macro_export]
    macro_rules! atmega1280_pins {
        ($p:expr) => {
            $crate::atmega1280::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF, $p.PORTG, $p.PORTH, $p.PORTJ, $p.PORTK, $p.PORTL)
        };
    }

    pub use atmega1280_pins as pins;
}

impl_mod_simple_pwm! {
    use crate::atmega1280 as hal;
    impl_simple_pwm_peripheral_1280_2560! {
    }
}

impl_mod_spi! {
    use crate::atmega1280 as hal;
    impl_spi_peripheral! {
        spi: Spi,
        peripheral: hal::pac::SPI,
        sclk: hal::port::PB1,
        mosi: hal::port::PB2,
        miso: hal::port::PB3,
        cs: hal::port::PB0,
    }
}

impl_mod_usart! {
    use crate::atmega1280 as hal;
    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART0,
        register_suffix: 0,
        rx: hal::port::PE0,
        tx: hal::port::PE1,
        usart_type: Usart0,
    }

    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART1,
        register_suffix: 1,
        rx: hal::port::PD2,
        tx: hal::port::PD3,
        usart_type: Usart1,
    }

    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART2,
        register_suffix: 2,
        rx: hal::port::PH0,
        tx: hal::port::PH1,
        usart_type: Usart2,
    }

    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART3,
        register_suffix: 3,
        rx: hal::port::PJ0,
        tx: hal::port::PJ1,
        usart_type: Usart3,
    }
}

impl_mod_wdt! {
    use crate::atmega1280 as hal;
    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}
