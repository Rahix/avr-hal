extern crate avr_hal_generic as avr_hal;

use crate::port::portc::{PC0, PC1, PC2, PC3, PC4, PC5};

use crate::atmega328p::adc::admux::MUX_A;

avr_hal::impl_adc! {
    pub struct Adc {
        type ChannelID = MUX_A;
        peripheral: crate::atmega328p::ADC,
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
