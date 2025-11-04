//! Analog-to-Digital Converter
//!
//! # Example
//!
//! Complete example source code can be found in the repository:
//! [`atmega2560-adc.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-adc.rs)
//!
//! ```
//! let dp = atmega_hal::Peripherals::take().unwrap();
//! let pins = atmega_hal::pins!(dp);
//!
//! let mut adc = Adc::new(dp.ADC, Default::default());
//!
//! let channels: [atmega_hal::adc::Channel; 4] = [
//!     pins.pf0.into_analog_input(&mut adc).into_channel(),
//!     pins.pf1.into_analog_input(&mut adc).into_channel(),
//!     pins.pf2.into_analog_input(&mut adc).into_channel(),
//!     pins.pf3.into_analog_input(&mut adc).into_channel(),
//! ];
//!
//! for (index, channel) in channels.iter().enumerate() {
//!     let value = adc.read_blocking(channel);
//!     ufmt::uwrite!(&mut serial, "CH{}: {} ", index, value).unwrap();
//! }
//! ```

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

fn apply_settings(peripheral: &crate::pac::ADC0, settings: AdcSettings) {
    peripheral.ctrlc.write(|w| {
        match settings.ref_voltage {
            ReferenceVoltage::Aref => w.refsel().vrefa(),
            ReferenceVoltage::AVcc => w.refsel().vddref(),
            ReferenceVoltage::Internal => w.refsel().intref(),
        };
        match settings.clock_divider {
            ClockDivider::Factor2 => w.presc().div2(),
            ClockDivider::Factor4 => w.presc().div4(),
            ClockDivider::Factor8 => w.presc().div8(),
            ClockDivider::Factor16 => w.presc().div16(),
            ClockDivider::Factor32 => w.presc().div32(),
            ClockDivider::Factor64 => w.presc().div64(),
            ClockDivider::Factor128 => w.presc().div128(),
            ClockDivider::Factor256 => w.presc().div256(),
        }
    });
    peripheral.ctrla.write(|w| w.enable().set_bit());
}

/// Check the [`avr_hal_generic::adc::Adc`] documentation.
pub type Adc<CLOCK> = avr_hal_generic::adc::Adc<crate::Atxmega, crate::pac::ADC0, CLOCK>;

/// Check the [`avr_hal_generic::adc::Channel`] documentation.
pub type Channel = avr_hal_generic::adc::Channel<crate::Atxmega, crate::pac::ADC0>;

/// Additional channels
///
/// Some channels are not directly connected to pins.  This module provides types which can be used
/// to access them.
///
/// # Example
/// ```
/// let dp = atxmega_hal::Peripherals::take().unwrap();
/// let mut adc = atxmega_hal::Adc::new(dp.ADC, Default::default());
///
/// let value = adc.read_blocking(&channel::Vbg);
/// ```
pub mod channel {
    #[cfg(all(any(feature = "atmega4809",), feature = "enable-extra-adc",))]
    pub struct ADC6;
    #[cfg(all(any(feature = "atmega4809",), feature = "enable-extra-adc",))]
    pub struct ADC7;
    #[cfg(any(feature = "atmega4809",))]
    pub struct Vbg;
    #[cfg(any(feature = "atmega4809",))]
    pub struct Gnd;
    #[cfg(any(feature = "atmega4809",))]
    pub struct Temperature;
}

#[cfg(any(feature = "atmega4809"))]
avr_hal_generic::impl_adc_new! {
    hal: crate::Atxmega,
    peripheral: crate::pac::ADC0,
    settings: AdcSettings,
    apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
    channel_id: crate::pac::adc0::muxpos::MUXPOS_A,
    set_channel: |peripheral, id| {
        peripheral.muxpos.modify(|_, w| w.muxpos().variant(id));
    },
    pins: {
        port::PD0: (crate::pac::adc0::muxpos::MUXPOS_A::AIN0),
        port::PD1: (crate::pac::adc0::muxpos::MUXPOS_A::AIN1),
        port::PD2: (crate::pac::adc0::muxpos::MUXPOS_A::AIN2),
        port::PD3: (crate::pac::adc0::muxpos::MUXPOS_A::AIN3),
        port::PD4: (crate::pac::adc0::muxpos::MUXPOS_A::AIN4),
        port::PD5: (crate::pac::adc0::muxpos::MUXPOS_A::AIN5),
        port::PF2: (crate::pac::adc0::muxpos::MUXPOS_A::AIN12),
        port::PF3: (crate::pac::adc0::muxpos::MUXPOS_A::AIN13),
    },
    channels: {
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC6: crate::pac::adc0::muxpos::MUXPOS_A::AIN6,
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC7: crate::pac::adc0::muxpos::MUXPOS_A::AIN7,
        channel::Vbg: crate::pac::adc0::muxpos::MUXPOS_A::DACREF,
        channel::Gnd: crate::pac::adc0::muxpos::MUXPOS_A::GND,
        channel::Temperature: crate::pac::adc0::muxpos::MUXPOS_A::TEMPSENSE,
    },
}
