macro_rules! impl_mod_adc {
    ($($mod:item)*) => {
        pub mod adc {
            //! Analog-to-Digital Converter
            //!
            //! # Example
            //!
            //! For full source code, please refer to the ATmega ADC example:
            //! [`atmega2560-adc.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-adc.rs)
            //!
            //! ```
            //! let dp = attiny_hal::Peripherals::take().unwrap();
            //! let pins = attiny_hal::pins!(dp);
            //!
            //! let mut adc = Adc::new(dp.ADC, Default::default());
            //!
            //! let channels: [attiny_hal::adc::Channel; 4] = [
            //!     pins.pa0.into_analog_input(&mut adc).into_channel(),
            //!     pins.pa1.into_analog_input(&mut adc).into_channel(),
            //!     pins.pa2.into_analog_input(&mut adc).into_channel(),
            //!     pins.pa3.into_analog_input(&mut adc).into_channel(),
            //! ];
            //!
            //! for (index, channel) in channels.iter().enumerate() {
            //!     let value = adc.read_blocking(channel);
            //!     ufmt::uwrite!(&mut serial, "CH{}: {} ", index, value).unwrap();
            //! }
            //! ```

            use crate::r#impl::{impl_adc_reference_voltage,impl_adc_channels,impl_adc_peripheral};

            $($mod)*
        }
        pub use adc::Adc;
    }
}
pub(crate) use impl_mod_adc;

macro_rules! impl_adc_reference_voltage {
    ($enum:item) => {
        /// Select the voltage reference for the ADC peripheral
        ///
        /// The internal voltage reference options may not be used if an external reference voltage is
        /// being applied to the AREF pin.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        $enum
    }
}
pub(crate) use impl_adc_reference_voltage;

macro_rules! impl_adc_channels {
    ($($channel:item)*) => {
        /// Additional channels
        ///
        /// Some channels are not directly connected to pins.  This module provides types which can be used
        /// to access them.
        ///
        /// # Example
        /// ```
        /// let dp = attiny_hal::Peripherals::take().unwrap();
        /// let mut adc = attiny_hal::Adc::new(dp.ADC, Default::default());
        ///
        /// let value = adc.read_blocking(&channel::Vbg);
        /// ```
        #[allow(non_camel_case_types)]
        pub mod channel {
            $($channel)*
        }
    }
}
pub(crate) use impl_adc_channels;

macro_rules! impl_adc_peripheral {
    (
        pac: $($pac:ident)::+,
        hal: $($hal:ident)::+ $(,)?
    ) => {
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

        /// Check the [`avr_hal_generic::adc::Adc`] documentation.
        pub type Adc<CLOCK> =
            avr_hal_generic::adc::Adc<$($hal)::+, $($pac)::+::ADC, CLOCK>;

        /// Check the [`avr_hal_generic::adc::Channel`] documentation.
        pub type Channel = avr_hal_generic::adc::Channel<$($hal)::+, $($pac)::+::ADC>;

        fn apply_clock(peripheral: &$($pac)::+::ADC, settings: AdcSettings) {
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
        }
    };
}

pub(crate) use impl_adc_peripheral;
