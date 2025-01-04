pub(crate) mod r#impl;

#[cfg(feature = "_board-arduino-diecimila")]
pub mod diecimila;

#[cfg(feature = "_board-arduino-leonardo")]
pub mod leonardo;

#[cfg(feature = "_board-arduino-mega1280")]
pub mod mega1280;

#[cfg(feature = "_board-arduino-mega2560")]
pub mod mega2560;

#[cfg(feature = "_board-nano168")]
pub mod nano_v2;

#[cfg(feature = "_board-arduino-nano")]
pub mod nano_v3;

#[cfg(feature = "_board-arduino-uno")]
pub mod uno;
