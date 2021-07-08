#![no_std]

// Expose hal & pac crates
pub use atmega328p_hal as hal;
pub use crate::hal::pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use crate::hal::entry;

pub use crate::pac::Peripherals;

pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::hal::usart::BaudrateArduinoExt as _;
}

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;

/// Wait (busy spin) for `ms` milliseconds
pub fn delay_ms(ms: u16) {
    use prelude::*;

    Delay::new().delay_ms(ms)
}

/// Wait (busy spin) for `us` microseconds
pub fn delay_us(us: u16) {
    use prelude::*;

    Delay::new().delay_us(us)
}

mod pins;
pub use crate::pins::*;

pub mod spi {
    pub use atmega328p_hal::spi::*;
}

pub mod adc {
    pub use atmega328p_hal::adc::*;
}

pub mod pwm {
    pub use atmega328p_hal::pwm::*;
}

pub type Serial<IMODE> = hal::usart::Usart0<hal::clock::MHz16, IMODE>;

pub type I2cMaster<M> = hal::i2c::I2cMaster<hal::clock::MHz16, M>;

pub mod wdt {
    pub use atmega328p_hal::wdt::*;
}

