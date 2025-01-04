// Deprecated globals
#[cfg(
    any(
        // More than one board selected -> error
        all(
            feature = "_board-arduino-diecimila",
            any(
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-arduino-leonardo",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-arduino-mega2560",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-arduino-mega1280",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-arduino-nano",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-arduino-uno",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-trinket-pro",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-sparkfun-promicro",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-sparkfun-promini-3v3",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-sparkfun-promini-5v",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-trinket",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-trinket",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-nano168",
            )
        ),
        all(
            feature = "_board-nano168",
            any(
                feature = "_board-arduino-diecimila",
                feature = "_board-arduino-leonardo",
                feature = "_board-arduino-mega2560",
                feature = "_board-arduino-mega1280",
                feature = "_board-arduino-nano",
                feature = "_board-arduino-uno",
                feature = "_board-trinket-pro",
                feature = "_board-sparkfun-promicro",
                feature = "_board-sparkfun-promini-3v3",
                feature = "_board-sparkfun-promini-5v",
                feature = "_board-trinket",
            )
        )
    )
)]
compile_error!(
    "When using deprecated globals (default in arduino-hal 0.1.x), you cannot target multiple boards.

    To target multiple boards, turn off deprecated globals by using the following features

    * arduino-diecimila-no-deprecated-globals instead of arduino-diecimila
    * arduino-leonardo-no-deprecated-globals instead of arduino-leonardo
    * arduino-mega2560-no-deprecated-globals instead of arduino-mega2560
    * arduino-mega1280-no-deprecated-globals instead of arduino-mega1280
    * arduino-nano-no-deprecated-globals instead of arduino-nano
    * arduino-uno-no-deprecated-globals instead of arduino-uno
    * trinket-pro-no-deprecated-globals instead of trinket-pro
    * sparkfun-promicro-no-deprecated-globals instead of sparkfun-promicro
    * sparkfun-promini-3v3-no-deprecated-globals instead of sparkfun-promini-3v3
    * sparkfun-promini-5v-no-deprecated-globals instead of sparkfun-promini-5v
    * trinket-no-deprecated-globals instead of trinket
    * nano168-no-deprecated-globals instead of nano168
    "
);

#[cfg(feature = "_board-trinket")]
pub use crate::adafruit::trinket as board;
#[cfg(feature = "_board-trinket-pro")]
pub use crate::adafruit::trinket_pro as board;
#[cfg(feature = "_board-arduino-diecimila")]
pub use crate::arduino::diecimila as board;
#[cfg(feature = "_board-arduino-leonardo")]
pub use crate::arduino::leonardo as board;
#[cfg(feature = "_board-arduino-mega1280")]
pub use crate::arduino::mega1280 as board;
#[cfg(feature = "_board-arduino-mega2560")]
pub use crate::arduino::mega2560 as board;
#[cfg(feature = "_board-nano168")]
pub use crate::arduino::nano_v2 as board;
#[cfg(feature = "_board-arduino-nano")]
pub use crate::arduino::nano_v3 as board;
#[cfg(feature = "_board-arduino-uno")]
pub use crate::arduino::uno as board;
#[cfg(feature = "_board-sparkfun-promicro")]
pub use crate::sparkfun::pro_micro as board;
#[cfg(feature = "_board-sparkfun-promini-3v3")]
pub use crate::sparkfun::pro_mini_3v3 as board;
#[cfg(feature = "_board-sparkfun-promini-5v")]
pub use crate::sparkfun::pro_mini_5v as board;

#[cfg(feature = "_mcu-atmega")]
pub use atmega_hal as hal;
#[cfg(feature = "_mcu-attiny")]
pub use attiny_hal as hal;

pub use board::{
    clock,
    clock::DefaultClock,
    delay,
    delay::{delay_ms, delay_us, Delay},
    eeprom,
    eeprom::Eeprom,
    hal::Peripherals,
    pac,
    port::{Pins,pins},
    simple_pwm,
};

pub mod port {
    pub use super::board::port::*;

    #[cfg(feature = "_board-arduino-diecimila")]
    pub use super::board::port as diecimila;
    #[cfg(feature = "_board-arduino-leonardo")]
    pub use super::board::port as leonardo;
    #[cfg(any(
        feature = "_board-arduino-mega2560",
        feature = "_board-arduino-mega1280"
    ))]
    pub use super::board::port as mega;
    #[cfg(any(
        feature = "_board-arduino-nano",
        feature = "_board-arduino-uno",
        feature = "_board-nano168",
        feature = "_board-sparkfun-promini-3v3",
        feature = "_board-sparkfun-promini-5v"
    ))]
    pub use super::board::port as uno;
    #[cfg(feature = "_board-sparkfun-promicro")]
    pub use super::board::port as promicro;
    #[cfg(feature = "_board-trinket-pro")]
    pub use super::board::port as trinket_pro;
    #[cfg(feature = "_board-trinket")]
    pub use super::board::port as trinket;
}

#[cfg(feature = "_mcu-atmega")]
pub use board::{adc, adc::Adc, i2c, i2c::I2c, prelude, spi, spi::Spi, usart, usart::Usart};

#[cfg(feature = "_default-serial")]
pub use board::{default_serial};

