#![no_std]

#[cfg(all(
    not(feature = "device-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "This crate requires you to specify your target chip as a feature.

    Please select one of the following

    * atmega168
    * atmega328p
    * atmega32u4
    * atmega1280
    * atmega2560
    "
);

/// Reexport of `atmega1280` from `avr-device`
#[cfg(feature = "atmega1280")]
pub use avr_device::atmega1280 as pac;
/// Reexport of `atmega168` from `avr-device`
#[cfg(feature = "atmega168")]
pub use avr_device::atmega168 as pac;
/// Reexport of `atmega2560` from `avr-device`
#[cfg(feature = "atmega2560")]
pub use avr_device::atmega2560 as pac;
/// Reexport of `atmega328p` from `avr-device`
#[cfg(feature = "atmega328p")]
pub use avr_device::atmega328p as pac;
/// Reexport of `atmega32u4` from `avr-device`
#[cfg(feature = "atmega32u4")]
pub use avr_device::atmega32u4 as pac;

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
    #[cfg(any(
        feature = "atmega168",
        feature = "atmega328p",
        feature = "atmega1280",
        feature = "atmega2560"
    ))]
    pub USART0: RawPeripheral<pac::USART0>,
    #[cfg(any(feature = "atmega32u4", feature = "atmega1280", feature = "atmega2560"))]
    pub USART1: RawPeripheral<pac::USART1>,
    #[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
    pub USART2: RawPeripheral<pac::USART2>,
    #[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
    pub USART3: RawPeripheral<pac::USART3>,
}

#[cfg(feature = "device-selected")]
impl Peripherals {
    fn new(dp: pac::Peripherals) -> Self {
        Self {
            #[cfg(any(feature = "atmega168", feature = "atmega328p"))]
            pins: Pins::new(dp.PORTB, dp.PORTC, dp.PORTD),
            #[cfg(feature = "atmega32u4")]
            pins: Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF),
            #[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
            pins: Pins::new(
                dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH,
                dp.PORTJ, dp.PORTK, dp.PORTL,
            ),

            #[cfg(any(
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega1280",
                feature = "atmega2560"
            ))]
            USART0: RawPeripheral(dp.USART0),
            #[cfg(any(feature = "atmega32u4", feature = "atmega1280", feature = "atmega2560"))]
            USART1: RawPeripheral(dp.USART1),
            #[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
            USART2: RawPeripheral(dp.USART2),
            #[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
            USART3: RawPeripheral(dp.USART3),
        }
    }

    pub fn take() -> Option<Self> {
        pac::Peripherals::take().map(Self::new)
    }
}
