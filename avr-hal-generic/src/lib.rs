#![no_std]
#![cfg_attr(avr_hal_asm_macro, feature(asm_experimental_arch))]
#![cfg_attr(not(avr_hal_asm_macro), feature(llvm_asm))]
//! Defines the internal API used to generate the per-device external API
//! (user-facing) in the HAL crates.
//!
//! # Macro Design (WIP)
//!
//! As part of an effort to make the macros here more maintainable and less
//! repetitive, they are being restructured module-by-module. Currently revised
//! modules are:
//! - [`adc`]
//! - [`port`]
//!
//! The general guiding principals are:
//! 1. What goes inside the macro invocation should look like regular code as
//! much as possible;
//! 2. Information related to groups of implementations of a feature should be
//! encoded as alternative matchers in the macro, rather than by introducing
//! many metavariables that each invocation will need to repeat;
//!    As an example of such information, take the ADC's reference voltage.  All
//! Atmega processors can be abstracted with the same definition of the
//! `ReferenceVoltage` type, but Attiny processors differ among themselves and
//! also from the Atmega implementation. Rather than leave that type up to the
//! invocation, write one fully general matcher and write smaller matchers that
//! expand to pre-filled versions of the former. The HAL crates then use these
//! as much as possible, falling back only when there is singular hardware
//! that would need its own matcher but would use it only once.
//! 3. Information unique to each implementation should be left to the
//! invocation, but make the macro smart enough to avoid repeating ourselves.
//!    An example of this is the mapping between ADC channels and pins. The best
//! scenario here is a mapping like `<channel> = <pin>`, maybe `<channel>:
//! <type> = <pin>`, if there's more information needed to encode the mapping.
//! [`paste::paste`] can be used for gluing the information into adequate
//! identifiers.

pub use embedded_hal_v0 as hal;

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
    pub use crate::hal::prelude::*;
    pub use ufmt::uWrite as _ufmt_uWrite;
    pub use unwrap_infallible::UnwrapInfallible as _unwrap_infallible_UnwrapInfallible;
}

// For making certain traits unimplementable from outside this crate.
mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;
