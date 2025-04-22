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
                ReferenceVoltage::AVcc => w.refs0().avcc(),
                ReferenceVoltage::Internal1_1 => w.refs0().internal(),
            });
        },
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
            port::PA0: (crate::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            port::PA1: (crate::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        },
        channels: {
            channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
            channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
        },
    }
}

pub mod eeprom {
    pub use crate::periphals::eeprom::*;

    avr_hal_generic::impl_eeprom_attiny! {
        hal: crate::Attiny,
        peripheral: crate::pac::EEPROM,
        capacity: 64,
        addr_width: u8,
        set_address: |peripheral, address| {
            peripheral.eearl.write(|w| w.bits(address));
        },
    }
}
