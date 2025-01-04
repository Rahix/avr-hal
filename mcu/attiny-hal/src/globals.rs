// Deprecated globals

#[cfg(all(
    // More than one MCU selected -> error
    all(
        feature = "_mcu-attiny167",
        any(
            feature = "_mcu-attiny2313",
            feature = "_mcu-attiny84",
            feature = "_mcu-attiny85",
            feature = "_mcu-attiny88",
        )
    ),
    all(
        feature = "_mcu-attiny2313",
        any(
            feature = "_mcu-attiny167",
            feature = "_mcu-attiny84",
            feature = "_mcu-attiny85",
            feature = "_mcu-attiny88",
        )
    ),
    all(
        feature = "_mcu-attiny84",
        any(
            feature = "_mcu-attiny167",
            feature = "_mcu-attiny2313",
            feature = "_mcu-attiny85",
            feature = "_mcu-attiny88",
        )
    ),
    all(
        feature = "_mcu-attiny85",
        any(
            feature = "_mcu-attiny167",
            feature = "_mcu-attiny2313",
            feature = "_mcu-attiny84",
            feature = "_mcu-attiny88",
        )
    ),
    all(
        feature = "_mcu-attiny88",
        any(
            feature = "_mcu-attiny167",
            feature = "_mcu-attiny2313",
            feature = "_mcu-attiny84",
            feature = "_mcu-attiny85",
        )
    ),
))]
compile_error!(
    "When using deprecated globals (default in attiny-hal 0.1.x), you cannot target multiple chips.

    To target multiple chips, turn off deprecated globals by using the following features

    * attiny84-no-deprecated-globals instead of attiny84
    * attiny85-no-deprecated-globals instead of attiny85
    * attiny88-no-deprecated-globals instead of attiny88
    * attiny167-no-deprecated-globals instead of attiny167
    * attiny2313-no-deprecated-globals instead of attiny2313
    "
);

#[cfg(feature = "_mcu-attiny167")]
pub use crate::attiny167 as hal;

#[cfg(feature = "_mcu-attiny2313")]
pub use crate::attiny2313 as hal;

#[cfg(feature = "_mcu-attiny84")]
pub use crate::attiny84 as hal;

#[cfg(feature = "_mcu-attiny85")]
pub use crate::attiny85 as hal;

#[cfg(feature = "_mcu-attiny88")]
pub use crate::attiny88 as hal;

pub use hal::{eeprom, pac, port, wdt, Hal as Attiny, pins};
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
