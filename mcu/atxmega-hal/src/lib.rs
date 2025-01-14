#![no_std]

#[cfg(all(
    not(feature = "device-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "This crate requires you to specify your target chip as a feature.

    Please select one of the following

    * atmega4809
    "
);

/// Reexport of `atmega4809` from `avr-device`
#[cfg(feature = "atmega4809")]
pub use avr_device::atmega4809 as pac;

#[cfg(feature = "device-selected")]
pub use pac::Peripherals;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;
pub use avr_hal_generic::prelude;

#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub mod usart;
#[cfg(feature = "device-selected")]
pub use port::Pins;
#[cfg(feature = "device-selected")]
pub use usart::Usart;

pub struct Atxmega;

#[cfg(feature = "device-selected")]
pub mod adc;
#[cfg(feature = "device-selected")]
pub use adc::Adc;

#[cfg(feature = "device-selected")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new(
            &$p.PORTMUX, &$p.CLKCTRL, &$p.CPU, $p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF,
        )
    };
}

#[cfg(feature = "device-selected")]
pub mod simple_pwm;
