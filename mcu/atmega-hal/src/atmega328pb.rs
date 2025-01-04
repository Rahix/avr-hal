pub use avr_device::atmega328pb as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::atmega328pb as hal;
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
    hal: crate::atmega328pb,
    capacity: 1024,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega,
}

impl_mod_i2c! {
    use crate::atmega328pb as hal;
    impl_i2c_peripheral! {
        i2c_type: I2c0,
        peripheral: hal::pac::TWI0,
        sda: hal::port::PC4,
        scl: hal::port::PC5,
    }

    impl_i2c_peripheral! {
        i2c_type: I2c1,
        peripheral: hal::pac::TWI1,
        sda: hal::port::PE0,
        scl: hal::port::PE1,
    }
}

impl_mod_port! {
    use crate::atmega328pb as hal;
    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            C: hal::pac::PORTC = [0, 1, 2, 3, 4, 5, 6],
            D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
            E: hal::pac::PORTE = [0, 1, 2, 3],
        }
    }

    #[macro_export]
    macro_rules! atmega328pb_pins {
        ($p:expr) => {
            $crate::atmega328pb::Pins::new($p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE)
        };
    }

    pub use atmega328pb_pins as pins;
}

impl_mod_simple_pwm! {
    use crate::atmega328pb as hal;
    impl_simple_pwm_peripheral_48p_168_328p_328pb! {
    }

    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC3` for PWM (pins `PD0`, `PD2`)
        pub struct Timer3Pwm {
            timer: hal::pac::TC3,
            init: |tim, prescaler| {
                tim.tccr3a.modify(|_r, w| w.wgm3().bits(0b01));
                tim.tccr3b.modify(|_r, w| {
                    unsafe { w.wgm3().bits(0b01) };

                    match prescaler {
                        Prescaler::Direct => w.cs3().direct(),
                        Prescaler::Prescale8 => w.cs3().prescale_8(),
                        Prescaler::Prescale64 => w.cs3().prescale_64(),
                        Prescaler::Prescale256 => w.cs3().prescale_256(),
                        Prescaler::Prescale1024 => w.cs3().prescale_1024(),
                    }
                });
            },
            pins: {
                hal::port::PD0: {
                    ocr: ocr3a,
                    into_pwm: |tim| if enable {
                        tim.tccr3a.modify(|_r, w| w.com3a().match_clear());
                    } else {
                        tim.tccr3a.modify(|_r, w| w.com3a().disconnected());
                    },
                },

                hal::port::PD2: {
                    ocr: ocr3b,
                    into_pwm: |tim| if enable {
                        tim.tccr3a.modify(|_r, w| w.com3b().match_clear());
                    } else {
                        tim.tccr3a.modify(|_r, w| w.com3b().disconnected());
                    },
                },
            },
        }
    }

    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC4` for PWM (pins `PD1`, `PD2`)
        pub struct Timer4Pwm {
            timer: hal::pac::TC4,
            init: |tim, prescaler| {
                tim.tccr4a.modify(|_r, w| w.wgm4().bits(0b01));
                tim.tccr4b.modify(|_r, w| {
                    unsafe { w.wgm4().bits(0b01) };

                    match prescaler {
                        Prescaler::Direct => w.cs4().direct(),
                        Prescaler::Prescale8 => w.cs4().prescale_8(),
                        Prescaler::Prescale64 => w.cs4().prescale_64(),
                        Prescaler::Prescale256 => w.cs4().prescale_256(),
                        Prescaler::Prescale1024 => w.cs4().prescale_1024(),
                    }
                });
            },
            pins: {
                hal::port::PD1: {
                    ocr: ocr4a,
                    into_pwm: |tim| if enable {
                        tim.tccr4a.modify(|_r, w| w.com4a().match_clear());
                    } else {
                        tim.tccr4a.modify(|_r, w| w.com4a().disconnected());
                    },
                },

                hal::port::PD2: {
                    ocr: ocr4b,
                    into_pwm: |tim| if enable {
                        tim.tccr4a.modify(|_r, w| w.com4b().match_clear());
                    } else {
                        tim.tccr4a.modify(|_r, w| w.com4b().disconnected());
                    },
                },
            },
        }
    }
}

impl_mod_spi! {
    use crate::atmega328pb as hal;
    impl_spi_peripheral! {
        spi: Spi0,
        peripheral: hal::pac::SPI0,
        sclk: hal::port::PB5,
        mosi: hal::port::PB3,
        miso: hal::port::PB4,
        cs: hal::port::PB2,
    }

    impl_spi_peripheral! {
        spi: Spi1,
        peripheral: hal::pac::SPI1,
        sclk: hal::port::PC1,
        mosi: hal::port::PE3,
        miso: hal::port::PC0,
        cs: hal::port::PE2,
    }
}

impl_mod_usart! {
    use crate::atmega328pb as hal;
    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART0,
        register_suffix: 0,
        rx: hal::port::PD0,
        tx: hal::port::PD1,
        usart_type: Usart0,
    }

    impl_usart_peripheral_traditional! {
        peripheral: hal::pac::USART1,
        register_suffix: 1,
        rx: hal::port::PB4,
        tx: hal::port::PB3,
        usart_type: Usart1,
    }
}

impl_mod_wdt! {
    use crate::atmega328pb as hal;

    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}

