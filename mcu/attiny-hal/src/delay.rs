use avr_hal_generic::hal::delay::DelayNs;
use avr_hal_generic::hal_v0::blocking::delay::{DelayMs, DelayUs};

/// Default clock speed for ATtiny devices (1 MHz internal RC oscillator)
/// Note: ATtiny13A defaults to 9.6MHz internal oscillator divided by 8 = 1.2MHz,
/// but this assumes 1MHz for compatibility. For accurate timing at other speeds,
/// use the generic delay types with appropriate clock configuration.
pub type DefaultClock = avr_hal_generic::clock::MHz1;

/// Delay type for `embedded-hal` compatibility.
///
/// This type can be used to pass a generic delay utility to `embedded-hal` drivers.  For direct
/// use in `attiny-hal` code, usage of [`delay_ms`] or [`delay_us`] is preferred.
pub type Delay = avr_hal_generic::delay::Delay<DefaultClock>;

/// Delay execution for a number of milliseconds.
///
/// Busy-loop for the given time.  This function assumes the default clock speed of 1 MHz
/// for ATtiny devices. Note: ATtiny13A ships with 9.6MHz/8 = 1.2MHz by default.
/// For precise timing, configure the clock speed appropriately or use calibrated delays.
pub fn delay_ms(ms: u32) {
    let mut delay = Delay::new();
    DelayMs::delay_ms(&mut delay, ms as u16);
}

/// Delay execution for a number of microseconds.
///
/// Busy-loop for the given time.  This function assumes the default clock speed of 1 MHz
/// for ATtiny devices. Note: ATtiny13A ships with 9.6MHz/8 = 1.2MHz by default.
/// For precise timing, configure the clock speed appropriately or use calibrated delays.
pub fn delay_us(us: u32) {
    let mut delay = Delay::new();
    DelayUs::delay_us(&mut delay, us as u16);
}

/// Delay execution for a number of nanoseconds.
///
/// Busy-loop for the given time.  This function assumes the default clock speed of 1 MHz
/// for ATtiny devices. Note: ATtiny13A ships with 9.6MHz/8 = 1.2MHz by default.
/// For precise timing, configure the clock speed appropriately or use calibrated delays.
pub fn delay_ns(ns: u32) {
    let mut delay = Delay::new();
    delay.delay_ns(ns);
}
