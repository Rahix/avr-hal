#![no_std]

#[cfg(not(feature = "board-selected"))]
compile_error!(
    "This crate requires you to specify your target SparkFun board as a feature.

    Please select one of the following

    * pro-micro
    "
);

#[cfg(feature = "mcu-atmega")]
pub use atmega_hal as hal;
#[cfg(feature = "mcu-atmega")]
pub use atmega_hal::entry;
#[cfg(feature = "mcu-atmega")]
pub use atmega_hal::pac;

#[cfg(feature = "board-selected")]
pub use hal::Peripherals;

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
pub mod adc {
    pub use crate::hal::adc::{
        channel, AdcChannel, AdcOps, AdcSettings, Channel, ClockDivider, ReferenceVoltage,
    };

    /// Check the [`avr_hal_generic::adc::Adc`] documentation.
    pub type Adc = crate::hal::Adc<crate::DefaultClock>;
}
#[cfg(feature = "board-selected")]
pub use adc::Adc;

#[cfg(feature = "board-selected")]
pub mod i2c {
    pub use crate::hal::i2c::*;

    pub type I2c = crate::hal::i2c::I2c<crate::DefaultClock>;
}
#[cfg(feature = "board-selected")]
pub use i2c::I2c;

#[cfg(feature = "board-selected")]
pub mod spi {
    pub use crate::hal::spi::*;

    pub type Spi = crate::hal::spi::Spi;
}
#[cfg(feature = "board-selected")]
pub use spi::Spi;

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
    pub use crate::hal::usart::BaudrateExt as _;

    pub use ufmt::uWrite as _;
    pub use void::ResultVoidErrExt as _;
    pub use void::ResultVoidExt as _;
}

#[cfg(feature = "board-selected")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::with_mcu_pins($crate::hal::pins!($p))
    };
}

#[cfg(any(feature = "sparkfun-promicro"))]
#[macro_export]
macro_rules! default_serial {
    ($p:expr, $pins:expr, $baud:expr) => {
        $crate::Usart::new(
            $p.USART1,
            $pins.d0,
            $pins.d1.into_output(),
            $crate::hal::usart::BaudrateExt::into_baudrate($baud),
        )
    };
}
