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

/// Select the voltage reference for the ADC peripheral
///
/// The internal voltage reference options may not be used if an external reference voltage is
/// being applied to the AREF pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReferenceVoltage {
    /// Voltage applied to AREF pin.
    Aref,
    /// Default reference voltage (default).
    AVcc,
    /// Internal reference voltage.
    Internal,
}

impl Default for ReferenceVoltage {
    fn default() -> Self {
        Self::AVcc
    }
}

/// Configuration for the ADC peripheral.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdcSettings {
    pub clock_divider: ClockDivider,
    pub ref_voltage: ReferenceVoltage,
}

/// Internal trait for the low-level ADC peripheral.
///
/// **Prefer using the [`Adc`] API instead of this trait.**
pub trait AdcOps<H> {
    /// Channel ID type for this ADC.
    type Channel: PartialEq + Copy;

    /// Initialize the ADC peripheral with the specified settings.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_init(&mut self, settings: AdcSettings);

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
}

/// Trait marking a type as an ADC channel for a certain ADC.
pub trait AdcChannel<H, ADC: AdcOps<H>> {
    fn channel(&self) -> ADC::Channel;
}

/// Analog-to-Digital Converter
/// ```
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let pins = atmega_hal::pins!(dp);
/// let mut adc = atmega_hal::Adc::new(dp.ADC, Default::default());
///
/// let a0 = dp.pc0.into_analog_input(&mut adc);
///
/// // the following two calls are equivalent
/// let voltage = a0.analog_read(&mut adc);
/// let voltage = adc.read_blocking(&a0);
///
/// // alternatively, a non-blocking interface exists
/// let voltage = nb::block!(adc.read_nonblocking(&a0)).void_unwrap();
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
    pub fn new(p: ADC, settings: AdcSettings) -> Self {
        let mut adc = Self {
            p,
            reading_channel: None,
            _clock: PhantomData,
            _h: PhantomData,
        };
        adc.initialize(settings);
        adc
    }

    pub fn initialize(&mut self, settings: AdcSettings) {
        self.p.raw_init(settings);
    }

    #[inline]
    pub(crate) fn enable_pin<PIN: AdcChannel<H, ADC>>(&mut self, pin: &PIN) {
        self.p.raw_enable_channel(pin.channel());
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
    ) -> nb::Result<u16, void::Void> {
        match (&self.reading_channel, self.p.raw_is_converting()) {
            // Measurement on current pin is ongoing
            (Some(channel), true) if *channel == pin.channel() => Err(nb::Error::WouldBlock),
            // Measurement on current pin completed
            (Some(channel), false) if *channel == pin.channel() => {
                self.reading_channel = None;
                Ok(self.p.raw_read_adc().into())
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
        channel_id: $Channel:ty,
        set_channel: |$periph_var:ident, $chan_var:ident| $set_channel:block,
        pins: {
            $(
                $(#[$pin_attr:meta])*
                $pin:ty: ($pin_channel:expr, $didr:ident::$didr_method:ident),
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

            #[inline]
            fn raw_init(&mut self, settings: $crate::adc::AdcSettings) {
                self.adcsra.write(|w| {
                    w.aden().set_bit();
                    match settings.clock_divider {
                        ClockDivider::Factor2 => w.adps().prescaler_2(),
                        ClockDivider::Factor4 => w.adps().prescaler_4(),
                        ClockDivider::Factor8 => w.adps().prescaler_8(),
                        ClockDivider::Factor16 => w.adps().prescaler_16(),
                        ClockDivider::Factor32 => w.adps().prescaler_32(),
                        ClockDivider::Factor64 => w.adps().prescaler_64(),
                        ClockDivider::Factor128 => w.adps().prescaler_128(),
                    }
                });
                self.admux.write(|w| match settings.ref_voltage {
                    ReferenceVoltage::Aref => w.refs().aref(),
                    ReferenceVoltage::AVcc => w.refs().avcc(),
                    ReferenceVoltage::Internal => w.refs().internal(),
                });
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
                        $(#[$pin_attr])*
                        x if x == $pin_channel => self.$didr.modify(|_, w| w.$didr_method().set_bit()),
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
        )*)?
    };
}
