/// Analog-to-Digial converter
use core::marker::PhantomData;

/// The division factor between the system clock frequency and the input clock to the AD converter.
///
/// To get 10-bit precision, clock from 50kHz to 200kHz must be supplied.  If you need less
/// precision, you can supply a higher clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockDivider {
    Factor2,
    Factor4,
    Factor8,
    Factor16,
    Factor32,
    Factor64,
    /// (default)
    Factor128,
}

impl Default for ClockDivider {
    fn default() -> Self {
        Self::Factor128
    }
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

#[macro_export]
macro_rules! impl_adc {
    (
        hal: $HAL:ty,
        peripheral: $ADC:ty,
        settings: $Settings:ty,
        apply_settings: |$settings_periph_var:ident, $settings_var:ident| $apply_settings:block,
        channel_id: $Channel:ty,
        set_channel: |$periph_var:ident, $chan_var:ident| $set_channel:block,
        pins: {
            $(
                $(#[$pin_attr:meta])*
                $pin:ty: ($pin_channel:expr$(, $didr:ident::$didr_method:ident)?),
            )+
        },
        $(channels: {
            $(
                $(#[$channel_attr:meta])*
                $channel_ty:ty: $channel:expr,
            )*
        },)?
    ) => {
        impl $crate::adc::AdcOps<$HAL> for $ADC {
            type Channel = $Channel;
            type Settings = $Settings;

            #[inline]
            fn raw_init(&mut self, settings: Self::Settings) {
                let $settings_periph_var = self;
                let $settings_var = settings;

                $apply_settings
            }

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
            fn raw_set_channel(&mut self, channel: Self::Channel) {
                let $periph_var = self;
                let $chan_var = channel;

                $set_channel
            }

            #[inline]
            fn raw_enable_channel(&mut self, channel: Self::Channel) {
                match channel {
                    $(
                        x if x == $pin_channel => {
                            $(self.$didr.modify(|_, w| w.$didr_method().set_bit());)?
                        }
                    )+
                    _ => unreachable!(),
                }
            }

            #[inline]
            fn raw_disable_channel(&mut self, channel: Self::Channel) {
                match channel {
                    $(
                        x if x == $pin_channel => {
                            $(self.$didr.modify(|_, w| w.$didr_method().clear_bit());)?
                        }
                    )+
                    _ => unreachable!(),
                }
            }
        }

        $(
        $(#[$pin_attr])*
        impl $crate::adc::AdcChannel<$HAL, $ADC> for $crate::port::Pin<$crate::port::mode::Analog, $pin> {
            #[inline]
            fn channel(&self) -> $Channel {
                $pin_channel
            }
        }
        )+

        $($(
        $(#[$channel_attr])*
        impl $crate::adc::AdcChannel<$HAL, $ADC> for $channel_ty {
            #[inline]
            fn channel(&self) -> $Channel {
                $channel
            }
        }

        /// Convert this channel into a generic "[`Channel`][adc-channel]" type.
        ///
        /// The generic channel type can be used to store multiple channels in an array.
        ///
        /// [adc-channel]: crate::adc::Channel
        $(#[$channel_attr])*
        impl $channel_ty {
            pub fn into_channel(self) -> $crate::adc::Channel<$HAL, $ADC> {
                $crate::adc::Channel::new(self)
            }
        }
        )*)?
    };
}
