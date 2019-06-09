#![no_std]
#![feature(asm)]

pub extern crate embedded_hal as hal;

#[doc(hidden)]
pub extern crate nb;
#[doc(hidden)]
pub extern crate void;
#[doc(hidden)]
pub extern crate ufmt;

pub mod clock;
pub mod delay;
pub mod port;
pub mod serial;

pub mod prelude {
    pub use hal::prelude::*;
    pub use hal::digital::v2::OutputPin as _embedded_hal_digital_v2_OutputPin;
    pub use hal::digital::v2::InputPin as _embedded_hal_digital_v2_InputPin;
    pub use hal::digital::v2::StatefulOutputPin as _embedded_hal_digital_v2_StatefulOutputPin;
    pub use hal::digital::v2::ToggleableOutputPin as _embedded_hal_digital_v2_ToggleableOutputPin;
    pub use void::ResultVoidExt as _void_ResultVoidExt;
    pub use void::ResultVoidErrExt as _void_ResultVoidErrExt;
    pub use ufmt::uWrite as _ufmt_uWrite;
}
