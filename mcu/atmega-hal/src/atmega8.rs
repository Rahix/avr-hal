pub use avr_device::atmega8 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::atmega8 as hal;

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
            hal::port::PC0: (hal::pac::adc::admux::MUX_A::ADC0),
            hal::port::PC1: (hal::pac::adc::admux::MUX_A::ADC1),
            hal::port::PC2: (hal::pac::adc::admux::MUX_A::ADC2),
            hal::port::PC3: (hal::pac::adc::admux::MUX_A::ADC3),
            hal::port::PC4: (hal::pac::adc::admux::MUX_A::ADC4),
            hal::port::PC5: (hal::pac::adc::admux::MUX_A::ADC5),
        },
        channels: {
            #[cfg(feature = "enable-extra-adc")]
            channel::ADC6: hal::pac::adc::admux::MUX_A::ADC6,
            #[cfg(feature = "enable-extra-adc")]
            channel::ADC7: hal::pac::adc::admux::MUX_A::ADC7,
            channel::Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::atmega8,
    capacity: 512,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega_old,
}

impl_mod_i2c! {
    use crate::atmega8 as hal;

    impl_i2c_peripheral! {
        i2c_type: I2c,
        peripheral: hal::pac::TWI,
        sda: hal::port::PC4,
        scl: hal::port::PC5,
    }
}

impl_mod_port! {
    use crate::atmega8 as hal;

    impl_port_peripheral_b8_c7_d8! {
    }

    #[macro_export]
    macro_rules! atmega8_pins {
        ($p:expr) => {
            $crate::atmega8::Pins::new($p.PORTB, $p.PORTC, $p.PORTD)
        };
    }

    pub use atmega8_pins as pins;
}

impl_mod_simple_pwm! {
    use crate::atmega8 as hal;

    avr_hal_generic::impl_simple_pwm! {
            /// Use `TC1` for PWM (pins `PB1`, `PB2`)
            ///
        /// # Example
        /// ```
        /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
        ///
        /// let mut b1 = pins.b1.into_output().into_pwm(&mut timer1);
        /// let mut b2 = pins.b2.into_output().into_pwm(&mut timer1);
        ///
        /// d9.set_duty(128);
        /// d9.enable();
        /// ```
        pub struct Timer1Pwm {
            timer: hal::pac::TC1,
            init: |tim, prescaler| {
                tim.tccr1a.modify(|_r, w| w.wgm1().bits(0b01));
                tim.tccr1b.modify(|_r, w| {
                    w.wgm1().bits(0b01);

                    match prescaler {
                        Prescaler::Direct => w.cs1().direct(),
                        Prescaler::Prescale8 => w.cs1().prescale_8(),
                        Prescaler::Prescale64 => w.cs1().prescale_64(),
                        Prescaler::Prescale256 => w.cs1().prescale_256(),
                        Prescaler::Prescale1024 => w.cs1().prescale_1024(),
                    }
                });
            },
            pins: {
                hal::port::PB1: {
                    ocr: ocr1a,
                    into_pwm: |tim| if enable {
                        tim.tccr1a.modify(|_r, w| w.com1a().match_clear());
                    } else {
                        tim.tccr1a.modify(|_r, w| w.com1a().disconnected());
                    },
                },

                hal::port::PB2: {
                    ocr: ocr1b,
                    into_pwm: |tim| if enable {
                        tim.tccr1a.modify(|_r, w| w.com1b().match_clear());
                    } else {
                        tim.tccr1a.modify(|_r, w| w.com1b().disconnected());
                    },
                },
            },
        }
    }

    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC2` for PWM (pins `PB3`, `PD3`)
        ///
        /// # Example
        /// ```
        /// let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
        ///
        /// let mut d11 = pins.d11.into_output().into_pwm(&mut timer2);
        /// let mut d3 = pins.d3.into_output().into_pwm(&mut timer2);
        ///
        /// d11.set_duty(128);
        /// d11.enable();
        /// ```
        pub struct Timer2Pwm {
            timer: hal::pac::TC2,
            init: |tim, prescaler| {
                tim.tccr2.modify(|_r, w| w.wgm20().set_bit().wgm21().set_bit());
                tim.tccr2.modify(|_r, w| match prescaler {
                    Prescaler::Direct => w.cs2().direct(),
                    Prescaler::Prescale8 => w.cs2().prescale_8(),
                    Prescaler::Prescale64 => w.cs2().prescale_64(),
                    Prescaler::Prescale256 => w.cs2().prescale_256(),
                    Prescaler::Prescale1024 => w.cs2().prescale_1024(),
                });
            },
            pins: {
                hal::port::PB3: {
                    ocr: ocr2,
                    into_pwm: |tim| if enable {
                        tim.tccr2.modify(|_r, w| w.com2().match_clear());
                    } else {
                        tim.tccr2.modify(|_r, w| w.com2().disconnected());
                    },
                },
            },
        }
    }
}

impl_mod_spi! {
    use crate::atmega8 as hal;
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
    use crate::atmega8 as hal;
    impl_usart_peripheral_ubrrh_ucsrc! {
        peripheral: hal::pac::USART,
        rx: hal::port::PD0,
        tx: hal::port::PD1,
        usart_type: Usart0,
    }
}

impl_mod_wdt! {
    use crate::atmega8 as hal;
    impl_wdt_peripheral_ms2000! {
        mcusr: hal::pac::cpu::MCUCSR,
        wdtcsr_name: wdtcr,
    }
}