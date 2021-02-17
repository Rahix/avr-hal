#[cfg(feature = "arduino-leonardo")]
mod leonardo;
#[cfg(feature = "arduino-leonardo")]
pub use leonardo::*;
#[cfg(feature = "arduino-uno")]
mod uno;
#[cfg(feature = "arduino-uno")]
pub use uno::*;
#[cfg(feature = "arduino-mega2560")]
mod mega;
#[cfg(feature = "arduino-mega2560")]
pub use mega::*;
