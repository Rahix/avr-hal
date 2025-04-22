pub mod adc {
    pub use crate::periphals::adc::*;

    use crate::port;

    avr_hal_generic::impl_adc! {
        hal: crate::Attiny,
        peripheral: crate::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| {
            apply_clock(peripheral, settings);
            peripheral.amiscr.write(|w| match settings.ref_voltage {
                ReferenceVoltage::Aref => w.arefen().set_bit(),
                _ => w.arefen().clear_bit(),
            });
            peripheral.admux.write(|w| match settings.ref_voltage {
                ReferenceVoltage::Aref => w.refs().avcc(),
                ReferenceVoltage::AVcc => w.refs().avcc(),
                ReferenceVoltage::Internal1_1 => w.refs().internal_11(),
                ReferenceVoltage::Internal2_56 => w.refs().internal_256(),
            });
        },
        channel_id: crate::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            port::PA0: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            port::PA1: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            port::PA2: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            port::PA3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            port::PA4: (crate::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            port::PA5: (crate::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
            port::PA6: (crate::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            port::PA7: (crate::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
            port::PB5: (crate::pac::adc::admux::MUX_A::ADC8, didr1::adc8d),
            port::PB6: (crate::pac::adc::admux::MUX_A::ADC9, didr1::adc9d),
            port::PB7: (crate::pac::adc::admux::MUX_A::ADC10, didr1::adc10d),
        },
        channels: {
            channel::AVcc_4: crate::pac::adc::admux::MUX_A::ADC_AVCC_4,
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
        capacity: 512,
        addr_width: u16,
        set_address: |peripheral, address| {
            peripheral.eear.write(|w| w.bits(address));
        },
    }
}
