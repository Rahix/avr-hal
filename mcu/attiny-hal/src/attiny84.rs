pub mod adc {
    pub use crate::periphals::adc::*;

    // Fixme: Implement ADC for ATtiny84.
}

pub mod eeprom {
    pub use crate::periphals::eeprom::*;

    // Fixme: Implement EEPROM for ATtiny84.
}

pub mod port {
    pub use crate::periphals::port::*;

    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
            B: crate::pac::PORTB = [0, 1, 2, 3],
        }
    }
}

pub mod simple_pwm {
    pub use crate::periphals::simple_pwm::*;

    use crate::port::*;

    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC0` for PWM (pins `PB2`, `PA7`)
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
                PB2: {
                    ocr: ocr0a,
                    into_pwm: |tim| if enable {
                        tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                    } else {
                        tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                    },
                },

                PA7: {
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
        /// Use `TC1` for PWM (pins `PA6`, 'PA5')
        pub struct Timer1Pwm {
            timer: crate::pac::TC1,
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
                PA6: {
                    ocr: ocr1a,
                    into_pwm: |tim| if enable {
                        tim.tccr1a.modify(|_, w| w.com1a().bits(0b10));
                    } else {
                        tim.tccr1a.modify(|_, w| w.com1a().disconnected());
                    },
                },

                PA5: {
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
}
