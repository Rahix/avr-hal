extern crate avr_hal_generic as avr_hal;

use crate::port::portf::{PF0, PF1, PF2, PF3, PF4, PF5, PF6, PF7};
use crate::port::portk::{PK0, PK1, PK2, PK3, PK4, PK5, PK6, PK7};

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum AdcMux {
    Adc0 = 0b000000,
    Adc1 = 0b000001,
    Adc2 = 0b000010,
    Adc3 = 0b000011,
    Adc4 = 0b000100,
    Adc5 = 0b000101,
    Adc6 = 0b000110,
    Adc7 = 0b000111,

    AdcVbg = 0b011110,
    AdcGnd = 0b011111,

    Adc8 = 0b100000,
    Adc9 = 0b100001,
    Adc10 = 0b100010,
    Adc11 = 0b100011,
    Adc12 = 0b100100,
    Adc13 = 0b100101,
    Adc14 = 0b100110,
    Adc15 = 0b100111,
}

avr_hal_generic::impl_adc! {
    pub struct Adc {
        type ChannelID = AdcMux;
        peripheral: crate::pac::ADC,
        set_mux: |peripheral, id| {
            let id = id as u8;
            peripheral.admux.modify(|_, w| w.mux().bits(id & 0x1F));
            // n.b. the high bit of ADMUX[MUX] is in the ADCSRB register
            peripheral.adcsrb.modify(|_, w| w.mux5().bit(id & 0x20 != 0));
        },
        pins: {
            pf0: (PF0, AdcMux::Adc0, didr0::adc0d),
            pf1: (PF1, AdcMux::Adc1, didr0::adc1d),
            pf2: (PF2, AdcMux::Adc2, didr0::adc2d),
            pf3: (PF3, AdcMux::Adc3, didr0::adc3d),
            pf4: (PF4, AdcMux::Adc4, didr0::adc4d),
            pf5: (PF5, AdcMux::Adc5, didr0::adc5d),
            pf6: (PF6, AdcMux::Adc6, didr0::adc6d),
            pf7: (PF7, AdcMux::Adc7, didr0::adc7d),
            pk0: (PK0, AdcMux::Adc8, didr2::adc8d),
            pk1: (PK1, AdcMux::Adc9, didr2::adc9d),
            pk2: (PK2, AdcMux::Adc10, didr2::adc10d),
            pk3: (PK3, AdcMux::Adc11, didr2::adc11d),
            pk4: (PK4, AdcMux::Adc12, didr2::adc12d),
            pk5: (PK5, AdcMux::Adc13, didr2::adc13d),
            pk6: (PK6, AdcMux::Adc14, didr2::adc14d),
            pk7: (PK7, AdcMux::Adc15, didr2::adc15d),
        }
    }
}

/// Additional channels
///
/// This module contains ADC channels, additional to the direct pin channels.
pub mod channel {
    use super::AdcMux;
    use avr_hal_generic::hal::adc::Channel;

    /// Channel for the _Bandgap Reference Voltage_
    pub struct Vbg;
    impl Channel<super::Adc> for Vbg {
        type ID = AdcMux;
        fn channel() -> Self::ID {
            AdcMux::AdcVbg
        }
    }

    /// Channel for _GND_
    pub struct Gnd;
    impl Channel<super::Adc> for Gnd {
        type ID = AdcMux;
        fn channel() -> Self::ID {
            AdcMux::AdcGnd
        }
    }
}
