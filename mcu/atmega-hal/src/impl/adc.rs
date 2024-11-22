#![allow(unused_macros)]

macro_rules! impl_mod_adc {
    (
        hal: crate::$hal:ident,
        pins: {
            $($pin_name:ident: ($pin_channel:expr$(, $didr:ident::$didr_method:ident)?),)+
        },
        channels: {
            $(
                $(#[$channel_attr:meta])*
                $channel_name:ident: $channel_mux: expr,
            )*
        },
        impl!: $($impl_macro:ident)::* $({
            $($arg_name:ident: $arg_value:expr,)*
        })?,
    ) => {
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

            use avr_hal_generic::paste::paste;
            use crate::$hal as hal;

            #[allow(unused_imports)]
            use crate::r#impl::{impl_adc_admux, impl_adc_admux_adcsrb};

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
            #[allow(non_camel_case_types)]
            pub mod channel {
                $(
                    $(#[$channel_attr])*
                    pub struct $channel_name;
                )*
            }

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

            $($impl_macro)::+! {
                hal: crate::$hal,
                pins: {
                    $($pin_name: ($pin_channel$(, $didr::$didr_method)?),)+
                },
                channels: {
                    $(
                        $(#[$channel_attr])*
                        $channel_name: $channel_mux,
                    )*
                },
                $($($arg_name: $arg_value,)*)?
            }
        }

        pub use adc::Adc;
    }
}
pub(crate) use impl_mod_adc;

macro_rules! impl_adc_admux {
    (
        hal: crate::$hal:ident,
        pins: {
            $($pin_name:ident: ($pin_channel:expr$(, $didr:ident::$didr_method:ident)?),)+
        },
        channels: {
            $(
                $(#[$channel_attr:meta])*
                $channel_name:ident: $channel_mux: expr,
            )*
        },
        $(impl!: $($impl_macro:ident)::* $({
            $($arg_name:ident: $arg_value:expr,)*
        })?,)?
    ) => {
        paste! {
            avr_hal_generic::impl_adc! {
                hal: crate::$hal::Hal,
                peripheral: crate::$hal::pac::ADC,
                settings: crate::$hal::adc::AdcSettings,
                apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
                channel_id: crate::$hal::pac::adc::admux::MUX_A,
                set_channel: |peripheral, id| {
                    peripheral.admux.modify(|_, w| w.mux().variant(id));
                },
                pins: {
                    $(hal::port::[<$pin_name>]: ($pin_channel $(, $didr::$didr_method)?),)*
                },
                channels: {
                    $(
                        $(#[$channel_attr])*
                        channel::$channel_name: $channel_mux,
                    )*
                },
            }
        }
    }
}
pub(crate) use impl_adc_admux;

macro_rules! impl_adc_admux_adcsrb {
    (
        hal: crate::$hal:ident,
        pins: {
            $($pin_name:ident: ($pin_channel:expr$(, $didr:ident::$didr_method:ident)?),)+
        },
        channels: {
            $(
                $(#[$channel_attr:meta])*
                $channel_name:ident: $channel_mux: expr,
            )*
        },
        $(impl!: $($impl_macro:ident)::* $({
            $($arg_name:ident: $arg_value:expr,)*
        })?,)?
    ) => {
        paste! {
            avr_hal_generic::impl_adc! {
                hal: crate::$hal::Hal,
                peripheral: crate::$hal::pac::ADC,
                settings: crate::$hal::adc::AdcSettings,
                apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
                channel_id: u8,
                set_channel: |peripheral, id| {
                    peripheral.admux.modify(|_, w| w.mux().bits(id & 0x1f));
                    peripheral.adcsrb.modify(|_, w| w.mux5().bit(id & 0x20 != 0));
                },
                pins: {
                    $(hal::port::[<$pin_name>]: ($pin_channel $(, $didr::$didr_method)?),)*
                },
                channels: {
                    $(
                        $(#[$channel_attr])*
                        channel::$channel_name: $channel_mux,
                    )*
                },
            }
        }
    }
}
pub(crate) use impl_adc_admux_adcsrb;
