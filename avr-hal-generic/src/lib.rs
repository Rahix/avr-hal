#![no_std]
#![cfg_attr(avr_hal_asm_macro, feature(asm_experimental_arch))]
#![cfg_attr(not(avr_hal_asm_macro), feature(llvm_asm))]

pub use embedded_hal as hal;
pub use embedded_hal_v0 as hal_v0;

#[doc(hidden)]
pub use avr_device;
#[doc(hidden)]
pub use nb;
#[doc(hidden)]
pub use paste;

pub mod adc;
pub mod clock;
pub mod delay;
pub mod eeprom;
pub mod i2c;
pub mod port;
pub mod simple_pwm;
pub mod spi;
pub mod usart;
pub mod wdt;

/// Prelude containing all HAL traits
pub mod prelude {
    pub use crate::hal_v0::prelude::*;
    pub use ufmt::uWrite as _ufmt_uWrite;
    pub use unwrap_infallible::UnwrapInfallible as _unwrap_infallible_UnwrapInfallible;
}

// For making certain traits unimplementable from outside this crate.
mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;
