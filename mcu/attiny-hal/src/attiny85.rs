pub use avr_device::attiny85 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::attiny85 as hal;

    impl_adc_reference_voltage! {
        pub enum ReferenceVoltage {
            /// Voltage applied to AREF pin.
            Aref,
            /// Default reference voltage (default).
            AVcc,
            /// Internal 1.1V reference.
            Internal1_1,
            /// Internal 2.56V reference.
            Internal2_56,
        }
    }

    impl_adc_channels! {
        pub struct Vbg;
        pub struct Gnd;
        pub struct Temperature;
    }

    impl_adc_peripheral! {
        pac: crate::attiny85::pac,
        hal: crate::attiny85::Hal,
    }

    avr_hal_generic::impl_adc! {
        hal: hal::Hal,
        peripheral: hal::pac::ADC,
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
        channel_id: hal::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            hal::port::PB5: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            hal::port::PB2: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            hal::port::PB4: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            hal::port::PB3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        },
        channels: {
            channel::Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
            channel::Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::attiny85,
    capacity: 512,
    addr_width: u16,
    addr_reg: eear,
}

impl_mod_port! {
    use crate::attiny85 as hal;

    pub use avr_hal_generic::port::{mode, PinMode, PinOps};
    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5],
        }
    }

    #[macro_export]
    macro_rules! attiny85_pins {
        ($p:expr) => {
            $crate::attiny85::Pins::new($p.PORTB)
        };
    }

    pub use attiny85_pins as pins;
}

impl_mod_simple_pwm! {
    use crate::attiny85 as hal;

    pub use avr_hal_generic::simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps};
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
            timer: crate::attiny85::pac::TC0,
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
                hal::port::PB0: {
                    ocr: ocr0a,
                    into_pwm: |tim| if enable {
                        tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                    } else {
                        tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                    },
                },

                hal::port::PB1: {
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
            timer: crate::attiny85::pac::TC1,
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
                hal::port::PB4: {
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

impl_mod_wdt! {
    hal: crate::attiny85,
    wdtcsr_name: wdtcr,
}

