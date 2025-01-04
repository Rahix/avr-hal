pub use avr_device::attiny84 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_eeprom! {
    hal: crate::attiny84,
    capacity: 512,
    addr_width: u16,
    addr_reg: eear,
}

impl_mod_port! {
    use crate::attiny84 as hal;

    pub use avr_hal_generic::port::{mode, PinMode, PinOps};
    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: hal::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
            B: hal::pac::PORTB = [0, 1, 2, 3],
        }
    }

    #[macro_export]
    macro_rules! attiny84_pins {
        ($p:expr) => {
            $crate::attiny84::port::Pins::new($p.PORTA, $p.PORTB)
        };
    }

    pub use attiny84_pins as pins;
}

impl_mod_simple_pwm! {
    hal: crate::attiny84,
    impl: {
        pub use avr_hal_generic::simple_pwm::{IntoPwmPin, Prescaler, PwmPinOps};
    
        avr_hal_generic::impl_simple_pwm! {
            /// Use `TC0` for PWM (pins `PB2`, `PA7`)
            pub struct Timer0Pwm {
                timer: crate::attiny84::pac::TC0,
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
                    hal::port::PB2: {
                        ocr: ocr0a,
                        into_pwm: |tim| if enable {
                            tim.tccr0a.modify(|_r, w| w.com0a().match_clear());
                        } else {
                            tim.tccr0a.modify(|_r, w| w.com0a().disconnected());
                        },
                    },
    
                    hal::port::PA7: {
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
                timer: crate::attiny84::pac::TC1,
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
                    hal::port::PA6: {
                        ocr: ocr1a,
                        into_pwm: |tim| if enable {
                            tim.tccr1a.modify(|_, w| w.com1a().bits(0b10));
                        } else {
                            tim.tccr1a.modify(|_, w| w.com1a().disconnected());
                        },
                    },
    
                    hal::port::PA5: {
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

impl_mod_wdt! {
    hal: crate::attiny84,
    wdtcsr_name: wdtcsr,
}

