#[cfg(feature = "_mcu-atmega")]
macro_rules! impl_adc_atmega {
    (
        board: $($board:ident)::+  $(,)?
    ) => {
        pub use $($board)::+::hal::adc::{
            channel, AdcChannel, AdcOps, AdcSettings, Channel, ClockDivider, ReferenceVoltage,
        };

        /// Check the [`avr_hal_generic::adc::Adc`] documentation.
        pub type Adc = $($board)::+::hal::Adc<$($board)::+::clock::DefaultClock>;


}

}

#[cfg(feature = "_mcu-atmega")]
pub(crate) use impl_adc_atmega;
