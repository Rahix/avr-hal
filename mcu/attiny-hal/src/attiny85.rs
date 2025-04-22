pub mod adc {
    pub use crate::periphals::adc::*;
    use crate::port;

    avr_hal_generic::impl_adc! {
        hal: crate::Attiny,
        peripheral: crate::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| {
            apply_clock(peripheral, settings);
            peripheral.admux.write(|w| match settings.ref_voltage {
                ReferenceVoltage::Aref => w.refs().aref(),
                ReferenceVoltage::AVcc => w.refs().vcc(),
                ReferenceVoltage::Internal1_1 => w.refs().internal().refs2().clear_bit(),
                ReferenceVoltage::Internal2_56 => w.refs().internal().refs2().set_bit(),
            });
        },
        channel_id: crate::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            port::PB5: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            port::PB2: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            port::PB4: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            port::PB3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        },
        channels: {
            channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
            channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
        },
    }
}
