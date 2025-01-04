pub use avr_device::atmega164pa as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::atmega164pa as hal;
    impl_adc_channels!();
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
            hal::port::PA0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            hal::port::PA1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            hal::port::PA2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            hal::port::PA3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            hal::port::PA4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            hal::port::PA5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
            hal::port::PA6: (hal::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            hal::port::PA7: (hal::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        },
        channels: {
            channel::Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::atmega164pa,
    capacity: 512,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega,
}

impl_mod_i2c! {
    use crate::atmega164pa as hal;
    impl_i2c_peripheral! {
        i2c_type: I2c,
        peripheral: hal::pac::TWI,
        sda: hal::port::PC1,
        scl: hal::port::PC0,
    }
}

impl_mod_port! {
    use crate::atmega164pa as hal;
    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: hal::pac::PORTA = [0, 1, 2, 3, 4, 5, 6 ,7],
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6 ,7],
            C: hal::pac::PORTC = [0, 1, 2, 3, 4, 5, 6 ,7],
            D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6 ,7],
                }
    }

    #[macro_export]
    macro_rules! atmega164pa_pins {
        ($p:expr) => {
            $crate::atmega164pa::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
        };
    }

    pub use atmega164pa_pins as pins;
}

impl_mod_simple_pwm! {
    use crate::atmega164pa as hal;

    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC0` for PWM (pins `PB3`)
        ///
        /// # Example
        /// ```
        /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
        ///
        /// let mut b3 = pins.pb3.into_output().into_pwm(&mut timer0);
        ///
        /// b3.set_duty(128);
        /// b3.enable();
        /// ```
        pub struct Timer0Pwm {
            timer: hal::pac::TC0,
            init: |tim, prescaler| {
                tim.tccr0a.modify(|_r, w| w.wgm0().bits(0b11));
                tim.tccr0a.modify(|_r, w| w.com0a().bits(0b00));

                tim.tccr0b.modify(|_r, w| match prescaler {
                    Prescaler::Direct => w.cs0().running_no_prescaling(),
                    Prescaler::Prescale8 => w.cs0().running_clk_8(),
                    Prescaler::Prescale64 => w.cs0().running_clk_64(),
                    Prescaler::Prescale256 => w.cs0().running_clk_256(),
                    Prescaler::Prescale1024 => w.cs0().running_clk_1024(),
                });
            },
            pins: {
                hal::port::PB3: {
                    ocr: ocr0a,
                    into_pwm: |tim| if enable {
                        tim.tccr0a.modify(|_r, w| w.com0a().bits(0b11));
                    } else {
                        tim.tccr0a.modify(|_r, w| w.com0a().bits(0b00));
                    },
                },
            },
        }
    }

    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC1` for PWM (pins `PD4`, `PD5`)
        ///
        /// # Example
        /// ```
        /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
        ///
        /// let mut d4 = pins.pd4.into_output().into_pwm(&mut timer1);
        /// let mut d5 = pins.pd5.into_output().into_pwm(&mut timer1);
        ///
        /// d4.set_duty(128);
        /// d4.enable();
        /// d5.set_duty(64);
        /// d5.enable();
        /// ```
        pub struct Timer1Pwm {
            timer: hal::pac::TC1,
            init: |tim, prescaler| {
                tim.tccr1a.modify(|_r, w| w.wgm1().bits(0b01));
                tim.tccr1a.modify(|_r, w| w.com1a().bits(0b00));
                tim.tccr1a.modify(|_r, w| w.com1b().bits(0b00));
                tim.tccr1b.modify(|_r, w| match prescaler {
                    Prescaler::Direct => w.cs1().running_no_prescaling(),
                    Prescaler::Prescale8 => w.cs1().running_clk_8(),
                    Prescaler::Prescale64 => w.cs1().running_clk_64(),
                    Prescaler::Prescale256 => w.cs1().running_clk_256(),
                    Prescaler::Prescale1024 => w.cs1().running_clk_1024(),
                });
            },
            pins: {
                hal::port::PD4: {
                    ocr: ocr1a,
                    into_pwm: |tim| if enable {
                        tim.tccr1a.modify(|_r, w| w.com1a().bits(0b11));
                    } else {
                        tim.tccr1a.modify(|_r, w| w.com1a().bits(0b00));
                    },
                },
                hal::port::PD5: {
                    ocr: ocr1b,
                    into_pwm: |tim| if enable {
                        tim.tccr1a.modify(|_r, w| w.com1b().bits(0b11));
                    } else {
                        tim.tccr1a.modify(|_r, w| w.com1b().bits(0b00));
                    },
                },
            },
        }
    }
}

impl_mod_usart! {
    use crate::atmega164pa as hal;
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
        rx: hal::port::PD2,
        tx: hal::port::PD3,
        usart_type: Usart1,
    }
}

impl_mod_wdt! {
    use crate::atmega164pa as hal;
    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}

