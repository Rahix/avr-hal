//! Analog-to-Digial Converter (ADC)
//!
//! # Basic information
//!
//! The AVR chips have ADCs, which allow the CPU to acquire information about
//! the "intensity" of a signal, in this case via voltage measurement. This is
//! in contrast to a digital input, which only acquires information about
//! whether there is signal or not.
//!
//! To do this, the converter has circuitry to transform this single continuous
//! signal into multiple discrete signals which the CPU can understand. These
//! signals map to increasing digits of a binary integer, and interpolating this
//! up-counter to a known scale yields "analog" information the chip can use.
//!
//! # Advanced information
//!
//! Due to size and resource constraints, some complexities are introduced to
//! aliviate issues in manufacturing. In this case, there's only one CPU
//! register to read all conversions, and the various ADC channels are
//! multiplexed to it via selection registers. Also, each channel can read from
//! any of a few pin choices, again via multiplexing with a selector.
//! Furthermore, we can choose to measure voltage with respect to references
//! other than the system GND. This module accounts for all these choices
//! statically with optional dynamic casting.

use core::marker::PhantomData;

/// The division factor between the system clock frequency and the input clock to the AD converter.
///
/// To get 10-bit precision, clock from 50kHz to 200kHz must be supplied.  If you need less
/// precision, you can supply a higher clock.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockDivider {
    Factor2,
    Factor4,
    Factor8,
    Factor16,
    Factor32,
    Factor64,
    #[default]
    Factor128,
}

/// Internal trait for the low-level ADC peripheral.
///
/// **Prefer using the [`Adc`] API instead of this trait.**
pub trait AdcOps<H> {
    /// Channel ID type for this ADC.
    type Channel: PartialEq + Copy;

    /// Settings type for this ADC.
    type Settings: PartialEq + Copy;

    /// Initialize the ADC peripheral with the specified settings.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_init(&mut self, settings: Self::Settings);

    /// Read out the ADC data register.
    ///
    /// This method must only be called after a conversion completed.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_read_adc(&self) -> u16;

    /// Check whether the ADC is currently converting a signal.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_is_converting(&self) -> bool;

    /// Start a conversion on the currently selected channel.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_start_conversion(&mut self);

    /// Set the multiplexer to a certain channel.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_set_channel(&mut self, channel: Self::Channel);

    /// Set the DIDR (Digital Input Disable) for a certain channel.
    ///
    /// This disabled digital logic on the corresponding pin and allows measuring analog signals.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_enable_channel(&mut self, channel: Self::Channel);

    /// Clear the DIDR (Digital Input Disable) for a certain channel.
    ///
    /// Enables digital logic on the corresponding pin after it has been used as an ADC channel.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_disable_channel(&mut self, channel: Self::Channel);
}

/// Trait marking a type as an ADC channel for a certain ADC.
pub trait AdcChannel<H, ADC: AdcOps<H>> {
    fn channel(&self) -> ADC::Channel;
}

/// Representation of any ADC Channel.
///
/// Typically, distinct types are used per channel, like for example `Pin<mode::Analog, PC0>`.  In
/// some situations, however, a type is needed which can represent _any_ channel.  This is required
/// to, for example, store multiple channels in an array.
///
/// `Channel` is such a type.  It can be created by calling the [`into_channel()`][into-channel]
/// method of a distinct type:
///
/// ```
/// let a0 = pins.a0.into_analog_input(&mut adc);
/// let a1 = pins.a1.into_analog_input(&mut adc);
///
/// let channels: [atmega_hal::adc::Channel; 2] = [
///     a0.into_channel(),
///     a1.into_channel(),
/// ];
///
/// for ch in channels.iter() {
///     adc.read_blocking(ch);
/// }
/// ```
///
/// [into-channel]: crate::port::Pin::into_channel
pub struct Channel<H, ADC: AdcOps<H>> {
    ch: ADC::Channel,
    _h: PhantomData<H>,
}

impl<H, ADC: AdcOps<H>> Channel<H, ADC> {
    pub fn new<CH: AdcChannel<H, ADC>>(ch: CH) -> Self {
        Self {
            ch: ch.channel(),
            _h: PhantomData,
        }
    }
}

impl<H, ADC: AdcOps<H>> AdcChannel<H, ADC> for Channel<H, ADC> {
    #[inline]
    fn channel(&self) -> ADC::Channel {
        self.ch
    }
}

/// Analog-to-Digital Converter
/// ```
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let pins = atmega_hal::pins!(dp);
/// let mut adc = atmega_hal::Adc::new(dp.ADC, Default::default());
///
/// let a0 = pins.pc0.into_analog_input(&mut adc);
///
/// // the following two calls are equivalent
/// let voltage = a0.analog_read(&mut adc);
/// let voltage = adc.read_blocking(&a0);
///
/// // alternatively, a non-blocking interface exists
/// let voltage = nb::block!(adc.read_nonblocking(&a0)).unwrap_infallible();
/// ```
pub struct Adc<H, ADC: AdcOps<H>, CLOCK> {
    p: ADC,
    reading_channel: Option<ADC::Channel>,
    _clock: PhantomData<CLOCK>,
    _h: PhantomData<H>,
}

impl<H, ADC, CLOCK> Adc<H, ADC, CLOCK>
where
    ADC: AdcOps<H>,
    CLOCK: crate::clock::Clock,
{
    pub fn new(p: ADC, settings: ADC::Settings) -> Self {
        let mut adc = Self {
            p,
            reading_channel: None,
            _clock: PhantomData,
            _h: PhantomData,
        };
        adc.initialize(settings);
        adc
    }

    pub fn initialize(&mut self, settings: ADC::Settings) {
        self.p.raw_init(settings);
    }

    #[inline]
    pub(crate) fn enable_pin<PIN: AdcChannel<H, ADC>>(&mut self, pin: &PIN) {
        self.p.raw_enable_channel(pin.channel());
    }

    #[inline]
    pub(crate) fn disable_pin<PIN: AdcChannel<H, ADC>>(&mut self, pin: &PIN) {
        self.p.raw_disable_channel(pin.channel());
    }

    pub fn read_blocking<PIN: AdcChannel<H, ADC>>(&mut self, pin: &PIN) -> u16 {
        // assert!(self.reading_channel.is_none());
        self.p.raw_set_channel(pin.channel());
        self.p.raw_start_conversion();
        while self.p.raw_is_converting() {}
        self.p.raw_read_adc()
    }

    pub fn read_nonblocking<PIN: AdcChannel<H, ADC>>(
        &mut self,
        pin: &PIN,
    ) -> nb::Result<u16, core::convert::Infallible> {
        match (&self.reading_channel, self.p.raw_is_converting()) {
            // Measurement on current pin is ongoing
            (Some(channel), true) if *channel == pin.channel() => Err(nb::Error::WouldBlock),
            // Measurement on current pin completed
            (Some(channel), false) if *channel == pin.channel() => {
                self.reading_channel = None;
                Ok(self.p.raw_read_adc())
            }
            // Measurement on other pin is ongoing
            (Some(_), _) => {
                self.reading_channel = None;
                Err(nb::Error::WouldBlock)
            }
            // Start measurement
            (None, _) => {
                self.reading_channel = Some(pin.channel());
                self.p.raw_set_channel(pin.channel());
                self.p.raw_start_conversion();
                Err(nb::Error::WouldBlock)
            }
        }
    }
}

/// Generates implementations for the ADC traits and types plus some helper
/// types.
///
/// # Usage
///
/// *This is an internal macro of the library, user code does not need this.*
/// The main generator code is in the first alternative, it's fully generic to
/// what can vary between chips. For the Atmegas, which are pretty consistent, 2
/// convinience variants are provided that expand to the first with common
/// elements filled.
///
/// ## Adding a new implementation
///
/// The generic variant requires the following information (should be extracted
/// from the datasheet), generally in the form of register/bit names and masks
/// to generate safe Rust types properly:
/// - Available voltage references against which the ADC will compare;
///   - Passed as variants of a `ReferenceVoltage` enum;
/// - Procedure to select a reference;
///   - Passed as a function definition in a `set_reference` item, that must
///     set register state according to an argument with type `ReferenceVoltage`
///     defined above;
/// - Available ADC channels on the multiplexer;
///   - Passed as variants to a fake `Channels` enum, from which standalone structs
///     are generated per variant; The implementation of the structs is
///     parameterized by the value assigned to the fake variant, and should
///     be chosen as either [`crate::pac`] items or bitmasks that help the
///     assignment procedure;
/// - Procedure to select a channel;
///   - Passed as a function definition in a `set_channel` item, that must set
///     register state according to an argument with type [`AdcOps::Channel`]
///     adequate to the trait implementations;
/// - Available pins and respective channel;
///   - Passed as fake constant definitions that inform the pin type (see
///     [`crate::port`]), the corresponding register and bit to enable/disable
///     it, plus the value used to distinguish it when dynamic cast (an
///     arbitrary ID of sorts).
///
/// Helper matchers are defined for a "MegaA" kind, which only has one ADC
/// channel, and a "MegaAB" kind, which has two. These expand to calls of the
/// main one, because there are similarity patterns in the atmegas. The attinys
/// seem to have no recognizable pattern, so defining something like that would
/// be counterproductive; use the generic one unless you see the possibility to
/// merge your new chip with an existing chip.
///
/// # Example
///
/// ```ignore
/// #[cfg(any(
///     feature = "atmega328p",
///     // ...
/// ))]
/// avr_hal_generic::impl_adc! {
///     impl AdcProvider<MegaA> for pac::ADC {
///         type Hal = crate::Atmega;
///
///         const PC0: DIDR0::ADC0D = pac::adc::admux::MUX_A::ADC0;
///         const PC1: DIDR0::ADC1D = pac::adc::admux::MUX_A::ADC1;
///         // ...
///     }
///
///     type ChannelId = pac::adc::admux::MUX_A; // Or u8 if PAC doesn't define variants.
///     pub enum Channels {
///         channel::ADC7 = pac::adc::admux::MUX_A::ADC7,
///         channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
///         // ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_adc {
    (
        $(#[$ref_voltage_attr:meta])* pub enum ReferenceVoltage {
            $($(#[$ref_voltage_variant_attr:meta])* $ref_voltage_variant:ident,)*
        }

        pub fn set_reference($rself:ident, $rsettings:ident: Self::Settings) $set_reference_body:block
        pub fn set_channel($cself:ident, $cchannel:ident: Self::Channel) $set_channel_body:block

        impl AdcProvider for $adc:ty {
            type Hal = $hal:ty;

            $($(#[$pin_attr:meta])*
            const $pin:ty$(: $pin_reg:ident::$pin_bit:ident)? = $pin_id:expr;)+
        }

        type ChannelId = $channel_type:ty;
        pub enum Channels {
            $($(#[$channel_attr:meta])*
            $channel_variant:ty = $channel_id:expr,)+
        }
    ) => {
        $(#[$ref_voltage_attr])*
        pub enum ReferenceVoltage {
            $($(#[$ref_voltage_variant_attr])* $ref_voltage_variant),+
        }

        /// Configuration for the ADC peripheral.
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
        pub struct AdcSettings {
            pub clock_divider: $crate::adc::ClockDivider,
            pub ref_voltage: ReferenceVoltage,
        }

        impl $crate::adc::AdcOps<$hal> for $adc {
            type Channel = $channel_type;
            type Settings = AdcSettings;

            #[inline]
            fn raw_init(&mut $rself, $rsettings: Self::Settings) $set_reference_body

            #[inline]
            fn raw_read_adc(&self) -> u16 {
                self.adc.read().bits()
            }

            #[inline]
            fn raw_is_converting(&self) -> bool {
                self.adcsra.read().adsc().bit_is_set()
            }

            #[inline]
            fn raw_start_conversion(&mut self) {
                self.adcsra.modify(|_, w| w.adsc().set_bit());
            }

            #[inline]
            fn raw_set_channel(&mut $cself, $cchannel: Self::Channel) $set_channel_body

            #[inline]
            fn raw_enable_channel(&mut self, channel: Self::Channel) {
                $crate::paste::paste! {
                    match channel {
                        $(x @ $pin_id => { $(self.[<$pin_reg:lower>].modify(|_, w| w.[<$pin_bit:lower>]().set_bit()))? }),+
                        _ => unreachable!(),
                    }
                }
            }

            #[inline]
            fn raw_disable_channel(&mut self, channel: Self::Channel) {
                $crate::paste::paste! {
                    match channel {
                        $(x @ $pin_id => { $(self.[<$pin_reg:lower>].modify(|_, w| w.[<$pin_bit:lower>]().clear_bit()))? }),+
                        _ => unreachable!(),
                    }
                }
            }
        }

        $($(#[$pin_attr])*
        impl $crate::adc::AdcChannel<$hal, $adc> for $crate::port::Pin<$crate::port::mode::Analog, $pin> {
            #[inline]
            fn channel(&self) -> $channel_type {
                $pin_id
            }
        })+

        $(
            $(#[$channel_attr])*
            impl $crate::adc::AdcChannel<$hal, $adc> for $channel_variant {
                #[inline]
                fn channel(&self) -> $channel_type {
                    $channel_id
                }
            }
        )+
        $(
            /// Convert this channel into a generic "[`Channel`][adc-channel]" type.
            ///
            /// The generic channel type can be used to store multiple channels in an array.
            ///
            /// [adc-channel]: crate::adc::Channel
            $(#[$channel_attr])*
            impl $channel_variant {
                pub fn into_channel(self) -> $crate::adc::Channel<$hal, $adc> {
                    $crate::adc::Channel::new(self)
                }
            }
        )*
    };

    (
        impl AdcProvider<MegaA> for $adc:ty {
            type Hal = $hal:ty;

            $($(#[$pin_attr:meta])*
            const $pin:ty$(: $pin_reg:ident::$pin_bit:ident)? = $pin_id:expr;)+
        }

        type ChannelId = $channel_type:ty;
        pub enum Channels {
            $($(#[$channel_attr:meta])*
            $channel_variant:ty = $channel_value:expr,)+
        }
    ) => {
        $crate::impl_adc! {
            /// Select the voltage reference for the ADC peripheral
            ///
            /// The internal voltage reference options may not be used if an external reference voltage is
            /// being applied to the AREF pin.
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            #[repr(u8)]
            pub enum ReferenceVoltage {
                /// Voltage applied to AREF pin.
                Aref,
                /// System reference voltage, GND (default).
                #[default]
                AVcc,
                /// Internal reference.
                Internal,
            }

            pub fn set_reference(self, settings: Self::Settings) {
                self.adcsra.write(|w| {
                    w.aden().set_bit();
                    match settings.clock_divider {
                        $crate::adc::ClockDivider::Factor2 => w.adps().prescaler_2(),
                        $crate::adc::ClockDivider::Factor4 => w.adps().prescaler_4(),
                        $crate::adc::ClockDivider::Factor8 => w.adps().prescaler_8(),
                        $crate::adc::ClockDivider::Factor16 => w.adps().prescaler_16(),
                        $crate::adc::ClockDivider::Factor32 => w.adps().prescaler_32(),
                        $crate::adc::ClockDivider::Factor64 => w.adps().prescaler_64(),
                        $crate::adc::ClockDivider::Factor128 => w.adps().prescaler_128(),
                    }
                });
                self.admux.write(|w| match settings.ref_voltage {
                    ReferenceVoltage::Aref => w.refs().aref(),
                    ReferenceVoltage::AVcc => w.refs().avcc(),
                    ReferenceVoltage::Internal => w.refs().internal(),
                });
            }
            pub fn set_channel(self, channel: Self::Channel) {
                self.admux.modify(|_, w| w.mux().variant(channel));
            }

            impl AdcProvider for pac::ADC {
                type Hal = crate::Atmega;

                $($(#[$pin_attr])*
                const $pin$(: $pin_reg::$pin_bit)? = $pin_id;)+
            }

            type ChannelId = $channel_type;
            pub enum Channels {
                $($(#[$channel_attr])*
                $channel_variant = $channel_value,)+
            }
        }
    };
    (
        impl AdcProvider<MegaAB> for $adc:ty {
            type Hal = $hal:ty;

            $($(#[$pin_attr:meta])*
            const $pin:ty$(: $pin_reg:ident::$pin_bit:ident)? = $pin_id:expr;)+
        }

        pub enum Channels {
            $($(#[$channel_attr:meta])*
            $channel_variant:ty = $channel_value:expr,)+
        }
    ) => {
        $crate::impl_adc! {
            /// Select the voltage reference for the ADC peripheral
            ///
            /// The internal voltage reference options may not be used if an external reference voltage is
            /// being applied to the AREF pin.
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
            #[repr(u8)]
            pub enum ReferenceVoltage {
                /// Voltage applied to AREF pin.
                Aref,
                /// System reference voltage, GND (default).
                #[default]
                AVcc,
                /// Internal reference.
                Internal,
            }

            pub fn set_reference(self, settings: Self::Settings) {
                self.adcsra.write(|w| {
                    w.aden().set_bit();
                    match settings.clock_divider {
                        $crate::adc::ClockDivider::Factor2 => w.adps().prescaler_2(),
                        $crate::adc::ClockDivider::Factor4 => w.adps().prescaler_4(),
                        $crate::adc::ClockDivider::Factor8 => w.adps().prescaler_8(),
                        $crate::adc::ClockDivider::Factor16 => w.adps().prescaler_16(),
                        $crate::adc::ClockDivider::Factor32 => w.adps().prescaler_32(),
                        $crate::adc::ClockDivider::Factor64 => w.adps().prescaler_64(),
                        $crate::adc::ClockDivider::Factor128 => w.adps().prescaler_128(),
                    }
                });
                self.admux.write(|w| match settings.ref_voltage {
                    ReferenceVoltage::Aref => w.refs().aref(),
                    ReferenceVoltage::AVcc => w.refs().avcc(),
                    ReferenceVoltage::Internal => w.refs().internal(),
                });
            }
            pub fn set_channel(self, channel: Self::Channel) {
                self.admux.modify(|_, w| w.mux().bits(channel & 0x1f));
                self.adcsrb.modify(|_, w| w.mux5().bit(channel & 0x20 != 0));
            }

            impl AdcProvider for pac::ADC {
                type Hal = crate::Atmega;

                $($(#[$pin_attr])*
                const $pin$(: $pin_reg::$pin_bit)? = $pin_id;)+
            }

            type ChannelId = u8;
            pub enum Channels {
                $($(#[$channel_attr])*
                $channel_variant = $channel_value,)+
            }
        }
    };
}
