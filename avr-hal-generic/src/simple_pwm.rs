//! PWM Implementation

use core::marker::PhantomData;
use embedded_hal::pwm;
use embedded_hal::pwm::{ErrorKind, ErrorType, SetDutyCycle};

use crate::port::mode;
use crate::port::Pin;

/// Clock prescaler for PWM
///
/// The prescaler dictates the PWM frequency, together with the IO clock.  The formula is as
/// follows:
///
/// ```text
/// F_pwm = CLK_io / (Prescaler * 256);
/// ```
///
/// | Prescaler | 16 MHz Clock | 8 MHz Clock |
/// | --- | --- | ---|
/// | `Direct` | 62.5 kHz | 31.3 kHz |
/// | `Prescale8` | 7.81 kHz | 3.91 kHz |
/// | `Prescale64` | 977 Hz | 488 Hz |
/// | `Prescale256` | 244 Hz | 122 Hz |
/// | `Prescale1024` | 61.0 Hz | 30.5 Hz |
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prescaler {
    /// No prescaling, the IO clock drives the timer directly.
    Direct,
    /// Divide the IO clock by 8.
    Prescale8,
    /// Divide the IO clock by 64.
    Prescale64,
    /// Divide the IO clock by 256.
    Prescale256,
    /// Divide the IO clock by 1024.
    Prescale1024,
}

/// Implement traits and types for PWM timers
pub trait PwmPinOps<TC> {
    type Duty;

    fn enable(&mut self);
    fn disable(&mut self);
    fn get_duty(&self) -> Self::Duty;
    fn get_max_duty(&self) -> Self::Duty;

    fn set_duty(&mut self, value: u8);
}

pub trait IntoPwmPin<TC, PIN> {
    fn into_pwm(self, timer: &TC) -> Pin<mode::PwmOutput<TC>, PIN>;
}

impl<TC, PIN: PwmPinOps<TC>> IntoPwmPin<TC, PIN> for Pin<mode::Output, PIN> {
    fn into_pwm(self, _timer: &TC) -> Pin<mode::PwmOutput<TC>, PIN> {
        Pin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }
}

impl<TC, PIN: PwmPinOps<TC>> Pin<mode::PwmOutput<TC>, PIN> {
    pub fn enable(&mut self) {
        self.pin.enable();
    }

    pub fn disable(&mut self) {
        self.pin.disable();
    }

    pub fn get_duty(&self) -> <PIN as PwmPinOps<TC>>::Duty {
        self.pin.get_duty()
    }

    pub fn get_max_duty(&self) -> <PIN as PwmPinOps<TC>>::Duty {
        self.pin.get_max_duty()
    }

    pub fn set_duty(&mut self, duty: u8) {
        self.pin.set_duty(duty);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PwmError {
    /// `embedded-hal` supports duty cycles up to `u16`, however `avr` devices only support up to `u8`.
    /// Passing a duty cycle larger than [`u8::MAX`] will result in this error.
    DutyCycleTooLarge,
}

impl pwm::Error for PwmError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl<TC, PIN: PwmPinOps<TC>> ErrorType for Pin<mode::PwmOutput<TC>, PIN> {
    type Error = PwmError;
}

impl<TC, PIN: PwmPinOps<TC, Duty = u8>> SetDutyCycle for Pin<mode::PwmOutput<TC>, PIN> {
    fn max_duty_cycle(&self) -> u16 {
        self.get_max_duty() as u16
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        if duty > u8::MAX as u16 {
            return Err(PwmError::DutyCycleTooLarge);
        }
        self.set_duty(duty as u8);
        Ok(())
    }
}

#[macro_export]
macro_rules! impl_simple_pwm {
    (
        $(#[$timer_pwm_attr:meta])*
        pub struct $TimerPwm:ident {
            timer: $TIMER:ty,
            init: |$init_timer:ident, $prescaler:ident| $init_block:block,
            pins: {$(
                $PXi:ident: {
                    ocr: $ocr:ident,
                    $into_pwm:ident: |$pin_timer:ident| if enable
                        $pin_enable_block:block else $pin_disable_block:block,
                },
            )+},
        }
    ) => {
        $(#[$timer_pwm_attr])*
        pub struct $TimerPwm {
            timer: $TIMER,
        }

        impl $TimerPwm {
            pub fn new(timer: $TIMER, prescaler: $crate::simple_pwm::Prescaler) -> $TimerPwm {
                let mut t = $TimerPwm { timer };

                {
                    let $init_timer = &mut t.timer;
                    let $prescaler = prescaler;
                    $init_block
                }

                t
            }
        }

        $(
            impl avr_hal_generic::simple_pwm::PwmPinOps<$TimerPwm> for $PXi {
                type Duty = u8;

                fn enable(&mut self) {
                    // SAFETY: This block will usually result in a read-modify-write sequence which
                    // is not concurrency safe.  Thus, it is wrapped in a critical section which
                    // ensures we will never hit a race-condition here.
                    $crate::avr_device::interrupt::free(|_| {
                        let $pin_timer = unsafe { &*<$TIMER>::ptr() };
                        $pin_enable_block
                    });
                }

                fn disable(&mut self) {
                    // SAFETY: This block will usually result in a read-modify-write sequence which
                    // is not concurrency safe.  Thus, it is wrapped in a critical section which
                    // ensures we will never hit a race-condition here.
                    $crate::avr_device::interrupt::free(|_| {
                        let $pin_timer = unsafe { &*<$TIMER>::ptr() };
                        $pin_disable_block
                    });
                }

                fn get_duty(&self) -> Self::Duty {
                    unsafe { (&*<$TIMER>::ptr()) }.$ocr.read().bits() as Self::Duty
                }

                fn get_max_duty(&self) -> Self::Duty {
                    u8::MAX
                }

                fn set_duty(&mut self, duty: Self::Duty) {
                    // SAFETY: This register is exclusively used here so there are no concurrency
                    // issues.
                    unsafe { (&*<$TIMER>::ptr()).$ocr.write(|w| w.bits(duty.into())); };
                }
            }
        )+
    }
}
