#[cfg(any(
    feature = "_board-arduino-mega1280",
    feature = "_board-arduino-mega2560"
))]
mod mega;

#[cfg(any(
    feature = "_board-arduino-mega1280",
    feature = "_board-arduino-mega2560"
))]
pub(crate) use mega::*;

#[cfg(any(
    feature = "_board-arduino-nano",
    feature = "_board-arduino-uno",
    feature = "_board-nano168",
    feature = "_board-sparkfun-promini-3v3",
    feature = "_board-sparkfun-promini-5v"
))]
mod uno;

#[cfg(any(
    feature = "_board-arduino-nano",
    feature = "_board-arduino-uno",
    feature = "_board-nano168",
    feature = "_board-sparkfun-promini-3v3",
    feature = "_board-sparkfun-promini-5v"
))]
pub(crate) use uno::*;
