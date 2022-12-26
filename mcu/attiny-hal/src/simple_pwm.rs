pub use avr_hal_generic::simple_pwm::{PwmPinOps, Prescaler};

#[cfg(any(feature = "attiny85",feature = "attiny84",feature="attiny88"))]
use crate::port::*;

#[cfg(feature = "attiny84")]
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

#[cfg(feature = "attiny84")]
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

#[cfg(feature = "attiny85")]
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

#[cfg(feature = "attiny88")]
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
            PB1: {
                ocr: ocr1a,
                into_pwm: |tim| if enable {
                    tim.tccr1a.modify(|_, w| w.com1a().bits(0b10));
                } else {
                    tim.tccr1a.modify(|_, w| w.com1a().disconnected());
                },
            },

            PB2: {
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
