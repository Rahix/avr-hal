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

fn apply_settings(peripheral: &crate::pac::ADC, settings: AdcSettings) {
    peripheral.adcsra.write(|w| {
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
    peripheral.admux.write(|w| match settings.ref_voltage {
        ReferenceVoltage::Aref => w.refs().aref(),
        ReferenceVoltage::AVcc => w.refs().vcc(),
        ReferenceVoltage::Internal => w.refs().internal(),
    });
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
/// let dp = attiny_hal::Peripherals::take().unwrap();
/// let mut adc = attiny_hal::Adc::new(dp.ADC, Default::default());
///
/// let value = adc.read_blocking(&channel::Vbg);
/// ```
pub mod channel {
    #[cfg(any(feature = "attiny85"))]
    pub struct Vbg;
    #[cfg(any(feature = "attiny85"))]
    pub struct Gnd;
    #[cfg(any(feature = "attiny85"))]
    pub struct Temperature;
}

#[cfg(any(feature = "attiny85"))]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
    channel_id: crate::pac::adc::admux::MUX_A,
    set_channel: |peripheral, id| {
        peripheral.admux.modify(|_, w| w.mux().variant(id));
    },
    pins: {
        port::PB5: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        port::PB2: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        port::PB4: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        port::PB3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
    },
    channels: {
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC4: crate::pac::adc::admux::MUX_A::ADC4,
        channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
        #[cfg(any(feature = "attiny85"))]
        channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
    },
}
