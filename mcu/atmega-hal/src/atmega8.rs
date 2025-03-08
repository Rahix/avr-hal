pub use avr_device::atmega8 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega8,
    pins: {
        PC0: (hal::pac::adc::admux::MUX_A::ADC0),
        PC1: (hal::pac::adc::admux::MUX_A::ADC1),
        PC2: (hal::pac::adc::admux::MUX_A::ADC2),
        PC3: (hal::pac::adc::admux::MUX_A::ADC3),
        PC4: (hal::pac::adc::admux::MUX_A::ADC4),
        PC5: (hal::pac::adc::admux::MUX_A::ADC5),
    },
    channels: {
        #[cfg(feature = "enable-extra-adc")]
        ADC6: hal::pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        ADC7: hal::pac::adc::admux::MUX_A::ADC7,
        Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
        Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
    },
    impl!: impl_adc_admux,
}

impl_mod_eeprom! {
    hal: crate::atmega8,
    capacity: 512,
    addr_width: u16,
    addr_reg: eear,
    impl!: avr_hal_generic::impl_eeprom_atmega_old,
}

impl_mod_i2c! {
    hal: crate::atmega8,
    interfaces: {
        I2c: {
            peripheral: TWI,
            sda: PC4,
            scl: PC5,
        },
    },
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
    hal: crate::atmega8,
    impl: {
        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC1` for PWM (pins `PB1`, `PB2`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega8 as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer1Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
            ///
            /// let mut pb1 = pins.pb1.into_output().into_pwm(&mut timer1);
            /// let mut pb2 = pins.pb2.into_output().into_pwm(&mut timer1);
            ///
            /// pb1.set_duty(128);
            /// pb1.enable();
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
            /// Use `TC2` for PWM (pin `PB3`)
            ///
            /// # Example
            /// ```no_run
            /// use atmega_hal::atmega8 as hal;
            /// use hal::simple_pwm::{IntoPwmPin,Timer2Pwm,Prescaler};
            /// 
            /// let dp = hal::Peripherals::take().unwrap();
            /// let pins = hal::pins!(dp);
            /// let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
            ///
            /// let mut pb3 = pins.pb3.into_output().into_pwm(&mut timer2);
            ///
            /// pb3.set_duty(128);
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
    },
}

impl_mod_spi! {
    hal: crate::atmega8,
    interfaces: {
        Spi: {
            peripheral: SPI,
            sclk: PB5,
            mosi: PB3,
            miso: PB4,
            cs: PB2,
        },
    },
}

impl_mod_usart! {
    hal: crate::atmega8,
    interfaces: {
        Usart0: {
            peripheral: USART,
            rx: PD0,
            tx: PD1,
            impl!: crate::r#impl::impl_usart_ubrrh_ucsrc,
        },
    },
}

impl_mod_wdt! {
    use crate::atmega8 as hal;
    impl_wdt_peripheral_ms2000! {
        mcusr: hal::pac::cpu::MCUCSR,
        wdtcsr_name: wdtcr,
    }
}