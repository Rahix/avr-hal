//! Analog-to-Digital Converter

use crate::port;
pub use avr_hal_generic::adc::{AdcChannel, AdcOps, AdcSettings, ClockDivider, ReferenceVoltage};

/// Check the [`avr_hal_generic::adc::Adc`] documentation.
pub type Adc<CLOCK> = avr_hal_generic::adc::Adc<crate::Atmega, crate::pac::ADC, CLOCK>;

/// Check the [`avr_hal_generic::adc::Channel`] documentation.
pub type Channel = avr_hal_generic::adc::Channel<crate::Atmega, crate::pac::ADC>;

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
    #[cfg(all(
        any(
            feature = "atmega168",
            feature = "atmega328p",
            feature = "atmega328pb",
            feature = "atmega48p",
        ),
        feature = "enable-extra-adc",
    ))]
    pub struct ADC6;
    #[cfg(all(
        any(
            feature = "atmega168",
            feature = "atmega328p",
            feature = "atmega328pb",
            feature = "atmega48p"
        ),
        feature = "enable-extra-adc",
    ))]
    pub struct ADC7;
    #[cfg(any(
        feature = "atmega1280",
        feature = "atmega168",
        feature = "atmega2560",
        feature = "atmega328p",
        feature = "atmega328pb",
        feature = "atmega32u4",
        feature = "atmega48p",
    ))]
    pub struct Vbg;
    #[cfg(any(
        feature = "atmega1280",
        feature = "atmega168",
        feature = "atmega2560",
        feature = "atmega328p",
        feature = "atmega328pb",
        feature = "atmega32u4",
        feature = "atmega48p",
    ))]
    pub struct Gnd;
    #[cfg(any(
        feature = "atmega328p",
        feature = "atmega328pb",
        feature = "atmega32u4",
        feature = "atmega48p",
    ))]
    pub struct Temperature;
}

#[cfg(any(
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega328pb",
    feature = "atmega48p",
))]
avr_hal_generic::impl_adc! {
    hal: crate::Atmega,
    peripheral: crate::pac::ADC,
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
    },
    channels: {
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC6: crate::pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC7: crate::pac::adc::admux::MUX_A::ADC7,
        channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
        #[cfg(any(feature = "atmega328p", feature = "atmega328pb", feature = "atmega48p"))]
        channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
    },
}

#[cfg(feature = "atmega32u4")]
avr_hal_generic::impl_adc! {
    hal: crate::Atmega,
    peripheral: crate::pac::ADC,
    channel_id: u8,
    set_channel: |peripheral, id| {
        peripheral.admux.modify(|_, w| w.mux().bits(id & 0x1f));
        peripheral.adcsrb.modify(|_, w| w.mux5().bit(id & 0x20 != 0));
    },
    pins: {
        port::PF0: (0b000000, didr0::adc0d),
        port::PF1: (0b000001, didr0::adc1d),
        port::PF4: (0b000100, didr0::adc4d),
        port::PF5: (0b000101, didr0::adc5d),
        port::PF6: (0b000110, didr0::adc6d),
        port::PF7: (0b000111, didr0::adc7d),
        port::PD4: (0b100000, didr2::adc8d),
        port::PD6: (0b100001, didr2::adc9d),
        port::PD7: (0b100010, didr2::adc10d),
        port::PB4: (0b100011, didr2::adc11d),
        port::PB5: (0b100100, didr2::adc12d),
        port::PB6: (0b100101, didr2::adc13d),
    },
    channels: {
        channel::Vbg: 0b011110,
        channel::Gnd: 0b011111,
        channel::Temperature: 0b100111,
    },
}

#[cfg(any(feature = "atmega2560", feature = "atmega1280"))]
avr_hal_generic::impl_adc! {
    hal: crate::Atmega,
    peripheral: crate::pac::ADC,
    channel_id: u8,
    set_channel: |peripheral, id| {
        peripheral.admux.modify(|_, w| w.mux().bits(id & 0x1f));
        peripheral.adcsrb.modify(|_, w| w.mux5().bit(id & 0x20 != 0));
    },
    pins: {
        port::PF0: (0b000000, didr0::adc0d),
        port::PF1: (0b000001, didr0::adc1d),
        port::PF2: (0b000010, didr0::adc2d),
        port::PF3: (0b000011, didr0::adc3d),
        port::PF4: (0b000100, didr0::adc4d),
        port::PF5: (0b000101, didr0::adc5d),
        port::PF6: (0b000110, didr0::adc6d),
        port::PF7: (0b000111, didr0::adc7d),
        port::PK0: (0b100000, didr2::adc8d),
        port::PK1: (0b100001, didr2::adc9d),
        port::PK2: (0b100010, didr2::adc10d),
        port::PK3: (0b100011, didr2::adc11d),
        port::PK4: (0b100100, didr2::adc12d),
        port::PK5: (0b100101, didr2::adc13d),
        port::PK6: (0b100110, didr2::adc14d),
        port::PK7: (0b100111, didr2::adc15d),
    },
    channels: {
        channel::Vbg: 0b011110,
        channel::Gnd: 0b011111,
    },
}
