pub use avr_device::attiny167 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::attiny167,
    references: {
        /// Voltage applied to AREF pin.
        Aref: |peripheral| {
            peripheral.amiscr.write(|w| w.arefen().set_bit());
            peripheral.admux.write(|w| w.refs().avcc());
        },
        /// Default reference voltage (default).
        AVcc: |peripheral| {
            peripheral.amiscr.write(|w| w.arefen().clear_bit());
            peripheral.admux.write(|w| w.refs().avcc());
        },
        /// Internal 1.1V reference.
        Internal1_1: |peripheral| {
            peripheral.amiscr.write(|w| w.arefen().clear_bit());
            peripheral.admux.write(|w| w.refs().internal_11());
        },
        /// Internal 2.56V reference.
        Internal2_56: |peripheral| {
            peripheral.amiscr.write(|w| w.arefen().clear_bit());
            peripheral.admux.write(|w| w.refs().internal_256());
        },
    },
    pins: {
        PA0: (hal::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
        PA1: (hal::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
        PA2: (hal::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
        PA3: (hal::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
        PA4: (hal::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
        PA5: (hal::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
        PA6: (hal::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
        PA7: (hal::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        PB5: (hal::pac::adc::admux::MUX_A::ADC8, didr1::adc8d),
        PB6: (hal::pac::adc::admux::MUX_A::ADC9, didr1::adc9d),
        PB7: (hal::pac::adc::admux::MUX_A::ADC10, didr1::adc10d),
    },
    channels: {
        AVcc_4: hal::pac::adc::admux::MUX_A::ADC_AVCC_4,
        Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
        Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        Temperature: hal::pac::adc::admux::MUX_A::TEMPSENS,
    },
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
            $crate::attiny167::port::Pins::new($p.PORTA, $p.PORTB)
        };
    }

    pub use attiny167_pins as pins;
}

impl_mod_spi! {
    hal: crate::attiny167,
    sclk: PA5,
    mosi: PA4,
    miso: PA2,
    cs: PA6,
}

impl_mod_wdt! {
    hal: crate::attiny167,
    wdtcsr_name: wdtcr,
}
