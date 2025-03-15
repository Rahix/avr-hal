// Deprecated globals

#[cfg(
    any(
        // More than one MCU selected -> error
        all(
            feature = "atmega48p",
            any(
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega164pa",
            any(
                feature = "atmega48p",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega168",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega328p",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega328pb",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega32a",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega32u4",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega2560",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega128a",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega1280",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1284p",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega1284p",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega8",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega8",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega88p",
            )
        ),
        all(
            feature = "atmega88p",
            any(
                feature = "atmega48p",
                feature = "atmega164pa",
                feature = "atmega168",
                feature = "atmega328p",
                feature = "atmega328pb",
                feature = "atmega32a",
                feature = "atmega32u4",
                feature = "atmega2560",
                feature = "atmega128a",
                feature = "atmega1280",
                feature = "atmega1284p",
                feature = "atmega8",
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
    * atmega88p-no-deprecated-globals instead of atmega88p
    "
);

#[cfg(feature = "atmega48p")]
pub use crate::atmega48p as hal;

#[cfg(feature = "atmega164pa")]
pub use crate::atmega164pa as hal;

#[cfg(feature = "atmega168")]
pub use crate::atmega168 as hal;

#[cfg(feature = "atmega328p")]
pub use crate::atmega328p as hal;

#[cfg(feature = "atmega328pb")]
pub use crate::atmega328pb as hal;

#[cfg(feature = "atmega32a")]
pub use crate::atmega32a as hal;

#[cfg(feature = "atmega32u4")]
pub use crate::atmega32u4 as hal;

#[cfg(feature = "atmega2560")]
pub use crate::atmega2560 as hal;

#[cfg(feature = "atmega128a")]
pub use crate::atmega128a as hal;

#[cfg(feature = "atmega1280")]
pub use crate::atmega1280 as hal;

#[cfg(feature = "atmega1284p")]
pub use crate::atmega1284p as hal;

#[cfg(feature = "atmega8")]
pub use crate::atmega8 as hal;

#[cfg(feature = "atmega88p")]
pub use crate::atmega88p as hal;

pub use hal::{adc, eeprom, i2c, pac, pins, port, wdt, Hal as Atmega};
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
