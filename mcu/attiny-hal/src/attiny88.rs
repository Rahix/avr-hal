pub use avr_device::attiny88 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::attiny88,
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
}

impl_mod_eeprom! {
    hal: crate::attiny88,
    capacity: 64,
    addr_width: u8,
    addr_reg: eearl,
}

impl_mod_port! {
    use crate::attiny88 as hal;

    pub use avr_hal_generic::port::{mode, PinMode, PinOps};
    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: hal::pac::PORTA = [0, 1, 2, 3],
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            C: hal::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
            D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
        }
    }

    #[macro_export]
    macro_rules! attiny88_pins {
        ($p:expr) => {
            $crate::attiny88::port::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
        };
    }

    pub use attiny88_pins as pins;
}

impl_mod_simple_pwm! {
    hal: crate::attiny88,
    impl: {
        pub use avr_hal_generic::simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps};
    
        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC1` for PWM (pins `PB1`, 'PB2')
            ///
            /// # Example
            /// ```
            /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
            ///
            /// let mut d9 = pins.d9.into_output().into_pwm(&mut timer1);
            /// let mut d10 = pins.d10.into_output().into_pwm(&mut timer1);
            ///
            /// d9.set_duty(128);
            /// d9.enable();
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
}

impl_mod_spi! {
    hal: crate::attiny88,
    sclk: PB5,
    mosi: PB3,
    miso: PB4,
    cs: PB2,
}

impl_mod_wdt! {
    hal: crate::attiny88,
    wdtcsr_name: wdtcsr,
}

