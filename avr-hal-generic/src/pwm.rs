//! PWM Implementation

/// Implement traits and types for PWM timers
#[macro_export]
macro_rules! impl_pwm {
    (
        $(#[$timer_pwm_attr:meta])*
        pub struct $TimerPwm:ident {
            timer: $TIMER:ty,
            init: |$init_timer:ident| $init_block:block,
            pins: {$(
                $port:ident::$PXi:ident: {
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
            pub fn new(timer: $TIMER) -> $TimerPwm {
                let mut t = $TimerPwm { timer };

                {
                    let $init_timer = &mut t.timer;
                    $init_block
                }

                t
            }
        }

        $(
            impl $port::$PXi<$crate::port::mode::Output> {
                pub fn $into_pwm(self, pwm_timer: &mut $TimerPwm)
                    -> $port::$PXi<$crate::port::mode::Pwm<$TimerPwm>>
                {
                    $port::$PXi { _mode: core::marker::PhantomData }
                }
            }

            impl $crate::hal::PwmPin for $port::$PXi<$crate::port::mode::Pwm<$TimerPwm>> {
                type Duty = u8;

                fn enable(&mut self) {
                    // SAFETY: This block will usually result in a read-modify-write sequence which
                    // is not concurrency safe.  Thus, it is wrapped in a critical section which
                    // ensures we will never hit a race-condition here.
                    $crate::avr_device::interrupt::free(|_| {
                        let $pin_timer = unsafe { &*<$TIMER>::ptr() };
                        $pin_enable_block
                    })
                }

                fn disable(&mut self) {
                    // SAFETY: This block will usually result in a read-modify-write sequence which
                    // is not concurrency safe.  Thus, it is wrapped in a critical section which
                    // ensures we will never hit a race-condition here.
                    $crate::avr_device::interrupt::free(|_| {
                        let $pin_timer = unsafe { &*<$TIMER>::ptr() };
                        $pin_disable_block
                    })
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
                    unsafe { (&*<$TIMER>::ptr()).$ocr.write(|w| w.bits(duty.into())) };
                }
            }
        )+
    }
}
