#![no_std]

#[cfg(not(feature = "board-selected"))]
compile_error!(
    "This crate requires you to specify your target Arduino board as a feature.

    Please select one of the following

    * arduino-uno
    * arduino-leonardo
    * arduino-mega2560
    "
);

#[cfg(feature = "mcu-atmega")]
pub use atmega_hal as hal;
#[cfg(feature = "mcu-atmega")]
pub use atmega_hal::entry;
#[cfg(feature = "mcu-atmega")]
pub use atmega_hal::pac;

#[cfg(feature = "board-selected")]
pub mod clock;
#[cfg(feature = "board-selected")]
pub use clock::default::DefaultClock;

#[cfg(feature = "board-selected")]
mod delay;
#[cfg(feature = "board-selected")]
pub use delay::{delay_ms, delay_us, Delay};

#[cfg(feature = "board-selected")]
pub mod port;
#[cfg(feature = "board-selected")]
pub use port::Pins;

#[cfg(feature = "board-selected")]
pub mod usart {
    pub use crate::hal::usart::{Baudrate, UsartOps};

    pub type Usart<USART, RX, TX> = crate::hal::usart::Usart<USART, RX, TX, crate::DefaultClock>;
    pub type UsartWriter<USART, RX, TX> =
        crate::hal::usart::UsartWriter<USART, RX, TX, crate::DefaultClock>;
    pub type UsartReader<USART, RX, TX> =
        crate::hal::usart::UsartReader<USART, RX, TX, crate::DefaultClock>;
}
#[cfg(feature = "board-selected")]
pub use usart::Usart;

#[cfg(feature = "board-selected")]
pub mod prelude {
    cfg_if::cfg_if! {
        if #[cfg(feature = "arduino-uno")] {
            pub use crate::hal::usart::BaudrateArduinoExt as _;
        } else {
            pub use crate::hal::usart::BaudrateExt as _;
        }
    }

    pub use ufmt::uWrite as _;
    pub use void::ResultVoidErrExt as _;
    pub use void::ResultVoidExt as _;
}

#[allow(non_snake_case)]
#[cfg(feature = "board-selected")]
pub struct Peripherals {
    pub pins: Pins,
    #[cfg(any(feature = "arduino-uno", feature = "arduino-mega2560"))]
    pub USART0: hal::RawPeripheral<pac::USART0>,
    #[cfg(any(feature = "arduino-leonardo", feature = "arduino-mega2560"))]
    pub USART1: hal::RawPeripheral<pac::USART1>,
    #[cfg(feature = "arduino-mega2560")]
    pub USART2: hal::RawPeripheral<pac::USART2>,
    #[cfg(feature = "arduino-mega2560")]
    pub USART3: hal::RawPeripheral<pac::USART3>,
}

#[cfg(feature = "board-selected")]
impl Peripherals {
    fn new(dp: hal::Peripherals) -> Self {
        Self {
            #[cfg(feature = "atmega-hal")]
            pins: Pins::with_mcu_pins(dp.pins),

            #[cfg(any(feature = "arduino-uno", feature = "arduino-mega2560"))]
            USART0: dp.USART0,
            #[cfg(any(feature = "arduino-leonardo", feature = "arduino-mega2560"))]
            USART1: dp.USART1,
            #[cfg(feature = "arduino-mega2560")]
            USART2: dp.USART2,
            #[cfg(feature = "arduino-mega2560")]
            USART3: dp.USART3,
        }
    }

    pub fn take() -> Option<Self> {
        hal::Peripherals::take().map(Self::new)
    }
}
