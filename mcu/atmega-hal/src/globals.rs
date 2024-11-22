// Deprecated globals

#[cfg(
    any(
        // More than one MCU selected -> error
        all(
            feature = "_mcu-atmega48p",
            any(
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega164pa",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega168",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega328p",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega328pb",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega32a",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega32u4",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega2560",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega128a",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega1280",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1284p",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega1284p",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega8",
            )
        ),
        all(
            feature = "_mcu-atmega8",
            any(
                feature = "_mcu-atmega48p",
                feature = "_mcu-atmega164pa",
                feature = "_mcu-atmega168",
                feature = "_mcu-atmega328p",
                feature = "_mcu-atmega328pb",
                feature = "_mcu-atmega32a",
                feature = "_mcu-atmega32u4",
                feature = "_mcu-atmega2560",
                feature = "_mcu-atmega128a",
                feature = "_mcu-atmega1280",
                feature = "_mcu-atmega1284p",
            )
        ),
    )
)]
compile_error!(
    "When using deprecated globals (default in atmega-hal 0.1.x), you cannot target multiple chips.

    To target multiple chips, turn off deprecated globals by using the following features

    * atmega48p-no-deprecated-globals instead of atmega48p
    * atmega164pa-no-deprecated-globals instead of atmega164pa
    * atmega168-no-deprecated-globals instead of atmega168
    * atmega328p-no-deprecated-globals instead of atmega328p
    * atmega328pb-no-deprecated-globals instead of atmega328pb
    * atmega32a-no-deprecated-globals instead of atmega32a
    * atmega32u4-no-deprecated-globals instead of atmega32u4
    * atmega2560-no-deprecated-globals instead of atmega2560
    * atmega128a-no-deprecated-globals instead of atmega128a
    * atmega1280-no-deprecated-globals instead of atmega1280
    * atmega1284p-no-deprecated-globals instead of atmega1284p
    * atmega8-no-deprecated-globals instead of atmega8
    "
);

#[cfg(feature = "_mcu-atmega48p")]
pub use crate::atmega48p as hal;

#[cfg(feature = "_mcu-atmega164pa")]
pub use crate::atmega164pa as hal;

#[cfg(feature = "_mcu-atmega168")]
pub use crate::atmega168 as hal;

#[cfg(feature = "_mcu-atmega328p")]
pub use crate::atmega328p as hal;

#[cfg(feature = "_mcu-atmega328pb")]
pub use crate::atmega328pb as hal;

#[cfg(feature = "_mcu-atmega32a")]
pub use crate::atmega32a as hal;

#[cfg(feature = "_mcu-atmega32u4")]
pub use crate::atmega32u4 as hal;

#[cfg(feature = "_mcu-atmega2560")]
pub use crate::atmega2560 as hal;

#[cfg(feature = "_mcu-atmega128a")]
pub use crate::atmega128a as hal;

#[cfg(feature = "_mcu-atmega1280")]
pub use crate::atmega1280 as hal;

#[cfg(feature = "_mcu-atmega1284p")]
pub use crate::atmega1284p as hal;

#[cfg(feature = "_mcu-atmega8")]
pub use crate::atmega8 as hal;

pub use hal::{adc, eeprom, i2c, pac, port, wdt, Hal as Atmega, pins};
pub use {adc::Adc, eeprom::Eeprom, i2c::I2c, pac::Peripherals, port::Pins, wdt::Wdt};

#[cfg(feature = "_peripheral-simple-pwm")]
pub use hal::simple_pwm;

#[cfg(feature = "_peripheral-spi")]
pub use crate::spi::Spi;
#[cfg(feature = "_peripheral-spi")]
pub use hal::spi;

#[cfg(feature = "_peripheral-usart")]
pub use hal::usart;
#[cfg(feature = "_peripheral-usart")]
pub use hal::usart::Usart;

