// Deprecated globals

#[cfg(all(
    // More than one MCU selected -> error
    all(
        feature = "attiny167",
        any(
            feature = "attiny2313",
            feature = "attiny84",
            feature = "attiny85",
            feature = "attiny88",
        )
    ),
    all(
        feature = "attiny2313",
        any(
            feature = "attiny167",
            feature = "attiny84",
            feature = "attiny85",
            feature = "attiny88",
        )
    ),
    all(
        feature = "attiny84",
        any(
            feature = "attiny167",
            feature = "attiny2313",
            feature = "attiny85",
            feature = "attiny88",
        )
    ),
    all(
        feature = "attiny85",
        any(
            feature = "attiny167",
            feature = "attiny2313",
            feature = "attiny84",
            feature = "attiny88",
        )
    ),
    all(
        feature = "attiny88",
        any(
            feature = "attiny167",
            feature = "attiny2313",
            feature = "attiny84",
            feature = "attiny85",
        )
    ),
))]
compile_error!(
    "When using globals, you cannot target multiple chips. Either choose exactly one chip feature, or add the `no-globals` feature."
);

#[cfg(feature = "attiny167")]
pub use crate::attiny167 as hal;

#[cfg(feature = "attiny2313")]
pub use crate::attiny2313 as hal;

#[cfg(feature = "attiny84")]
pub use crate::attiny84 as hal;

#[cfg(feature = "attiny85")]
pub use crate::attiny85 as hal;

#[cfg(feature = "attiny88")]
pub use crate::attiny88 as hal;

pub use hal::{eeprom, pac, pins, port, wdt, Hal as Attiny};
pub use {eeprom::Eeprom, pac::Peripherals, port::Pins, wdt::Wdt};

#[cfg(feature = "_peripheral-simple-pwm")]
pub use hal::simple_pwm;

#[cfg(feature = "_peripheral-adc")]
pub use crate::adc::Adc;
#[cfg(feature = "_peripheral-adc")]
pub use hal::adc;

#[cfg(feature = "_peripheral-spi")]
pub use crate::spi::Spi;
#[cfg(feature = "_peripheral-spi")]
pub use hal::spi;
