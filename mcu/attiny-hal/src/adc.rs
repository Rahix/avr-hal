#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter

use crate::port;
pub use avr_hal_generic::adc::{AdcChannel, AdcOps, ClockDivider};

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
    #[cfg(feature = "attiny167")]
    pub struct AVcc_4;
    pub struct Vbg;
    pub struct Gnd;
    pub struct Temperature;
}

#[cfg(feature = "attiny85")]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| {
        apply_clock(peripheral, settings);
        peripheral.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.refs().aref(),
            ReferenceVoltage::AVcc => w.refs().vcc(),
            ReferenceVoltage::Internal1_1 => w.refs().internal().refs2().clear_bit(),
            ReferenceVoltage::Internal2_56 => w.refs().internal().refs2().set_bit(),
        });
    },
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
        channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
    },
}


#[cfg(feature = "attiny88")]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| {
        apply_clock(peripheral, settings);
        peripheral.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::AVcc => w.refs0().avcc(),
            ReferenceVoltage::Internal1_1 => w.refs0().internal(),
        });
    },
    channel_id: crate::pac::adc::admux::MUX_A,
    set_channel: |peripheral, id| {
        peripheral.admux.modify(|_, w| w.mux().variant(id));
    },
    pins: {
        port::PC0: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        port::PC1: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        port::PC2: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        port::PC3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        port::PC4: (crate::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        port::PC5: (crate::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
        port::PA0: (crate::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
        port::PA1: (crate::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
    },
    channels: {
        channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
    },
}


#[cfg(feature = "attiny167")]
avr_hal_generic::impl_adc! {
    hal: crate::Attiny,
    peripheral: crate::pac::ADC,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| {
        apply_clock(peripheral, settings);
        peripheral.amiscr.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.arefen().set_bit(),
            _ => w.arefen().clear_bit(),
        });
        peripheral.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.refs().avcc(),
            ReferenceVoltage::AVcc => w.refs().avcc(),
            ReferenceVoltage::Internal1_1 => w.refs().internal_11(),
            ReferenceVoltage::Internal2_56 => w.refs().internal_256(),
        });
    },
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
