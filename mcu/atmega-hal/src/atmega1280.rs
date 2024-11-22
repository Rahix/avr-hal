pub use avr_device::atmega1280 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega1280,
    pins: {
        PF0: (0b000000, didr0::adc0d),
        PF1: (0b000001, didr0::adc1d),
        PF2: (0b000010, didr0::adc2d),
        PF3: (0b000011, didr0::adc3d),
        PF4: (0b000100, didr0::adc4d),
        PF5: (0b000101, didr0::adc5d),
        PF6: (0b000110, didr0::adc6d),
        PF7: (0b000111, didr0::adc7d),
        PK0: (0b100000, didr2::adc8d),
        PK1: (0b100001, didr2::adc9d),
        PK2: (0b100010, didr2::adc10d),
        PK3: (0b100011, didr2::adc11d),
        PK4: (0b100100, didr2::adc12d),
        PK5: (0b100101, didr2::adc13d),
        PK6: (0b100110, didr2::adc14d),
        PK7: (0b100111, didr2::adc15d),
    },
    channels: {
        Vbg: 0b011110,
        Gnd: 0b011111,
    },
    impl!: impl_adc_admux_adcsrb,
}

impl_mod_eeprom! {
    hal: crate::atmega1280,
    capacity: 4096,
    addr_width: u16,
    addr_reg: eear,
    impl!: avr_hal_generic::impl_eeprom_atmega,
}

impl_mod_i2c! {
    hal: crate::atmega1280,
    interfaces: {
        I2c: {
            peripheral: TWI,
            sda: PD1,
            scl: PD0,
        },
    },
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
    hal: crate::atmega1280,
    impl!: impl_simple_pwm_1280_2560,
}

impl_mod_spi! {
    hal: crate::atmega1280,
    interfaces: {
        Spi: {
            peripheral: SPI,
            sclk: PB1,
            mosi: PB2,
            miso: PB3,
            cs: PB0,
        },
    },
}

impl_mod_usart! {
    hal: crate::atmega1280,
    interfaces: {
        Usart0: {
            peripheral: USART0,
            rx: PE0,
            tx: PE1,
            impl!: crate::r#impl::impl_usart_traditional {
                register_suffix: 0,
            },
        },
        Usart1: {
            peripheral: USART1,
            rx: PD2,
            tx: PD3,
            impl!: crate::r#impl::impl_usart_traditional {
                register_suffix: 1,
            },
        },
        Usart2: {
            peripheral: USART2,
            rx: PH0,
            tx: PH1,
            impl!: crate::r#impl::impl_usart_traditional {
                register_suffix: 2,
            },
        },
        Usart3: {
            peripheral: USART3,
            rx: PJ0,
            tx: PJ1,
            impl!: crate::r#impl::impl_usart_traditional {
                register_suffix: 3,
            },
        },
    },
}

impl_mod_wdt! {
    use crate::atmega1280 as hal;
    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}
