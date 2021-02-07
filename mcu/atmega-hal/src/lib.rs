#![no_std]

#[cfg(all(
    not(feature = "device-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "This crate requires you to specify your target chip as a feature.

    Please select one of the following

    * atmega328p
    "
);

/// Reexport of `atmega328p` from `avr-device`
#[cfg(feature = "atmega328p")]
pub use avr_device::atmega328p as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;

#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub use port::Pins;

#[cfg(feature = "device-selected")]
pub mod usart;
#[cfg(feature = "device-selected")]
pub use usart::Usart;

pub struct RawPeripheral<P>(pub(crate) P);

#[allow(non_snake_case)]
#[cfg(feature = "device-selected")]
pub struct Peripherals {
    pub pins: Pins,
    #[cfg(feature = "atmega328p")]
    pub USART0: RawPeripheral<pac::USART0>,
}

#[cfg(feature = "device-selected")]
impl Peripherals {
    fn new(dp: pac::Peripherals) -> Self {
        Self {
            #[cfg(feature = "atmega328p")]
            pins: Pins::new(dp.PORTB, dp.PORTC, dp.PORTD),
            #[cfg(feature = "atmega328p")]
            USART0: RawPeripheral(dp.USART0),
        }
    }

    pub fn take() -> Option<Self> {
        pac::Peripherals::take().map(Self::new)
    }
}
