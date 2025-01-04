pub use avr_device::atmega164pa as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega164pa,
    pins: {
        PA0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        PA1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        PA2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        PA3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        PA4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        PA5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
        PA6: (hal::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
        PA7: (hal::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
    },
    channels: {
        Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
        Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
    },
    impl!: impl_adc_admux,
}

impl_mod_eeprom! {
    hal: crate::atmega164pa,
    capacity: 512,
    addr_width: u16,
    addr_reg: eear,
    impl!: avr_hal_generic::impl_eeprom_atmega,
}

impl_mod_i2c! {
    hal: crate::atmega164pa,
    interfaces: {
        I2c: {
            peripheral: TWI,
            sda: PC1,
            scl: PC0,
        },
    },
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
    hal: crate::atmega164pa,
    impl: {

        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC0` for PWM (pins `PB3`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega164pa as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer0Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
            ///
            /// let mut pb3 = pins.pb3.into_output().into_pwm(&mut timer0);
            ///
            /// pb3.set_duty(128);
            /// pb3.enable();
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
            /// ```no_run
            /// use atmega_hal::atmega164pa as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer1Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
            ///
            /// let mut pd4 = pins.pd4.into_output().into_pwm(&mut timer1);
            /// let mut pd5 = pins.pd5.into_output().into_pwm(&mut timer1);
            ///
            /// pd4.set_duty(128);
            /// pd4.enable();
            /// pd5.set_duty(64);
            /// pd5.enable();
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
    },
}

impl_mod_usart! {
    hal: crate::atmega164pa,
    interfaces: {
        Usart0: {
            peripheral: USART0,
            rx: PD0,
            tx: PD1,
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
    },
}

impl_mod_wdt! {
    use crate::atmega164pa as hal;
    impl_wdt_peripheral_ms8000! {
        mcusr: hal::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
    }
}

