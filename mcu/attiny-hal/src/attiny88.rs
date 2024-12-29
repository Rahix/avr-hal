use crate::r#impl::avr_hal;

avr_hal! {
    device: attiny88,
    eeprom: {
        capacity: 64,
        addr_width: u8,
        addr_reg: eearl,
    },
    port: {
        ports: {
            A: [0, 1, 2, 3],
            B: [0, 1, 2, 3, 4, 5, 6, 7],
            C: [0, 1, 2, 3, 4, 5, 6, 7],
            D: [0, 1, 2, 3, 4, 5, 6, 7],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },
    pwm: {
        impl: {
            pub use avr_hal_generic::simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps};

            avr_hal_generic::impl_simple_pwm! {
                /// Use `TC1` for PWM (pins `PB1`, 'PB2')
                ///
                /// # Example
                /// ```no_run
                /// use attiny_hal::attiny88 as hal;
                /// use hal::simple_pwm::{Timer1Pwm,Prescaler,IntoPwmPin};
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
                    timer: crate::attiny88::pac::TC1,
                    init: |tim, prescaler| {
                        tim.tccr1a.modify(|_, w| w.wgm1().bits(0b01));
                        tim.tccr1b.modify(|_, w| w.wgm1().bits(0b01));

                        tim.tccr1b.modify(|_r, w| match prescaler {
                            Prescaler::Direct => w.cs1().direct(),
                            Prescaler::Prescale8 => w.cs1().prescale_8(),
                            Prescaler::Prescale64 => w.cs1().prescale_64(),
                            Prescaler::Prescale256 => w.cs1().prescale_256(),
                            Prescaler::Prescale1024 => w.cs1().prescale_1024(),
                        });
                    },
                    pins: {
                        hal::port::PB1: {
                            ocr: ocr1a,
                            into_pwm: |tim| if enable {
                                tim.tccr1a.modify(|_, w| w.com1a().bits(0b10));
                            } else {
                                tim.tccr1a.modify(|_, w| w.com1a().disconnected());
                            },
                        },

                        hal::port::PB2: {
                            ocr: ocr1b,
                            into_pwm: |tim| if enable {
                                tim.tccr1a.modify(|_, w| w.com1b().bits(0b10));
                            } else {
                                tim.tccr1a.modify(|_, w| w.com1b().disconnected());
                            },
                        },
                    },
                }
            }
        },
    },
    spi: {
        interfaces: {
            Spi: {
                peripheral: SPI,
                sclk: PB5,
                mosi: PB3,
                miso: PB4,
                cs: PB2,
                impl!: avr_hal_generic::impl_spi,
            },
        },
    },
    adc: {
        references: {
            /// Default reference voltage (default).
            AVcc: |peripheral| {
                peripheral.admux.write(|w| w.refs0().avcc())
            },
            /// Internal 1.1V reference.
            Internal1_1: |peripheral| {
                peripheral.admux.write(|w| w.refs0().internal())
            },
        },
        pins: {
            PC0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            PC1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            PC2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            PC3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            PC4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            PC5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
            PA0: (hal::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            PA1: (hal::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        },
        channels: {
            Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
            Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
        },
    },
    wdt: {
        wdtcsr_name: wdtcsr,
    },
}
