use crate::port::portb::{PB4, PB5, PB6};
use crate::port::portd::{PD4, PD6, PD7};
use crate::port::portf::{PF0, PF1, PF4, PF5, PF6, PF7};

pub use avr_hal_generic::adc::*;

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum AdcMux {
    Adc0 = 0b000000,
    Adc1 = 0b000001,
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

    AdcTemp = 0b100111,
}

avr_hal_generic::impl_adc! {
    pub struct Adc {
        type ChannelID = AdcMux;
        peripheral: crate::pac::ADC,
        set_mux: |peripheral, id| {
            let id = id as u8;
            peripheral.admux.modify(|_, w| w.mux().bits(id & 0x1f));
            peripheral.adcsrb.modify(|_, w| w.mux5().bit(id & 0x20 != 0));
        },
        pins: {
            pf0: (PF0, AdcMux::Adc0, didr0::adc0d),
            pf1: (PF1, AdcMux::Adc1, didr0::adc1d),
            pf4: (PF4, AdcMux::Adc4, didr0::adc4d),
            pf5: (PF5, AdcMux::Adc5, didr0::adc5d),
            pf6: (PF6, AdcMux::Adc6, didr0::adc6d),
            pf7: (PF7, AdcMux::Adc7, didr0::adc7d),
            pd4: (PD4, AdcMux::Adc8, didr2::adc8d),
            pd6: (PD6, AdcMux::Adc9, didr2::adc9d),
            pd7: (PD7, AdcMux::Adc10, didr2::adc10d),
            pb4: (PB4, AdcMux::Adc11, didr2::adc11d),
            pb5: (PB5, AdcMux::Adc12, didr2::adc12d),
            pb6: (PB6, AdcMux::Adc13, didr2::adc13d),
        }
    }
}

/// Additional channels
///
/// This module contains ADC channels, additional to the direct pin channels.
pub mod channel {
    use avr_hal_generic::hal::adc::Channel;
    use super::AdcMux;

    /// Channel for the _Bandgap Reference Voltage_
    pub struct Vbg;
    impl Channel<super::Adc> for Vbg {
        type ID = AdcMux;
        fn channel() -> Self::ID { AdcMux::AdcVbg }
    }

    /// Channel for _GND_
    pub struct Gnd;
    impl Channel<super::Adc> for Gnd {
        type ID = AdcMux;
        fn channel() -> Self::ID { AdcMux::AdcGnd }
    }

    /// Channel for the built-in _Temperature Sensor_
    pub struct Temperature;
    impl Channel<super::Adc> for Temperature {
        type ID = AdcMux;
        fn channel() -> Self::ID { AdcMux::AdcTemp }
    }
}
