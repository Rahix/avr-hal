pub mod adc {
    pub use crate::periphals::adc::*;
    use crate::port;

    avr_hal_generic::impl_adc! {
        hal: crate::Attiny,
        peripheral: crate::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| {
            apply_clock(peripheral, settings);
            peripheral.admux.write(|w| match settings.ref_voltage {
                ReferenceVoltage::Aref => w.refs().aref(),
                ReferenceVoltage::AVcc => w.refs().vcc(),
                ReferenceVoltage::Internal1_1 => w.refs().internal().refs2().clear_bit(),
                ReferenceVoltage::Internal2_56 => w.refs().internal().refs2().set_bit(),
            });
        },
        channel_id: crate::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            port::PB5: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            port::PB2: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            port::PB4: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            port::PB3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        },
        channels: {
            channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
            channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
        },
    }
}

pub mod eeprom {
    pub use crate::periphals::eeprom::*;

    avr_hal_generic::impl_eeprom_attiny! {
        hal: crate::Attiny,
        peripheral: crate::pac::EEPROM,
        capacity: 512,
        addr_width: u16,
        set_address: |peripheral, address| {
            peripheral.eear.write(|w| w.bits(address));
        },
    }
}

pub mod port {
    pub use crate::periphals::port::*;

    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5],
        }
    }
}

pub mod simple_pwm {
    pub use crate::periphals::simple_pwm::*;

    use crate::port::*;

    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC0` for PWM (pins `PB0`, `PB1`)
        ///
        /// # Example
        /// ```
        /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
        ///
        /// let mut d0 = pins.d0.into_output().into_pwm(&mut timer0);
        /// let mut d1 = pins.d1.into_output().into_pwm(&mut timer0);
        ///
        /// d0.set_duty(128);
        /// d0.enable();
        /// ```
        pub struct Timer0Pwm {
            timer: crate::pac::TC0,
            init: |tim, prescaler| {
                tim.tccr0a.modify(|_r, w| w.wgm0().pwm_fast());
                tim.tccr0b.modify(|_r, w| match prescaler {
                    Prescaler::Direct => w.cs0().direct(),
                    Prescaler::Prescale8 => w.cs0().prescale_8(),
                    Prescaler::Prescale64 => w.cs0().prescale_64(),
                    Prescaler::Prescale256 => w.cs0().prescale_256(),
                    Prescaler::Prescale1024 => w.cs0().prescale_1024(),
                });
            },
            pins: {
                PB0: {
                    ocr: ocr0a,
                    into_pwm: |tim| if enable {
                        tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                    } else {
                        tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                    },
                },

                PB1: {
                    ocr: ocr0b,
                    into_pwm: |tim| if enable {
                        tim.tccr0a.modify(|_r, w| w.com0b().match_clear());
                    } else {
                        tim.tccr0a.modify(|_r, w| w.com0b().disconnected());
                    },
                },
            },
        }
    }

    #[cfg(feature = "attiny85")]
    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC1` for PWM (pins `PB4`)
        ///
        /// # Example
        /// ```
        /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
        ///
        /// let mut d4 = pins.d4.into_output().into_pwm(&mut timer1);
        ///
        /// d4.set_duty(128);
        /// d4.enable();
        /// ```
        pub struct Timer1Pwm {
            timer: crate::pac::TC1,
            init: |tim, prescaler| {
                tim.gtccr.modify(|_, w| w.pwm1b().bit(true));

                tim.tccr1.modify(|_r, w| match prescaler {
                    Prescaler::Direct => w.cs1().direct(),
                    Prescaler::Prescale8 => w.cs1().prescale_8(),
                    Prescaler::Prescale64 => w.cs1().prescale_64(),
                    Prescaler::Prescale256 => w.cs1().prescale_256(),
                    Prescaler::Prescale1024 => w.cs1().prescale_1024(),
                });
            },
            pins: {
                PB4: {
                    ocr: ocr1b,
                    into_pwm: |tim| if enable {
                        tim.gtccr.modify(|_, w| w.com1b().bits(0b10));
                    } else {
                        tim.gtccr.modify(|_, w| w.com1b().disconnected());
                    },
                },
            },
        }
    }
}

pub mod spi {
    pub use crate::periphals::spi::*;

    // Fixme: Implement SPI for ATtiny85.
}
