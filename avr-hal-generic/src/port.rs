pub mod mode {
    pub trait DigitalIO: private::Unimplementable {}
    pub trait InputMode: private::Unimplementable {}

    pub struct Input<MODE: InputMode> {
        _m: core::marker::PhantomData<MODE>,
    }
    pub struct Output;

    impl private::Unimplementable for Output {}
    impl<M: InputMode> private::Unimplementable for Input<M> {}
    impl DigitalIO for Output {}
    impl<M: InputMode> DigitalIO for Input<M> {}

    pub struct Floating;
    pub struct PullUp;

    impl private::Unimplementable for Floating {}
    impl private::Unimplementable for PullUp {}
    impl InputMode for Floating {}
    impl InputMode for PullUp {}

    mod private {
        pub trait Unimplementable {}
    }
}

/// Implement pin abstractions for a port peripheral
#[macro_export]
macro_rules! port_impl {
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
                    _mode: marker::PhantomData<MODE>,
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

                    pub fn into_output<D: AsDDR>(self, ddr: &D) -> $PXi<mode::Output> {
                        unsafe {
                            (*<$PORTX>::ptr()).$reg_ddr.modify(|r, w| {
                                w.bits(r.bits() | (1 << $i))
                            });
                        }
                        $PXi { _mode: marker::PhantomData }
                    }

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
                        Ok(unsafe {
                            (*<$PORTX>::ptr()).$reg_port.read().bits()
                        } & (1 << $i) == 0)
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
    }
}
