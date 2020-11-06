extern crate avr_hal_generic as avr_hal;

use crate::port::portc::{PC0, PC1, PC2, PC3, PC4, PC5};

use crate::pac::adc::admux::MUX_A;

avr_hal_generic::impl_adc! {
    pub struct Adc {
        type ChannelID = MUX_A;
        peripheral: crate::pac::ADC,
        set_mux: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            pc0: (PC0, MUX_A::ADC0, didr0::adc0d),
            pc1: (PC1, MUX_A::ADC1, didr0::adc1d),
            pc2: (PC2, MUX_A::ADC2, didr0::adc2d),
            pc3: (PC3, MUX_A::ADC3, didr0::adc3d),
            pc4: (PC4, MUX_A::ADC4, didr0::adc4d),
            pc5: (PC5, MUX_A::ADC5, didr0::adc5d),
        }
    }
}

/// Additional channels
///
/// This module contains ADC channels, additional to the direct pin channels.
pub mod channel {
    use avr_hal_generic::hal::adc::Channel;
    use crate::pac::adc::admux::MUX_A;

    /// Channel for `ADC6` pin.
    ///
    /// This pin is not available in all ATmega328P packages (only 32TQFP, 32MLF, 32UFBGA).  If you
    /// are using one of them, enable the `adc-pins` feature to make them available.
    #[cfg(feature = "adc-pins")]
    pub struct ADC6;
    #[cfg(feature = "adc-pins")]
    impl Channel<super::Adc> for ADC6 {
        type ID = MUX_A;
        fn channel() -> Self::ID { MUX_A::ADC6 }
    }

    /// Channel for `ADC7` pin.
    ///
    /// This pin is not available in all ATmega328P packages (only 32TQFP, 32MLF, 32UFBGA).  If you
    /// are using one of them, enable the `adc-pins` feature to make them available.
    #[cfg(feature = "adc-pins")]
    pub struct ADC7;
    #[cfg(feature = "adc-pins")]
    impl Channel<super::Adc> for ADC7 {
        type ID = MUX_A;
        fn channel() -> Self::ID { MUX_A::ADC7 }
    }

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

    /// Channel for the built-in _Temperature Sensor_
    pub struct Temperature;
    impl Channel<super::Adc> for Temperature {
        type ID = MUX_A;
        fn channel() -> Self::ID { MUX_A::TEMPSENS }
    }
}
