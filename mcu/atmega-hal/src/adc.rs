//! Analog-to-Digital Converter

use crate::{pac, port::*};
pub use avr_hal_generic::adc::{AdcChannel, AdcOps, ClockDivider};

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
            feature = "atmega32a",
            feature = "atmega328p",
            feature = "atmega328pb",
            feature = "atmega48p",
            feature = "atmega128a",
            feature = "atmega1284p",
            feature = "atmega8",
        ),
        feature = "enable-extra-adc",
    ))]
    pub struct ADC6;
    #[cfg(all(
        any(
            feature = "atmega168",
            feature = "atmega32a",
            feature = "atmega328p",
            feature = "atmega328pb",
            feature = "atmega48p",
            feature = "atmega128a",
            feature = "atmega1284p",
            feature = "atmega8",
        ),
        feature = "enable-extra-adc",
    ))]
    pub struct ADC7;
    #[cfg(any(
        feature = "atmega1280",
        feature = "atmega168",
        feature = "atmega2560",
        feature = "atmega32a",
        feature = "atmega328p",
        feature = "atmega328pb",
        feature = "atmega32u4",
        feature = "atmega48p",
        feature = "atmega128a",
        feature = "atmega1284p",
        feature = "atmega8",
        feature = "atmega164pa",
    ))]
    pub struct Vbg;
    #[cfg(any(
        feature = "atmega1280",
        feature = "atmega168",
        feature = "atmega2560",
        feature = "atmega32a",
        feature = "atmega328p",
        feature = "atmega328pb",
        feature = "atmega32u4",
        feature = "atmega48p",
        feature = "atmega128a",
        feature = "atmega1284p",
        feature = "atmega8",
        feature = "atmega164pa",
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
    impl AdcProvider<MegaA> for pac::ADC {
        type Hal = crate::Atmega;

        const PC0: DIDR0::ADC0D = pac::adc::admux::MUX_A::ADC0;
        const PC1: DIDR0::ADC1D = pac::adc::admux::MUX_A::ADC1;
        const PC2: DIDR0::ADC2D = pac::adc::admux::MUX_A::ADC2;
        const PC3: DIDR0::ADC3D = pac::adc::admux::MUX_A::ADC3;
        const PC4: DIDR0::ADC4D = pac::adc::admux::MUX_A::ADC4;
        const PC5: DIDR0::ADC5D = pac::adc::admux::MUX_A::ADC5;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC6 = pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC7 = pac::adc::admux::MUX_A::ADC7,
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
        #[cfg(any(feature = "atmega328p", feature = "atmega328pb", feature = "atmega48p"))]
        channel::Temperature = pac::adc::admux::MUX_A::TEMPSENS,
    }
}

#[cfg(any(feature = "atmega32a"))]
avr_hal_generic::impl_adc! {
    impl AdcProvider<MegaA> for pac::ADC {
        type Hal = crate::Atmega;

        const PA0 = pac::adc::admux::MUX_A::ADC0;
        const PA1 = pac::adc::admux::MUX_A::ADC1;
        const PA2 = pac::adc::admux::MUX_A::ADC2;
        const PA3 = pac::adc::admux::MUX_A::ADC3;
        const PA4 = pac::adc::admux::MUX_A::ADC4;
        const PA5 = pac::adc::admux::MUX_A::ADC5;
        const PA6 = pac::adc::admux::MUX_A::ADC6;
        const PA7 = pac::adc::admux::MUX_A::ADC7;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
    }
}

#[cfg(feature = "atmega32u4")]
avr_hal_generic::impl_adc! {
    impl AdcProvider<MegaAB> for pac::ADC {
        type Hal = crate::Atmega;

        const PF0: DIDR0::ADC0D = 0b000000;
        const PF1: DIDR0::ADC1D = 0b000001;
        const PF4: DIDR0::ADC4D = 0b000100;
        const PF5: DIDR0::ADC5D = 0b000101;
        const PF6: DIDR0::ADC6D = 0b000110;
        const PF7: DIDR0::ADC7D = 0b000111;
        const PD4: DIDR2::ADC8D = 0b100000;
        const PD6: DIDR2::ADC9D = 0b100001;
        const PD7: DIDR2::ADC10D = 0b100010;
        const PB4: DIDR2::ADC11D = 0b100011;
        const PB5: DIDR2::ADC12D = 0b100100;
        const PB6: DIDR2::ADC13D = 0b100101;
    }

    pub enum Channels {
        channel::Vbg = 0b011110,
        channel::Gnd = 0b011111,
        channel::Temperature = 0b100111,
    }
}

#[cfg(any(feature = "atmega128a"))]
avr_hal_generic::impl_adc! {
    impl AdcProvider<MegaA> for pac::ADC {
        type Hal = crate::Atmega;

        const PF0 = pac::adc::admux::MUX_A::ADC0;
        const PF1 = pac::adc::admux::MUX_A::ADC1;
        const PF2 = pac::adc::admux::MUX_A::ADC2;
        const PF3 = pac::adc::admux::MUX_A::ADC3;
        const PF4 = pac::adc::admux::MUX_A::ADC4;
        const PF5 = pac::adc::admux::MUX_A::ADC5;
        const PF6 = pac::adc::admux::MUX_A::ADC6;
        const PF7 = pac::adc::admux::MUX_A::ADC7;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
    }
}

#[cfg(any(feature = "atmega2560", feature = "atmega1280"))]
avr_hal_generic::impl_adc! {
    impl AdcProvider<MegaAB> for pac::ADC {
        type Hal = crate::Atmega;

        const PF0: DIDR0::ADC0D = 0b000000;
        const PF1: DIDR0::ADC1D = 0b000001;
        const PF2: DIDR0::ADC2D = 0b000010;
        const PF3: DIDR0::ADC3D = 0b000011;
        const PF4: DIDR0::ADC4D = 0b000100;
        const PF5: DIDR0::ADC5D = 0b000101;
        const PF6: DIDR0::ADC6D = 0b000110;
        const PF7: DIDR0::ADC7D = 0b000111;
        const PK0: DIDR2::ADC8D = 0b100000;
        const PK1: DIDR2::ADC9D = 0b100001;
        const PK2: DIDR2::ADC10D = 0b100010;
        const PK3: DIDR2::ADC11D = 0b100011;
        const PK4: DIDR2::ADC12D = 0b100100;
        const PK5: DIDR2::ADC13D = 0b100101;
        const PK6: DIDR2::ADC14D = 0b100110;
        const PK7: DIDR2::ADC15D = 0b100111;
    }

    pub enum Channels {
        channel::Vbg = 0b011110,
        channel::Gnd = 0b011111,
    }
}

#[cfg(any(feature = "atmega1284p"))]
avr_hal_generic::impl_adc! {
    impl AdcProvider<MegaA> for pac::ADC {
        type Hal = crate::Atmega;

        const PA0: DIDR0::ADC0D = pac::adc::admux::MUX_A::ADC0;
        const PA1: DIDR0::ADC1D = pac::adc::admux::MUX_A::ADC1;
        const PA2: DIDR0::ADC2D = pac::adc::admux::MUX_A::ADC2;
        const PA3: DIDR0::ADC3D = pac::adc::admux::MUX_A::ADC3;
        const PA4: DIDR0::ADC4D = pac::adc::admux::MUX_A::ADC4;
        const PA5: DIDR0::ADC5D = pac::adc::admux::MUX_A::ADC5;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC6 = pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC7 = pac::adc::admux::MUX_A::ADC7,
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
    }
}

#[cfg(any(feature = "atmega8"))]
avr_hal_generic::impl_adc! {
    impl AdcProvider<MegaA> for pac::ADC {
        type Hal = crate::Atmega;

        const PC0 = pac::adc::admux::MUX_A::ADC0;
        const PC1 = pac::adc::admux::MUX_A::ADC1;
        const PC2 = pac::adc::admux::MUX_A::ADC2;
        const PC3 = pac::adc::admux::MUX_A::ADC3;
        const PC4 = pac::adc::admux::MUX_A::ADC4;
        const PC5 = pac::adc::admux::MUX_A::ADC5;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC6 = pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        channel::ADC7 = pac::adc::admux::MUX_A::ADC7,
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
    }
}

#[cfg(any(feature = "atmega164pa"))]
avr_hal_generic::impl_adc! {
    impl AdcProvider<MegaA> for pac::ADC {
        type Hal = crate::Atmega;

        const PC0 = pac::adc::admux::MUX_A::ADC0;
        const PC1 = pac::adc::admux::MUX_A::ADC1;
        const PC2 = pac::adc::admux::MUX_A::ADC2;
        const PC3 = pac::adc::admux::MUX_A::ADC3;
        const PC4 = pac::adc::admux::MUX_A::ADC4;
        const PC5 = pac::adc::admux::MUX_A::ADC5;
        const PC6 = pac::adc::admux::MUX_A::ADC6;
        const PC7 = pac::adc::admux::MUX_A::ADC7;
    }

    type ChannelId = pac::adc::admux::MUX_A;
    pub enum Channels {
        channel::Vbg = pac::adc::admux::MUX_A::ADC_VBG,
        channel::Gnd = pac::adc::admux::MUX_A::ADC_GND,
    }
}
