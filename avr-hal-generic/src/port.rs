//! PORTx digital IO Implementations
//!
//! # Design Rationale
//! Each pin has a distinct type which allows pin-dependent HAL functionality to ensure at
//! compile-time that the correct pins are used.  For example, certain peripherals have the IO
//! hardwired to some specific pins which can't be changed.  For purposes where the exact pin does
//! not matter, the distinct types can be 'downgraded' into a generic `Pin<MODE>` type.  See the
//! section about [downgrading](#downgrading) further down.
//!
//! To instanciate the pin types, a port is `.split()` into its pins:
//!
//! ```ignore
//! let dp = atmega32u4::Peripherals::take().unwrap();
//!
//! let mut portd = dp.PORTD.split();
//!
//! let pd2 = portd.pd2.into_output(&mut portd.ddr);
//! ```
//!
//! Board crates usually provide a wrapper around that which makes access more convenient:
//!
//! ```ignore
//! let dp = arduino_leonardo::Peripherals::take().unwrap();
//!
//! let mut pins = arduino_leonardo::Pins::new(
//!     dp.PORTB,
//!     dp.PORTC,
//!     dp.PORTD,
//!     dp.PORTE,
//! );
//!
//! let mut led0 = pins.led_rx.into_output(&mut pins.ddr);
//! let mut led1 = pins.led_tx.into_output(&mut pins.ddr);
//! let mut led2 = pins.d13.into_output(&mut pins.ddr);
//! ```
//!
//! # Modes
//! A pin's mode is modelled via the `<MODE>` generic parameter.  Only when the pin is in the
//! correct mode, relevant methods (e.g. `set_high()`) are available.  Changing the mode is done
//! via conversion methods that consume the pin:
//!
//! ```ignore
//! // By default, pins are floating inputs
//! let pd2: PD2<mode::Input<mode::Floating>> = portd.pd2;
//!
//! // Convert into pull-up input
//! let pd2: PD2<mode::Input<mode::PullUp>> = pd2.into_pull_up_input(&mut portd.ddr);
//!
//! // Convert into output
//! let pd2: PD2<mode::Output> = pd2.into_output(&mut portd.ddr);
//!
//! // Convert into tri-state input and output.
//! let pd2: PD2<mode::TriState> = pd2.into_tri_state(&mut portd.ddr);
//! ```
//!
//! ### Digital Input
//! Digital Input pins (i.e. where `MODE` = `mode::Input<_>`) have the following methods available:
//!
//! ```ignore
//! // `true` if the pin is high, `false` if it is low
//! pd2.is_high().void_unwrap();
//!
//! // `true if the pin is low, `false` if it is high
//! pd2.is_low().void_unwrap();
//! ```
//!
//! ### Digital Output
//! Digital Output pins (i.e. where `MODE` = `mode::Output`) can be used like this:
//!
//! ```ignore
//! // Set high or low
//! pd2.set_high().void_unwrap();
//! pd2.set_low().void_unwrap();
//!
//! // Check what the pin was last set to
//! pd2.is_set_high().void_unwrap();
//! pd2.is_set_low().void_unwrap();
//! ```
//!
//! ### Digital Tri-State Output and Input
//! Digital I/O pins in tri-state mode (i.e. where `MODE` = `mode::TriState`),
//! usually with an external pull-up, are useful for a one wire bus.
//! They can be used as both output and input pins, like this:
//!
//! ```ignore
//! // Actively drive the pin low.
//! pd2.set_low().void_unwrap();
//! // Release the pin, allowing the external pull-up to pull the pin
//! // in the absence of another driver
//! pd2.set_high().void_unwrap();
//!
//! // `true` if the pin is electrically high, `false` if it is low
//! pd2.is_high().void_unwrap();
//! // `true` if the pin is electrically low, driven by either the
//! // microcontroller or externally
//! pd2.is_low().void_unwrap();
//! ```
//!
//! ### Other Modes
//! Apart from input and output, certain pins can have other functionality associated with them.
//! E.g. some pins can be used for PWM output, others as ADC inputs.  For those pins, specific
//! conversion methods exist:
//!
//! ```ignore
//! // Digital IO by default
//! let pd2 = portd.pd2;
//!
//! // Make a pin an ADC channel
//! let pd2_analog = pd2.into_analog_input(&mut adc);
//!
//! // Make a pin a PWM output
//! let pd2_pwm = pd2.into_output().into_pwm(&mut timer0);
//! ```
//!
//! ## Downgrading
//! As described above, usually each pin has its own distinct type.  This is useful in a lot of
//! cases but can be difficult to deal with when code does not care about the exact pin(s) it is
//! working with.  An easy example is trying to store a number of pins in an array; this is not
//! possible when each pin has its own type.
//!
//! For those usecases, a generic `Pin` type exists which can represent any pin.  Specific pins are
//! converted using the `.downgrade()` method:
//!
//! ```ignore
//! let pd2 = portd.pd2.into_output(&mut portd.ddr);
//! let pd3 = portd.pd3.into_output(&mut portd.ddr);
//!
//! let pins: [Pin<mode::Output>; 2] = [pd2.downgrade(), pd3.downgrade()];
//! ```

/// IO Modes
pub mod mode {
    /// Any digital IO mode
    pub trait DigitalIO: private::Unimplementable {}
    /// Any input mode
    pub trait InputMode: private::Unimplementable {}

    /// Pin configured as a digital input
    pub struct Input<MODE: InputMode> {
        _m: core::marker::PhantomData<MODE>,
    }
    /// Pin configured as a digital output
    pub struct Output;
    /// Pin configured as an ADC channel
    pub struct Analog;
    /// Pin configured as PWM output
    pub struct Pwm<TIMER> {
        _m: core::marker::PhantomData<TIMER>,
    }
    /// Pin configured in open drain mode.
    pub struct TriState;

    impl private::Unimplementable for Output {}
    impl<M: InputMode> private::Unimplementable for Input<M> {}
    impl private::Unimplementable for TriState {}
    impl DigitalIO for Output {}
    impl<M: InputMode> DigitalIO for Input<M> {}
    impl DigitalIO for TriState {}

    /// Pin input configured **without** internal pull-up
    pub struct Floating;
    /// Pin input configured with internal pull-up
    pub struct PullUp;

    impl private::Unimplementable for Floating {}
    impl private::Unimplementable for PullUp {}
    impl InputMode for Floating {}
    impl InputMode for PullUp {}

    mod private {
        pub trait Unimplementable {}
    }
}

/// Create a generic pin to be used for downgrading
#[macro_export]
macro_rules! impl_generic_pin {
    (
        pub enum $GenericPin:ident {
            $($PortEnum:ident($PORTX:ty, $reg_port:ident, $reg_pin:ident, $reg_ddr:ident),)+
        }
    ) => {
        mod generic_pin {
            use $crate::hal::digital::v2 as digital;
            use $crate::port::mode;
            use $crate::void::Void;
            use core::marker;

            /// Generic pin type.
            ///
            /// As described in the [general Digital IO documentation][1], this type can represent
            /// any pin for use-cases where the exact pin is not relevant.  This is especially
            /// useful when, e.g. wanting to store a number of pins in an array.
            ///
            /// The generic pin implements all the same digital IO methods (`set_high()`,
            /// `set_low()`, `is_high()`, `is_low()`, etc.) as specific pin types, except that it
            /// cannot be used for pin-specific functions (e.g. PWM, ADC Channel).
            ///
            /// [1]: ../../avr_hal_generic/port/index.html
            pub enum $GenericPin<MODE> {
                $($PortEnum(u8, marker::PhantomData<MODE>),)+
            }

            // Input & Output implementations ------------------------- {{{
            // - Unsafe:  The unsafe blocks in here are ok, because these
            //            operations will compile down to single, atomic
            //            `sbi`, `cbi`, `sbic`, `sbis` instructions.
            impl digital::OutputPin for $GenericPin<mode::Output> {
                type Error = Void;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr())
                                    .$reg_port
                                    .modify(|r, w| {
                                        w.bits(r.bits() | (1 << *i))
                                    })
                            },
                        )+
                    }
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr())
                                    .$reg_port
                                    .modify(|r, w| {
                                        w.bits(r.bits() & !(1 << *i))
                                    })
                            },
                        )+
                    }
                    Ok(())
                }
            }

            impl digital::StatefulOutputPin for $GenericPin<mode::Output> {
                fn is_set_high(&self) -> Result<bool, Self::Error> {
                    Ok(match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr())
                                    .$reg_port.read().bits() & (1 << *i) != 0
                            },
                        )+
                    })
                }

                fn is_set_low(&self) -> Result<bool, Self::Error> {
                    self.is_set_high().map(|b| !b)
                }
            }

            impl digital::ToggleableOutputPin for $GenericPin<mode::Output> {
                type Error = Void;

                fn toggle(&mut self) -> Result<(), Self::Error> {
                    match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr())
                                    .$reg_pin
                                    .write(|w| w.bits(1 << *i))
                            },
                        )+
                    }
                    Ok(())
                }
            }

            impl<MODE: mode::InputMode> digital::InputPin for $GenericPin<mode::Input<MODE>> {
                type Error = Void;

                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok(match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr())
                                    .$reg_pin.read().bits() & (1 << *i) != 0
                            }
                        )+
                    })
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr())
                                    .$reg_pin.read().bits() & (1 << *i) == 0
                            }
                        )+
                    })
                }
            }

            impl digital::OutputPin for $GenericPin<mode::TriState> {
                type Error = Void;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                        w.bits(r.bits() & !(1 << *i))
                                    })
                            },
                        )+
                    }
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                        w.bits(r.bits() | (1 << *i))
                                    })
                            },
                        )+
                    }
                    Ok(())
                }
            }

            impl digital::InputPin for $GenericPin<mode::TriState> {
                type Error = Void;

                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok(match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr())
                                    .$reg_pin.read().bits() & (1 << *i) != 0
                            }
                        )+
                    })
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(match self {
                        $(
                            $GenericPin::$PortEnum(i, _) => unsafe {
                                (*<$PORTX>::ptr())
                                    .$reg_pin.read().bits() & (1 << *i) == 0
                            }
                        )+
                    })
                }
            }
            // -------------------------------------------------------- }}}
        }
        pub use self::generic_pin::$GenericPin;
    };
}

/// Implement pin abstractions for a port peripheral
#[macro_export]
macro_rules! impl_port {
    // With a generic pin
    (
        pub mod $portx:ident {
            #[port_ext]
            use $portext_use:path;

            #[generic_pin]
            use $GenericPin:ident::$PortEnum:ident;

            impl $PortExt:ident for $PORTX:ty {
                regs: ($reg_pin:ident, $reg_ddr:ident, $reg_port:ident),
                $($pxi:ident: ($PXi:ident, $i:expr),)+
            }
        }
    ) => {
        $crate::impl_port! {
            pub mod $portx {
                #[port_ext]
                use $portext_use;

                impl $PortExt for $PORTX {
                    regs: ($reg_pin, $reg_ddr, $reg_port),
                    $($pxi: ($PXi, $i),)+
                }
            }
        }

        // Downgrade implementation ------------------------------- {{{
        $(
            impl<MODE> $portx::$PXi<MODE> {
                /// Downgrade this pin into a type that is generic over all pins.
                ///
                /// The main use for this function is to store multiple pins in an array.  Please
                /// note that generic pins have a runtime overhead.
                ///
                /// See the [general Digital IO documentation][1] and the [`Pin` type][2] for more
                /// information.
                ///
                /// # Example
                /// ```rust
                /// let p1 = portb.pb1.downgrade();
                /// let p2 = portc.pc7.downgrade();
                /// let pins = [p1, p2];
                /// ```
                ///
                /// [1]: ../../../avr_hal_generic/port/index.html
                /// [2]: ../enum.Pin.html
                pub fn downgrade(self) -> $GenericPin<MODE> {
                    $GenericPin::$PortEnum($i, ::core::marker::PhantomData)
                }
            }
        )+
        // -------------------------------------------------------- }}}
    };
    // Without a generic pin
    (
        pub mod $portx:ident {
            #[port_ext]
            use $portext_use:path;

            impl $PortExt:ident for $PORTX:ty {
                regs: ($reg_pin:ident, $reg_ddr:ident, $reg_port:ident),
                $($pxi:ident: ($PXi:ident, $i:expr),)+
            }
        }
    ) => {
        pub mod $portx {
            use core::marker;
            use $crate::void::Void;
            use $crate::hal::digital::v2 as digital;
            use $crate::port::mode;

            // We have to "use" the port-ext trait so we can implement it inside
            // the macro.
            use $portext_use;

            pub struct Parts {
                pub ddr: DDR,
                $(
                    pub $pxi: $PXi<mode::Input<mode::Floating>>,
                )+
            }

            impl $PortExt for $PORTX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        ddr: DDR { _0: () },
                        $($pxi: $PXi { _mode: marker::PhantomData },)+
                    }
                }
            }

            /// Marker trait for types that can be used as DDR
            pub trait AsDDR {
                fn as_ddr(&self) -> &DDR;
            }

            pub struct DDR {
                _0: (),
            }

            impl AsDDR for DDR {
                fn as_ddr(&self) -> &DDR { self }
            }

            $(
                /// Type representing a specific pin.
                ///
                /// See the [general Digital IO documentation][1] for more info.  In short:
                ///
                /// - Mode is changed with
                ///   - `.into_output()`
                ///   - `.into_floating_input()`
                ///   - `.into_pull_up_input()`
                /// - Input pins are sampled with
                ///   - `.is_high().void_unwrap()`
                ///   - `.is_low().void_unwrap()`
                /// - Output pins are set with
                ///   - `.set_high().void_unwrap()`
                ///   - `.set_low().void_unwrap()`
                ///
                ///   and can be checked with
                ///   - `.is_set_high().void_unwrap()`
                ///   - `.is_set_low().void_unwrap()`
                /// - Pins can be downgraded into a generic type using `.downgrade()`.
                ///
                /// [1]: ../../../avr_hal_generic/port/index.html
                pub struct $PXi<MODE> {
                    pub(crate)_mode: marker::PhantomData<MODE>,
                }

                // Mode Switch implementations ---------------------------- {{{
                // - Unsafe:  The unsafe blocks in here are ok, because these
                //            operations will compile down to single, atomic
                //            `sbi`, `cbi`, `sbic`, `sbis` instructions.
                impl<MODE: mode::DigitalIO> $PXi<MODE> {
                    // The following methods are only defined if the pin is in
                    // a digital-io mode.  This ensures that a pin used by another
                    // peripheral can't be converted back (because that would
                    // not be universally possible).

                    /// Make this pin a digital output.
                    pub fn into_output<D: AsDDR>(self, ddr: &D) -> $PXi<mode::Output> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        }
                        $PXi { _mode: marker::PhantomData }
                    }

                    /// Make this pin a digital input **without** enabling the internal pull-up.
                    pub fn into_floating_input<D: AsDDR>(self, ddr: &D) -> $PXi<mode::Input<mode::Floating>> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            (*<$PORTX>::ptr()).$reg_port.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                        }
                        $PXi { _mode: marker::PhantomData }
                    }

                    /// Make this pin a digital input and enable the internal pull-up.
                    pub fn into_pull_up_input<D: AsDDR>(self, ddr: &D) -> $PXi<mode::Input<mode::PullUp>> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            (*<$PORTX>::ptr()).$reg_port.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        }
                        $PXi { _mode: marker::PhantomData }
                    }

                    /// Make this pin a tri-state pin. Default state is released (high) mode.
                    /// Internal pull-up is not used.
                    ///
                    /// Note that, as always, it is ***not safe*** to connect the external
                    /// pull-up to a voltage higher than VCC + 0.5.  See your chip's
                    /// datasheet for more details.
                    pub fn into_tri_state<D: AsDDR>(self, ddr: &D) -> $PXi<mode::TriState> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                            (*<$PORTX>::ptr()).$reg_port.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                        }
                        $PXi { _mode: marker::PhantomData }
                    }
                }
                // -------------------------------------------------------- }}}

                // Input & Output implementations ------------------------- {{{
                // - Unsafe:  The unsafe blocks in here are ok, because these
                //            operations will compile down to single, atomic
                //            `sbi`, `cbi`, `sbic`, `sbis` instructions.
                impl digital::OutputPin for $PXi<mode::Output> {
                    type Error = Void;

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_port.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        }
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_port.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                        }
                        Ok(())
                    }
                }

                impl digital::StatefulOutputPin for $PXi<mode::Output> {
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe {
                            (*<$PORTX>::ptr()).$reg_port.read().bits()
                        } & (1 << $i) != 0)
                    }

                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        self.is_set_high().map(|b| !b)
                    }
                }

                impl digital::ToggleableOutputPin for $PXi<mode::Output> {
                    type Error = Void;

                    fn toggle(&mut self) -> Result<(), Self::Error> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_pin.write(|w| {
                                w.bits(1 << $i)
                            });
                        }
                        Ok(())
                    }
                }

                impl<MODE: mode::InputMode> digital::InputPin for $PXi<mode::Input<MODE>> {
                    type Error = Void;

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe {
                            (*<$PORTX>::ptr()).$reg_pin.read().bits()
                        } & (1 << $i) != 0)
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe {
                            (*<$PORTX>::ptr()).$reg_pin.read().bits()
                        } & (1 << $i) == 0)
                    }
                }

                impl digital::OutputPin for $PXi<mode::TriState> {
                    type Error = Void;

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                w.bits(r.bits() & !(1 << $i))
                            });
                        }
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        }
                        Ok(())
                    }
                }

                impl digital::InputPin for $PXi<mode::TriState> {
                    type Error = Void;

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe {
                            (*<$PORTX>::ptr()).$reg_pin.read().bits()
                        } & (1 << $i) != 0)
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe {
                            (*<$PORTX>::ptr()).$reg_pin.read().bits()
                        } & (1 << $i) == 0)
                    }
                }

                // -------------------------------------------------------- }}}
            )+
        }
    };
}

/// Create a pin reexport struct for convenient access
#[macro_export]
macro_rules! impl_board_pins {
    (
        #[port_defs]
        use $portpath:path;

        $(#[$ddr_attr:meta])*
        pub struct $DDR:ident {
            $($portx:ident: $PORTX:ty,)+
        }

        $(#[$pins_attr:meta])*
        pub struct $Pins:ident {
            $(
                $(#[$pin_attr:meta])*
                pub $name:ident: $pinport:ident::$pin:ident::$Pin:ident,
            )+
        }
    ) => {
        use $portpath::{$($portx),+};

        $(#[$ddr_attr])*
        pub struct $DDR {
            $($portx: $portx::DDR,)+
        }

        $(
            impl $portx::AsDDR for $DDR {
                fn as_ddr(&self) -> &$portx::DDR {
                    &self.$portx
                }
            }
        )+

        $(#[$pins_attr])*
        pub struct $Pins {
            pub ddr: $DDR,
            $(
                $(#[$pin_attr])*
                pub $name: $pinport::$Pin<
                    $crate::port::mode::Input<$crate::port::mode::Floating>
                >,
            )+
        }

        impl $Pins {
            pub fn new($($portx: $PORTX),+) -> $Pins {
                $(let $portx = $portx.split();)+

                $Pins {
                    ddr: $DDR {
                        $($portx: $portx.ddr,)+
                    },
                    $($name: $pinport.$pin,)+
                }
            }
        }
    };
}
