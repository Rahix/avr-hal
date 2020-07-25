extern crate avr_hal_generic as avr_hal;

use crate::port::portc::{PC0, PC1, PC2, PC3, PC4, PC5};

use crate::atmega328p::adc::admux::MUX_A;

avr_hal::impl_adc! {
    pub struct Adc {
        type ChannelID = MUX_A;
        peripheral: crate::atmega328p::ADC,
        pins: {
            pc0: (PC0, MUX_A::ADC0, MUX_A::ADC0, adc0d),
            pc1: (PC1, MUX_A::ADC1, MUX_A::ADC1, adc1d),
            pc2: (PC2, MUX_A::ADC2, MUX_A::ADC2, adc2d),
            pc3: (PC3, MUX_A::ADC3, MUX_A::ADC3, adc3d),
            pc4: (PC4, MUX_A::ADC4, MUX_A::ADC4, adc4d),
            pc5: (PC5, MUX_A::ADC5, MUX_A::ADC5, adc5d),
        }
    }
}
