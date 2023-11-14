//! Generic trait for clock speeds

/// A clock speed
pub trait Clock {
    /// Frequency of this clock in Hz
    const FREQ: u32;
}

/// 24 MHz Clock
#[derive(ufmt::derive::uDebug, Debug)]
pub struct MHz24;
impl Clock for MHz24 {
    const FREQ: u32 = 24_000_000;
}

/// 20 MHz Clock
#[derive(ufmt::derive::uDebug, Debug)]
pub struct MHz20;
impl Clock for MHz20 {
    const FREQ: u32 = 20_000_000;
}

/// 16 MHz Clock
#[derive(ufmt::derive::uDebug, Debug)]
pub struct MHz16;
impl Clock for MHz16 {
    const FREQ: u32 = 16_000_000;
}

/// 12 MHz Clock
#[derive(ufmt::derive::uDebug, Debug)]
pub struct MHz12;
impl Clock for MHz12 {
    const FREQ: u32 = 12_000_000;
}

/// 10 MHz Clock
#[derive(ufmt::derive::uDebug, Debug)]
pub struct MHz10;
impl Clock for MHz10 {
    const FREQ: u32 = 10_000_000;
}

/// 8 MHz Clock
#[derive(ufmt::derive::uDebug, Debug)]
pub struct MHz8;
impl Clock for MHz8 {
    const FREQ: u32 = 8_000_000;
}

/// 1 MHz Clock
#[derive(ufmt::derive::uDebug, Debug)]
pub struct MHz1;
impl Clock for MHz1 {
    const FREQ: u32 = 1_000_000;
}
