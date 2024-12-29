macro_rules! impl_mod_adc {
    (
        hal: crate::$hal:ident,
        references: {
            $(
                $(#[$reference_meta:meta])*
                $reference_name:ident: |$peripheral_var:ident| $apply_reference:block,
            )*
        },
        pins: {
            $($pin_name:ident: ($pin_channel:expr$(, $didr:ident::$didr_method:ident)?),)+
        },
        channels: {
            $($channel_name:ident: $channel_mux: expr,)*
        },
    ) => {
        pub mod adc {
            //! Analog-to-Digital Converter
            //!
            //! For full source code, please refer to the ATmega ADC example:
            //! [`atmega2560-adc.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-adc.rs)
            //!
            //! # Example: Read pins using `analog_read()`
            //!
            //! ```no_run
            #![doc = concat!("use attiny_hal::", stringify!($hal), " as hal;")]
            //!
            //! let dp = hal::Peripherals::take().unwrap();
            //! let pins = hal::pins!(dp);
            //!
            //! type Clock = avr_hal_generic::clock::MHz16;
            //! let mut adc = hal::Adc::<Clock>::new(dp.ADC, Default::default());
            //! 
            $(
                #![doc = paste!{ concat!(
                    "let ", stringify!([< input_ $pin_name:lower >]), " = pins.", stringify!([< $pin_name:lower >]), ".into_analog_input(&mut adc);\n",
                    "let ", stringify!([< value_ $pin_name:lower >]), " = ", stringify!([< input_ $pin_name:lower >]), ".analog_read(&mut adc);\n\n"
                )}]
            )*
            //! ```
            //!
            //! # Example: Read channels (including pins) using `read_blocking()`
            //!
            //! ```no_run
            #![doc = concat!("use attiny_hal::", stringify!($hal), " as hal;")]
            //!
            //! let dp = hal::Peripherals::take().unwrap();
            //! let pins = hal::pins!(dp);
            //!
            //! type Clock = avr_hal_generic::clock::MHz16;
            //! let mut adc = hal::Adc::<Clock>::new(dp.ADC, Default::default());
            $(
                #![doc = paste!{ concat!(
                    "let ", stringify!([< channel_ $pin_name:lower >]), " = pins.", stringify!([< $pin_name:lower >]), ".into_analog_input(&mut adc).into_channel();\n",
                    "let ", stringify!([< value_ $pin_name:lower >]), " = adc.read_blocking(&", stringify!([< channel_ $pin_name:lower >]), ");\n\n"
                ) }]
            )*
            $(
                #![doc = paste!{ concat!(
                    "let ", stringify!([< value_ $channel_name:lower >]), " = adc.read_blocking(&hal::adc::channel::", stringify!([< $channel_name >]), ");\n\n"
                ) }]
            )*
            //! ```

            use avr_hal_generic::paste::paste;
            use crate::$hal as hal;

            /// Select the voltage reference for the ADC peripheral
            ///
            /// The internal voltage reference options may not be used if an external reference voltage is
            /// being applied to the AREF pin.
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[repr(u8)]
            pub enum ReferenceVoltage {
                $(
                    $(#[$reference_meta])*
                    $reference_name,
                )*
            }
    
            /// Additional channels
            ///
            /// Some channels are not directly connected to pins.  This module provides types which can be used
            /// to access them.
            #[allow(non_camel_case_types)]
            pub mod channel {
                $(
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

            /// Check the [`avr_hal_generic::adc::Adc`] documentation.
            pub type Adc<CLOCK> =
                avr_hal_generic::adc::Adc<crate::$hal::Hal, crate::$hal::pac::ADC, CLOCK>;

            /// Check the [`avr_hal_generic::adc::Channel`] documentation.
            pub type Channel = avr_hal_generic::adc::Channel<crate::$hal::Hal, crate::$hal::pac::ADC>;

            fn apply_clock(peripheral: &crate::$hal::pac::ADC, settings: AdcSettings) {
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

            paste! {
                avr_hal_generic::impl_adc! {
                    hal: crate::$hal::Hal,
                    peripheral: crate::$hal::pac::ADC,
                    settings: crate::$hal::adc::AdcSettings,
                    apply_settings: |peripheral, settings| {
                        apply_clock(peripheral, settings);
                        match settings.ref_voltage {
                            $(
                                ReferenceVoltage::$reference_name => {
                                    let $peripheral_var = peripheral;
                                    $apply_reference
                                },
                            )*
                        }
                    },
                    channel_id: crate::$hal::pac::adc::admux::MUX_A,
                    set_channel: |peripheral, id| {
                        peripheral.admux.modify(|_, w| w.mux().variant(id));
                    },
                    pins: {
                        $(hal::port::[<$pin_name>]: ($pin_channel $(, $didr::$didr_method)?),)*
                    },
                    channels: {
                        $(channel::$channel_name: $channel_mux,)*
                    },
                }
            }
    
        }
        pub use adc::Adc;
    }
}
pub(crate) use impl_mod_adc;
