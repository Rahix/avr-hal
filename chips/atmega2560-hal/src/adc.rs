extern crate avr_hal_generic as avr_hal;

use crate::port::portf::{PF0, PF1, PF2, PF3, PF4, PF5, PF6, PF7};
use crate::port::portk::{PK0, PK1, PK2, PK3, PK4, PK5, PK6, PK7};

use crate::atmega2560::adc::admux::MUX_A;

avr_hal::impl_adc! {
    pub struct Adc {
        type ChannelID = MUX_A;
        peripheral: crate::atmega2560::ADC,
        set_mux: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            pf0: (PF0, MUX_A::ADC0, didr0::adc0d),
            pf1: (PF1, MUX_A::ADC1, didr0::adc1d),
            pf2: (PF2, MUX_A::ADC2, didr0::adc2d),
            pf3: (PF3, MUX_A::ADC3, didr0::adc3d),
            pf4: (PF4, MUX_A::ADC4, didr0::adc4d),
            pf5: (PF5, MUX_A::ADC5, didr0::adc5d),
            pf6: (PF6, MUX_A::ADC6, didr0::adc6d),
            pf7: (PF7, MUX_A::ADC7, didr0::adc7d),
            pk0: (PK0, MUX_A::ADC8, didr2::adc8d),
            pk1: (PK1, MUX_A::ADC9, didr2::adc9d),
            pk2: (PK2, MUX_A::ADC10, didr2::adc10d),
            pk3: (PK3, MUX_A::ADC11, didr2::adc11d),
            pk4: (PK4, MUX_A::ADC12, didr2::adc12d),
            pk5: (PK5, MUX_A::ADC13, didr2::adc13d),
            pk6: (PK6, MUX_A::ADC14, didr2::adc14d),
            pk7: (PK7, MUX_A::ADC15, didr2::adc15d),
        }
    }
}

/// Additional channels
///
/// This module contains ADC channels, additional to the direct pin channels.
pub mod channel {
    use avr_hal::hal::adc::Channel;
    use crate::atmega2560::adc::admux::MUX_A;

    /// Channel for the _Bandgap Reference Voltage_
    pub struct Vbg;
    impl Channel<super::Adc> for Vbg {
        type ID = MUX_A;
        fn channel() -> Self::ID { MUX_A::ADC_VBG }
    }

    /// Channel for _GND_
    pub struct Gnd;
    impl Channel<super::Adc> for Gnd {
        type ID = MUX_A;
        fn channel() -> Self::ID { MUX_A::ADC_GND }
    }
}
