use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/// Delay type for `embedded-hal` compatibility.
///
/// This type can be used to pass a generic delay utility to `embedded-hal` drivers.  For direct
/// use in `arduino-hal` code, usage of [`delay_ms`] or [`delay_us`] is preferred.
pub type Delay = avr_hal_generic::delay::Delay<crate::DefaultClock>;

/// Delay execution for a number of milliseconds.
///
/// Busy-loop for the given time.  This function assumes the default clock speed defined by
/// [`arduino_hal::DefaultClock`][crate::DefaultClock].
pub fn delay_ms(ms: u16) {
    Delay::new().delay_ms(ms)
}

/// Delay execution for a number of microseconds.
///
/// Busy-loop for the given time.  This function assumes the default clock speed defined by
/// [`arduino_hal::DefaultClock`][crate::DefaultClock].
pub fn delay_us(us: u32) {
    Delay::new().delay_us(us)
}
