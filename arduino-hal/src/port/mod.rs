#[cfg(feature = "arduino-leonardo")]
mod leonardo;
#[cfg(feature = "arduino-leonardo")]
pub use leonardo::*;
#[cfg(feature = "arduino-mega2560")]
mod mega2560;
#[cfg(feature = "arduino-mega2560")]
pub use mega2560::*;
#[cfg(any(feature = "arduino-nano", feature = "arduino-uno"))]
mod uno;
#[cfg(any(feature = "arduino-nano", feature = "arduino-uno"))]
pub use uno::*;

#[cfg(feature = "arduino-micro")]
mod micro;
#[cfg(feature = "arduino-micro")]
pub use micro::*;