#![allow(unused_macros)]

macro_rules! impl_mod_adc {
    ($($mod:item)*) => {
        pub mod adc {
            //! Analog-to-Digital Converter
            //!
            //! # Example
            //!
            //! Complete example source code can be found in the repository:
            //! [`atmega2560-adc.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-adc.rs)
            //!
            //! ```
            //! let dp = atmega_hal::Peripherals::take().unwrap();
            //! let pins = atmega_hal::pins!(dp);
            //!
            //! let mut adc = Adc::new(dp.ADC, Default::default());
            //!
            //! let channels: [atmega_hal::adc::Channel; 4] = [
            //!     pins.pf0.into_analog_input(&mut adc).into_channel(),
            //!     pins.pf1.into_analog_input(&mut adc).into_channel(),
            //!     pins.pf2.into_analog_input(&mut adc).into_channel(),
            //!     pins.pf3.into_analog_input(&mut adc).into_channel(),
            //! ];
            //!
            //! for (index, channel) in channels.iter().enumerate() {
            //!     let value = adc.read_blocking(channel);
            //!     ufmt::uwrite!(&mut serial, "CH{}: {} ", index, value).unwrap();
            //! }
            //! ```

            #[allow(unused_imports)]
            use crate::r#impl::{impl_adc, impl_adc_channels,impl_adc_channels_extra_temp,impl_adc_channels_extra,impl_adc_channels_temp};

            $($mod)*
        }

        pub use adc::Adc;
    }
}
pub(crate) use impl_mod_adc;

/// Additional channels
///
/// Some channels are not directly connected to pins.  This module provides types which can be used
/// to access them.
///
/// # Example
/// ```
/// let dp = atmega_hal::Peripherals::take().unwrap();
/// let mut adc = atmega_hal::Adc::new(dp.ADC, Default::default());
///
/// let value = adc.read_blocking(&channel::Vbg);
/// ```

macro_rules! impl_adc_channels {
    () => {
        pub mod channel {
            pub struct Vbg;
            pub struct Gnd;
        }
    };
}
pub(crate) use impl_adc_channels;

macro_rules! impl_adc_channels_temp {
    () => {
        pub mod channel {
            pub struct Vbg;
            pub struct Gnd;
            pub struct Temperature;
        }
    };
}
pub(crate) use impl_adc_channels_temp;

macro_rules! impl_adc_channels_extra {
    () => {
        pub mod channel {
            #[cfg(feature = "enable-extra-adc")]
            pub struct ADC6;
            #[cfg(feature = "enable-extra-adc")]
            pub struct ADC7;
            pub struct Vbg;
            pub struct Gnd;
        }
    };
}
pub(crate) use impl_adc_channels_extra;

macro_rules! impl_adc_channels_extra_temp {
    () => {
        pub mod channel {
            #[cfg(feature = "enable-extra-adc")]
            pub struct ADC6;
            #[cfg(feature = "enable-extra-adc")]
            pub struct ADC7;
            pub struct Vbg;
            pub struct Gnd;
            pub struct Temperature;
        }
    };
}
pub(crate) use impl_adc_channels_extra_temp;

macro_rules! impl_adc {
    () => {
        pub use avr_hal_generic::adc::{AdcChannel, AdcOps, ClockDivider};

        impl Default for ReferenceVoltage {
            fn default() -> Self {
                Self::AVcc
            }
        }

        /// Configuration for the ADC peripheral.
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
        pub struct AdcSettings {
            pub clock_divider: ClockDivider,
            pub ref_voltage: ReferenceVoltage,
        }

        /// Select the voltage reference for the ADC peripheral
        ///
        /// The internal voltage reference options may not be used if an external reference voltage is
        /// being applied to the AREF pin.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        pub enum ReferenceVoltage {
            /// Voltage applied to AREF pin.
            Aref,
            /// Default reference voltage (default).
            AVcc,
            /// Internal reference voltage.
            Internal,
        }

        /// Check the [`avr_hal_generic::adc::Adc`] documentation.
        pub type Adc<CLOCK> = avr_hal_generic::adc::Adc<hal::Hal, hal::pac::ADC, CLOCK>;

        /// Check the [`avr_hal_generic::adc::Channel`] documentation.
        pub type Channel = avr_hal_generic::adc::Channel<hal::Hal, hal::pac::ADC>;

        fn apply_settings(peripheral: &hal::pac::ADC, settings: AdcSettings) {
            peripheral.adcsra.write(|w| {
                w.aden().set_bit();
                match settings.clock_divider {
                    ClockDivider::Factor2 => w.adps().prescaler_2(),
                    ClockDivider::Factor4 => w.adps().prescaler_4(),
                    ClockDivider::Factor8 => w.adps().prescaler_8(),
                    ClockDivider::Factor16 => w.adps().prescaler_16(),
                    ClockDivider::Factor32 => w.adps().prescaler_32(),
                    ClockDivider::Factor64 => w.adps().prescaler_64(),
                    ClockDivider::Factor128 => w.adps().prescaler_128(),
                }
            });
            peripheral.admux.write(|w| match settings.ref_voltage {
                ReferenceVoltage::Aref => w.refs().aref(),
                ReferenceVoltage::AVcc => w.refs().avcc(),
                ReferenceVoltage::Internal => w.refs().internal(),
            });
        }
    };
}
pub(crate) use impl_adc;
