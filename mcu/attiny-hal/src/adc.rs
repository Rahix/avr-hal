#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter

use crate::{pac, port::*};
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
        /// Internal 1.1V reference.
        Internal1_1,
        /// Internal 2.56V reference.
        Internal2_56,
    }

    pub fn set_reference(self, settings: Self::Settings) {
        self.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::Aref => w.refs().aref(),
            ReferenceVoltage::AVcc => w.refs().vcc(),
            ReferenceVoltage::Internal1_1 => w.refs().internal().refs2().clear_bit(),
            ReferenceVoltage::Internal2_56 => w.refs().internal().refs2().set_bit(),
        });
    }
    pub fn set_channel(self, channel: Self::Channel) {
        self.admux.modify(|_, w| w.mux().variant(channel));
    }

    impl AdcProvider for pac::ADC {
        type Hal = crate::Attiny;

        const PB5: DIDR0::ADC0D = pac::adc::admux::MUX_A::ADC0;
        const PB2: DIDR0::ADC1D = pac::adc::admux::MUX_A::ADC1;
        const PB4: DIDR0::ADC2D = pac::adc::admux::MUX_A::ADC2;
        const PB3: DIDR0::ADC3D = pac::adc::admux::MUX_A::ADC3;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature = pac::adc::admux::MUX_A::TEMPSENS,
    }
}

#[cfg(feature = "attiny88")]
avr_hal_generic::impl_adc! {
    /// Select the voltage reference for the ADC peripheral
    ///
    /// The internal voltage reference options may not be used if an external reference voltage is
    /// being applied to the AREF pin.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum ReferenceVoltage {
        /// System reference voltage, GND (default).
        #[default]
        AVcc,
        /// Internal 1.1V reference.
        Internal1_1,
    }

    pub fn set_reference(self, settings: Self::Settings) {
        self.admux.write(|w| match settings.ref_voltage {
            ReferenceVoltage::AVcc => w.refs0().avcc(),
            ReferenceVoltage::Internal1_1 => w.refs0().internal(),
        });
    }
    pub fn set_channel(self, channel: Self::Channel) {
        self.admux.modify(|_, w| w.mux().variant(channel));
    }

    impl AdcProvider for pac::ADC {
        type Hal = crate::Attiny;

        const PC0: DIDR0::ADC0D = pac::adc::admux::MUX_A::ADC0;
        const PC1: DIDR0::ADC1D = pac::adc::admux::MUX_A::ADC1;
        const PC2: DIDR0::ADC2D = pac::adc::admux::MUX_A::ADC2;
        const PC3: DIDR0::ADC3D = pac::adc::admux::MUX_A::ADC3;
        const PC4: DIDR0::ADC4D = pac::adc::admux::MUX_A::ADC4;
        const PC5: DIDR0::ADC5D = pac::adc::admux::MUX_A::ADC5;
        const PA0: DIDR0::ADC6D = pac::adc::admux::MUX_A::ADC6;
        const PA1: DIDR0::ADC7D = pac::adc::admux::MUX_A::ADC7;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature = pac::adc::admux::MUX_A::TEMPSENS,
    }
}

#[cfg(feature = "attiny167")]
avr_hal_generic::impl_adc! {
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
        /// Internal 1.1V reference.
        Internal1_1,
        /// Internal 2.56V reference.
        Internal2_56,
    }

    pub fn set_reference(self, settings: Self::Settings) {
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
    pub fn set_channel(self, channel: Self::Channel) {
        self.admux.modify(|_, w| w.mux().variant(channel));
    }

    impl AdcProvider for pac::ADC {
        type Hal = crate::Attiny;

        const PB5: DIDR0::ADC0D = pac::adc::admux::MUX_A::ADC0;
        const PB2: DIDR0::ADC1D = pac::adc::admux::MUX_A::ADC1;
        const PB4: DIDR0::ADC2D = pac::adc::admux::MUX_A::ADC2;
        const PB3: DIDR0::ADC3D = pac::adc::admux::MUX_A::ADC3;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        channel::AVcc_4 = pac::adc::admux::MUX_A::ADC_AVCC_4,
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
        channel::Temperature = pac::adc::admux::MUX_A::TEMPSENS,
    }
}
