//! Digital IO implementations for the `PORT#` peripherals
//!
//! Please take a look at the documentation for [`Pin`] for a detailed explanation.

use core::marker::PhantomData;
use embedded_hal::digital::v2::{OutputPin, InputPin};

pub trait PinMode: crate::Sealed {}
/// GPIO pin modes
pub mod mode {
    use core::marker::PhantomData;

    pub trait Io: crate::Sealed + super::PinMode {}

    /// Pin is configured as a digital output.
    pub struct Output;
    impl super::PinMode for Output {}
    impl Io for Output {}
    impl crate::Sealed for Output {}

    pub trait InputMode: crate::Sealed {}

    /// Pin is configured as digital input (floating or pulled-up).
    pub struct Input<IMODE = AnyInput> {
        pub(crate) _imode: PhantomData<IMODE>,
    }
    impl<IMODE: InputMode> super::PinMode for Input<IMODE> {}
    impl<IMODE: InputMode> Io for Input<IMODE> {}
    impl<IMODE: InputMode> crate::Sealed for Input<IMODE> {}

    /// Floating input, used like `Input<Floating>`.
    pub struct Floating;
    impl InputMode for Floating {}
    impl crate::Sealed for Floating {}

    /// Pulled-up input, used like `Input<PullUp>`.
    pub struct PullUp;
    impl InputMode for PullUp {}
    impl crate::Sealed for PullUp {}

    /// Any input (floating or pulled-up), used like `Input<AnyInput>`.
    pub struct AnyInput;
    impl InputMode for AnyInput {}
    impl crate::Sealed for AnyInput {}

    /// Pin is configured as an analog input (for the ADC).
    pub struct Analog;
}

pub trait PinOps {
    type Dynamic;

    fn into_dynamic(self) -> Self::Dynamic;

    unsafe fn out_set(&mut self);
    unsafe fn out_clear(&mut self);
    unsafe fn out_toggle(&mut self);
    unsafe fn out_get(&self) -> bool;

    unsafe fn in_get(&self) -> bool;

    unsafe fn make_output(&mut self);
    unsafe fn make_input(&mut self, pull_up: bool);
}

/// Representation of an MCU pin.
///
/// # Design Rationale
/// We want individual types per pin to model constraints which depend on a specific pin.  For
/// example, some peripherals are internally hard-wired to certain pins of the MCU.
///
/// Additionally, the mode of a pin should also be a part of the type to model enforcement of pins
/// being in a certain mode and preventing misuse like for example calling `set_high()` on a pin
/// configured as input.
///
/// To do this, the [`Pin`] type is generic over the `MODE` (input, output, ...) and the `PIN`
/// (pd0, pb5, pc6, ...).
///
/// Of course, in some applications one does not care about the specific pin used.  For these
/// situations, the specific pin types can be "downgraded" into a dynamic type that can represent
/// any pin.  See [Downgrading](#downgrading) for more details.
///
/// # Instantiation
/// The `Peripherals` struct in HAL and board-support crates usually contains a `.pins` field which
/// is of type `Pins`.  This `Pins` struct in turn has fields for each individual pin, in its
/// default mode.  You can then move the pin out of this struct to reconfigure it (examples in this
/// documentation are for `atmega-hal`):
///
/// ```ignore
/// use atmega_hal::port::{Pin, mode, self};
///
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let pins = atmega_hal::pins!(dp);
///
/// let output: Pin<mode::Output, port::PD3> = pins.pd3.into_output();
/// ```
pub struct Pin<MODE, PIN> {
    pub(crate) pin: PIN,
    pub(crate) _mode: PhantomData<MODE>,
}

impl<PIN: PinOps> Pin<mode::Input<mode::Floating>, PIN> {
    #[doc(hidden)]
    pub fn new(pin: PIN) -> Self {
        Pin {
            pin,
            _mode: PhantomData,
        }
    }
}

/// # Configuration
/// To change the mode of a pin, use one of the following conversion functions.  They consume the
/// original [`Pin`] and return one with the desired mode.  Only when a pin is in the correct mode,
/// does it have the mode-relevant methods availailable (e.g. `set_high()` is only available for
/// `Output` pins).
impl<PIN: PinOps, MODE: mode::Io> Pin<MODE, PIN> {
    /// Convert this pin into an output pin, setting the state to low.
    /// See [Digital Output](#digital-output).
    pub fn into_output(mut self) -> Pin<mode::Output, PIN> {
        unsafe { self.pin.out_clear() };
        unsafe { self.pin.make_output() };
        Pin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Convert this pin into an output pin, setting the state to high.
    /// See [Digital Output](#digital-output).
    pub fn into_output_high(mut self) -> Pin<mode::Output, PIN> {
        unsafe { self.pin.out_set() };
        unsafe { self.pin.make_output() };
        Pin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Convert this pin into a floating input pin.  See [Digital Input](#digital-input).
    ///
    /// *Note*: To read deterministic values from the pin, it must be externally pulled to a
    /// defined level (either VCC or GND).
    pub fn into_floating_input(mut self) -> Pin<mode::Input<mode::Floating>, PIN> {
        unsafe { self.pin.make_input(false) };
        Pin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Convert this pin into a pulled-up input pin.  See [Digital Input](#digital-input).
    ///
    /// With no external circuit pulling the pin low, it will be read high.
    pub fn into_pull_up_input(mut self) -> Pin<mode::Input<mode::PullUp>, PIN> {
        unsafe { self.pin.make_input(true) };
        Pin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }

    /// Convert this pin into an analog input (ADC channel).  See [Analog Input](#analog-input).
    ///
    /// Some pins can be repurposed as ADC channels.  For those pins, the `into_analog_input()`
    /// method is available.
    pub fn into_analog_input<H, ADC, CLOCK>(
        self,
        adc: &mut crate::adc::Adc<H, ADC, CLOCK>,
    ) -> Pin<mode::Analog, PIN>
    where
        Pin<mode::Analog, PIN>: crate::adc::AdcChannel<H, ADC>,
        ADC: crate::adc::AdcOps<H>,
        CLOCK: crate::clock::Clock,
    {
        let new = Pin {
            pin: self.pin,
            _mode: PhantomData,
        };
        adc.enable_pin(&new);
        new
    }
}

/// # Downgrading
/// For applications where the exact pin is irrelevant, a specific pin can be downgraded to a
/// "dynamic pin" which can represent any pin:
///
/// ```ignore
/// use atmega_hal::port::{Pin, mode};
///
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let pins = atmega_hal::pins!(dp);
///
/// let any_output_pin1: Pin<mode::Output> = pins.pd0.into_output().downgrade();
/// let any_output_pin2: Pin<mode::Output> = pins.pd1.into_output().downgrade();
///
/// // Because they now have the same type, you can, for example, stuff them into an array:
/// let pins: [Pin<mode::Output>; 2] = [any_output_pin1, any_output_pin2];
/// ```
impl<PIN: PinOps, MODE: mode::Io> Pin<MODE, PIN> {
    /// "Erase" type-level information about which specific pin is represented.
    ///
    /// *Note*: The returned "dynamic" pin has runtime overhead compared to a specific pin.
    pub fn downgrade(self) -> Pin<MODE, PIN::Dynamic> {
        Pin {
            pin: self.pin.into_dynamic(),
            _mode: PhantomData,
        }
    }
}

/// # Input-Mode Downgrading
/// There is a second kind of downgrading: In some cases it is not important whether an input pin
/// is configured as [`mode::PullUp`] or [`mode::Floating`].  For this, you can "forget" the
/// concrete input mode, leaving you with a type that is the same for pull-up or floating inputs:
///
/// ```ignore
/// use atmega_hal::port::{Pin, mode};
///
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let pins = atmega_hal::pins!(dp);
///
/// // This demo uses downgraded pins, but it works just as well
/// // with non-downgraded ones!
/// let input_pin1: Pin<mode::Input<mode::Floating>> = pins.pd0
///     .into_floating_input()
///     .downgrade();
/// let input_pin2: Pin<mode::Input<mode::Floating>> = pins.pd1
///     .into_pull_up_input()
///     .downgrade();
///
/// // With the input mode "forgotten", they have the same type now,
/// // even if electically different.
/// let any_inputs: [Pin<mode::Input>; 2] = [
///     input_pin1.forget_imode(),
///     input_pin2.forget_imode(),
/// ];
/// ```
impl<PIN: PinOps, IMODE> Pin<mode::Input<IMODE>, PIN> {
    /// "Erase" type-level information about whether the pin is currently a pull-up or a floating
    /// input.
    pub fn forget_imode(self) -> Pin<mode::Input, PIN> {
        Pin {
            pin: self.pin,
            _mode: PhantomData,
        }
    }
}

/// # Digital Output
impl<PIN: PinOps> Pin<mode::Output, PIN> {
    /// Set pin high (pull it to supply voltage).
    #[inline]
    pub fn set_high(&mut self) {
        unsafe { self.pin.out_set() }
    }

    /// Set pin low (pull it to GND).
    #[inline]
    pub fn set_low(&mut self) {
        unsafe { self.pin.out_clear() }
    }

    /// Toggle a high pin to low and a low pin to high.
    #[inline]
    pub fn toggle(&mut self) {
        unsafe { self.pin.out_toggle() }
    }

    /// Check whether the pin is set high.
    ///
    /// *Note*: The electrical state of the pin might differ due to external circuitry.
    #[inline]
    pub fn is_set_high(&self) -> bool {
        unsafe { self.pin.out_get() }
    }

    /// Check whether the pin is set low.
    ///
    /// *Note*: The electrical state of the pin might differ due to external circuitry.
    #[inline]
    pub fn is_set_low(&self) -> bool {
        !unsafe { self.pin.out_get() }
    }
}

// Implements OutputPin from embedded-hal to make sure external libraries work
impl<PIN: PinOps> OutputPin for Pin<mode::Output, PIN> {
    type Error = core::convert::Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

// Implements InputPin from embedded-hal to make sure external libraries work
impl<PIN: PinOps, IMODE: mode::InputMode> InputPin for Pin<mode::Input<IMODE>, PIN> {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

/// # Digital Input
impl<PIN: PinOps, IMODE: mode::InputMode> Pin<mode::Input<IMODE>, PIN> {
    /// Check whether the pin is driven high.
    #[inline]
    pub fn is_high(&self) -> bool {
        unsafe { self.pin.in_get() }
    }

    /// Check whether the pin is driven low.
    #[inline]
    pub fn is_low(&self) -> bool {
        !unsafe { self.pin.in_get() }
    }
}

/// # Analog Input
///
/// Some pins can be configured as ADC channels.  For those pins, `analog_read()` can be used to
/// read the voltage.  `analog_read()` corresponds to a blocking ADC read:
///
/// ```
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let pins = atmega_hal::pins!(dp);
/// let mut adc = atmega_hal::Adc::new(dp.ADC, Default::default());
///
/// let a0 = dp.pc0.into_analog_input(&mut adc);
///
/// let voltage = a0.analog_read(&mut adc);
/// // ^- this is equivalent to -v
/// let voltage = adc.read_blocking(&a0);
/// ```
impl<PIN: PinOps> Pin<mode::Analog, PIN> {
    pub fn analog_read<H, ADC, CLOCK>(&self, adc: &mut crate::adc::Adc<H, ADC, CLOCK>) -> u16
    where
        Pin<mode::Analog, PIN>: crate::adc::AdcChannel<H, ADC>,
        ADC: crate::adc::AdcOps<H>,
        CLOCK: crate::clock::Clock,
    {
        adc.read_blocking(self)
    }

    /// Convert this pin into a generic [`Channel`][adc-channel] type.
    ///
    /// The generic channel type can be used to store multiple channels in an array.
    ///
    /// [adc-channel]: crate::adc::Channel
    pub fn into_channel<H, ADC>(self) -> crate::adc::Channel<H, ADC>
    where
        Pin<mode::Analog, PIN>: crate::adc::AdcChannel<H, ADC>,
        ADC: crate::adc::AdcOps<H>,
    {
        crate::adc::Channel::new(self)
    }
}

#[macro_export]
macro_rules! impl_port_traditional {
    (
        enum Ports {
            $($PortName:ident: ($Port:ty, $port_port_reg:ident, $port_pin_reg:ident, $port_ddr_reg:ident),)+
        }

        $(#[$pins_attr:meta])*
        pub struct Pins {
            $($pin:ident: $Pin:ident = ($PinPort:ty, $PinPortName:ident, $pin_num:expr,
                                        $pin_port_reg:ident, $pin_pin_reg:ident,
                                        $pin_ddr_reg:ident),)+
        }
    ) => {
        pub use $crate::port::mode;

        /// Type-alias for a pin type which can represent any concrete pin.
        ///
        /// Sometimes it is easier to handle pins if they are all of the same type.  By default,
        /// each pin gets its own distinct type in `avr-hal`, but by
        /// [downgrading][avr_hal_generic::port::Pin#downgrading], you can cast them into this
        /// "dynamic" type.  Do note, however, that using this dynamic type has a runtime cost.
        pub type Pin<MODE, PIN = Dynamic> = $crate::port::Pin<MODE, PIN>;

        $(#[$pins_attr])*
        pub struct Pins {
            $(pub $pin: Pin<
                mode::Input<mode::Floating>,
                $Pin,
            >,)+
        }

        impl Pins {
            pub fn new(
                $(_: $Port,)+
            ) -> Self {
                Self {
                    $($pin: $crate::port::Pin::new(
                        $Pin { _private: (), }
                    ),)+
                }
            }
        }

        #[repr(u8)]
        pub enum DynamicPort {
            $($PortName,)+
        }

        pub struct Dynamic {
            port: DynamicPort,
            // We'll store the mask instead of the pin number because this allows much less code to
            // be generated for the trait method implementations.
            mask: u8,
        }

        impl Dynamic {
            fn new(port: DynamicPort, pin_num: u8) -> Self {
                Self {
                    port,
                    mask: 1 << pin_num,
                }
            }
        }

        impl $crate::port::PinOps for Dynamic {
            type Dynamic = Self;

            #[inline]
            fn into_dynamic(self) -> Self::Dynamic {
                self
            }

            #[inline]
            unsafe fn out_set(&mut self) {
                match self.port {
                    $(DynamicPort::$PortName => (*<$Port>::ptr()).$port_port_reg.modify(|r, w| {
                        w.bits(r.bits() | self.mask)
                    }),)+
                }
            }

            #[inline]
            unsafe fn out_clear(&mut self) {
                match self.port {
                    $(DynamicPort::$PortName => (*<$Port>::ptr()).$port_port_reg.modify(|r, w| {
                        w.bits(r.bits() & !self.mask)
                    }),)+
                }
            }

            #[inline]
            unsafe fn out_toggle(&mut self) {
                match self.port {
                    $(DynamicPort::$PortName => (*<$Port>::ptr()).$port_pin_reg.modify(|r, w| {
                        w.bits(r.bits() | self.mask)
                    }),)+
                }
            }

            #[inline]
            unsafe fn out_get(&self) -> bool {
                match self.port {
                    $(DynamicPort::$PortName => (*<$Port>::ptr()).$port_port_reg.read().bits()
                        & self.mask != 0,)+
                }
            }

            #[inline]
            unsafe fn in_get(&self) -> bool {
                match self.port {
                    $(DynamicPort::$PortName => (*<$Port>::ptr()).$port_pin_reg.read().bits()
                        & self.mask != 0,)+
                }
            }

            #[inline]
            unsafe fn make_output(&mut self) {
                match self.port {
                    $(DynamicPort::$PortName => (*<$Port>::ptr()).$port_ddr_reg.modify(|r, w| {
                        w.bits(r.bits() | self.mask)
                    }),)+
                }
            }

            #[inline]
            unsafe fn make_input(&mut self, pull_up: bool) {
                match self.port {
                    $(DynamicPort::$PortName => (*<$Port>::ptr()).$port_ddr_reg.modify(|r, w| {
                        w.bits(r.bits() & !self.mask)
                    }),)+
                }
                if pull_up {
                    self.out_clear()
                } else {
                    self.out_set()
                }
            }
        }

        $(
            pub struct $Pin {
                _private: ()
            }

            impl $crate::port::PinOps for $Pin {
                type Dynamic = Dynamic;

                #[inline]
                fn into_dynamic(self) -> Self::Dynamic {
                    Dynamic::new(DynamicPort::$PinPortName, $pin_num)
                }

                #[inline]
                unsafe fn out_set(&mut self) {
                    (*<$PinPort>::ptr()).$pin_port_reg.modify(|r, w| {
                        w.bits(r.bits() | (1 << $pin_num))
                    })
                }

                #[inline]
                unsafe fn out_clear(&mut self) {
                    (*<$PinPort>::ptr()).$pin_port_reg.modify(|r, w| {
                        w.bits(r.bits() & !(1 << $pin_num))
                    })
                }

                #[inline]
                unsafe fn out_toggle(&mut self) {
                    (*<$PinPort>::ptr()).$pin_pin_reg.modify(|r, w| {
                        w.bits(r.bits() | (1 << $pin_num))
                    })
                }

                #[inline]
                unsafe fn out_get(&self) -> bool {
                    (*<$PinPort>::ptr()).$pin_port_reg.read().bits() & (1 << $pin_num) != 0
                }

                #[inline]
                unsafe fn in_get(&self) -> bool {
                    (*<$PinPort>::ptr()).$pin_pin_reg.read().bits() & (1 << $pin_num) != 0
                }

                #[inline]
                unsafe fn make_output(&mut self) {
                    (*<$PinPort>::ptr()).$pin_ddr_reg.modify(|r, w| {
                        w.bits(r.bits() | (1 << $pin_num))
                    })
                }

                #[inline]
                unsafe fn make_input(&mut self, pull_up: bool) {
                    (*<$PinPort>::ptr()).$pin_ddr_reg.modify(|r, w| {
                        w.bits(r.bits() & !(1 << $pin_num))
                    });
                    if pull_up {
                        self.out_set()
                    } else {
                        self.out_clear()
                    }
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! renamed_pins {
    (
        type Pin = $PinType:ident;

        $(#[$pins_attr:meta])*
        pub struct Pins from $McuPins:ty {
            $($(#[$pin_attr:meta])* pub $pin:ident: $Pin:ty = $pin_orig:ident,)+
        }
    ) => {
        $(#[$pins_attr])*
        pub struct Pins {
            $($(#[$pin_attr])*
            pub $pin: $PinType<
                $crate::port::mode::Input<$crate::port::mode::Floating>,
                $Pin,
            >,)+
        }

        impl Pins {
            pub fn with_mcu_pins(pins: $McuPins) -> Self {
                Self {
                    $($pin: pins.$pin_orig,)+
                }
            }
        }
    };
}
