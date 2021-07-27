//! Analog-to-Digital Converter

use crate::port;
pub use avr_hal_generic::adc::{AdcChannel, AdcOps, ClockDivider};

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
    /// Internal 1.1V reference.
    Internal1_1,
    /// Internal 2.56V reference.
    Internal2_56,
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

/// Check the [`avr_hal_generic::adc::Adc`] documentation.
pub type Adc<CLOCK> = avr_hal_generic::adc::Adc<crate::Attiny, crate::pac::ADC, CLOCK>;

/// Check the [`avr_hal_generic::adc::Channel`] documentation.
pub type Channel = avr_hal_generic::adc::Channel<crate::Attiny, crate::pac::ADC>;

/// Additional channels
///
/// Some channels are not directly connected to pins.  This module provides types which can be used
/// to access them.
///
/// # Example
/// ```
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let mut adc = atmega_hal::Adc::new(dp.ADC, Default::default());
///
/// let value = adc.read_blocking(&channel::Vbg);
/// ```
pub mod channel {
    #[cfg(feature = "attiny167")]
    pub struct AVcc_4;
    #[cfg(any(
        feature = "attiny167",
    ))]
    pub struct Vbg;
    #[cfg(any(
        feature = "attiny167",
    ))]
    pub struct Gnd;
    #[cfg(any(
        feature = "attiny167",
    ))]
    pub struct Temperature;
}

impl avr_hal_generic::adc::AdcSettings<crate::Attiny> for crate::pac::ADC {
    type Settings = AdcSettings;

    #[inline]
    fn raw_init(&mut self, settings: Self::Settings) {
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
        self.amiscr.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.arefen().set_bit(),
            _ => w.arefen().clear_bit(),
        });
        self.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.refs().avcc(),
            ReferenceVoltage::AVcc => w.refs().avcc(),
            ReferenceVoltage::Internal1_1 => w.refs().internal_11(),
            ReferenceVoltage::Internal2_56 => w.refs().internal_256(),
        });
    }
}


#[cfg(feature = "attiny167")]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC,
    channel_id: crate::pac::adc::admux::MUX_A,
    set_channel: |peripheral, id| {
        peripheral.admux.modify(|_, w| w.mux().variant(id));
    },
    pins: {
        port::PA0: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        port::PA1: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        port::PA2: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        port::PA3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        port::PA4: (crate::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        port::PA5: (crate::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
        port::PA6: (crate::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
        port::PA7: (crate::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        port::PB5: (crate::pac::adc::admux::MUX_A::ADC8, didr1::adc8d),
        port::PB6: (crate::pac::adc::admux::MUX_A::ADC9, didr1::adc9d),
        port::PB7: (crate::pac::adc::admux::MUX_A::ADC10, didr1::adc10d),
    },
    channels: {
        channel::AVcc_4: crate::pac::adc::admux::MUX_A::ADC_AVCC_4,
        channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
    },
}
