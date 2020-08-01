//! PORTx digital IO Implementations

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
    pub struct Analog;
    /// Pin configured as PWM output
    pub struct Pwm<TIMER> {
        _m: core::marker::PhantomData<TIMER>,
    }

    impl private::Unimplementable for Output {}
    impl<M: InputMode> private::Unimplementable for Input<M> {}
    impl DigitalIO for Output {}
    impl<M: InputMode> DigitalIO for Input<M> {}

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
            $($PortEnum:ident($PORTX:ty, $reg_port:ident, $reg_pin:ident),)+
        }
    ) => {
        mod generic_pin {
            use $crate::hal::digital::v2 as digital;
            use $crate::port::mode;
            use $crate::void::Void;
            use core::marker;

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

            impl digital::toggleable::Default for $GenericPin<mode::Output> {}

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
                /// The main use for this function is to store multiple pins in
                /// an array.  Please note that generic pins have a runtime overhead.
                ///
                /// # Example
                /// ```rust
                /// let p1 = portb.pb1.downgrade();
                /// let p2 = portc.pc7.downgrade();
                /// let pins = [p1, p2];
                /// ```
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

                    /// Make this pin a digital output
                    pub fn into_output<D: AsDDR>(self, ddr: &D) -> $PXi<mode::Output> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        }
                        $PXi { _mode: marker::PhantomData }
                    }

                    /// Make this pin a digital input **without** enabling the internal pull-up
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

                    /// Make this pin a digital input and enable the internal pull-up
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

                impl digital::toggleable::Default for $PXi<mode::Output> {}

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
