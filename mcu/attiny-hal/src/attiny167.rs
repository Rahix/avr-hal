pub use avr_device::attiny167 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::attiny167 as hal;

    impl_adc_reference_voltage! {
        pub enum ReferenceVoltage {
            /// Voltage applied to AREF pin.
            Aref,
            /// Default reference voltage (default).
            AVcc,
            /// Internal 1.1V reference.
            Internal1_1,
            /// Internal 2.56V reference.
            Internal2_56,
        }
    }

    impl_adc_channels! {
        pub struct AVcc_4;
        pub struct Vbg;
        pub struct Gnd;
        pub struct Temperature;
    }

    impl_adc_peripheral! {
        pac: crate::attiny167::pac,
        hal: crate::attiny167::Hal,
    }

    avr_hal_generic::impl_adc! {
        hal: hal::Hal,
        peripheral: hal::pac::ADC,
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
        channel_id: hal::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            hal::port::PA0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            hal::port::PA1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            hal::port::PA2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            hal::port::PA3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            hal::port::PA4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            hal::port::PA5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
            hal::port::PA6: (hal::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            hal::port::PA7: (hal::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
            hal::port::PB5: (hal::pac::adc::admux::MUX_A::ADC8, didr1::adc8d),
            hal::port::PB6: (hal::pac::adc::admux::MUX_A::ADC9, didr1::adc9d),
            hal::port::PB7: (hal::pac::adc::admux::MUX_A::ADC10, didr1::adc10d),
        },
        channels: {
            channel::AVcc_4: hal::pac::adc::admux::MUX_A::ADC_AVCC_4,
            channel::Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
            channel::Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::attiny167,
    capacity: 512,
    addr_width: u16,
    addr_reg: eear,
}

impl_mod_port! {
    use crate::attiny167 as hal;

    pub use avr_hal_generic::port::{mode, PinMode, PinOps};
    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: hal::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
        }
    }

    #[macro_export]
    macro_rules! attiny167_pins {
        ($p:expr) => {
            $crate::attiny167_pins::Pins::new($p.PORTA, $p.PORTB)
        };
    }

    pub use attiny167_pins as pins;
}

impl_mod_spi! {
    hal: crate::attiny167,
    sclk: hal::port::PA5,
    mosi: hal::port::PA4,
    miso: hal::port::PA2,
    cs: hal::port::PA6,
}

impl_mod_wdt! {
    hal: crate::attiny167,
    wdtcsr_name: wdtcr,
}

