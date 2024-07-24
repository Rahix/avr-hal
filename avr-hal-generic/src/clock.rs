//! Core clock speed management
//!
//! AVR microcontrollers support different core clock speeds.  Peripheral drivers need to know
//! about this speed to calculate timing parameters.  To make this as efficient as possible, the
//! clock speed is tracked as a compile-time constant.  This means peripheral drivers can do
//! compile-time calculation of timing parameters.
//!
//! # How To Use
//! If you are using `arduino-hal`, there is nothing you need to do - the core clock speed is
//! defined in `arduino-hal` as `arduino_hal::DefaultClock` and the const-generic parameters of all
//! peripheral drivers are preset to this value.
//!
//! If you are using a MCU HAL like `atmega-hal` or `attiny-hal`, you need to take care of clock
//! speed management manually.  The best way to do this is as follows:
//!
//! - Define a "constant" for your core clock speed in the crate root:
//!   ```ignore
//!   type CoreClock = atmega_hal::clock::MHz16;
//!   ```
//! - Define aliases for peripheral driver types based on this clock:
//!   ```ignore
//!   type Adc = atmega_hal::adc::Adc<crate::CoreClock>;
//!   type I2c = atmega_hal::i2c::I2c<crate::CoreClock>;
//!   ```

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
