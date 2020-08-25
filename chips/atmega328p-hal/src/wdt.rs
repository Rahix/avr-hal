//! Provides a system reset when a counter reaches a given time-out value.
//!
//! # Note
//! Changing the watchdog configuration requires two separate writes to WDTCSR where the second
//! write must occur within 4 cycles of the first or the configuration will not change. You may need
//! to adjust optimization settings to prevent other operations from being emitted between these two
//! writes.
//!
//! # Example
//! ```
//! let mut watchdog = board::wdt::Wdt::new(&dp.CPU.mcusr, dp.WDT);
//! watchdog.disable();
//! watchdog.start(board::wdt::WatchdogTimeOutPeriod::Ms8000);
//!
//! loop {
//!     watchdog.feed();
//! }
//! ```

extern crate avr_hal_generic as avr_hal;

pub use avr_hal::wdt::*;

avr_hal::impl_wdt! {
    pub struct Wdt {
        mcu_status_register: crate::atmega328p::cpu::mcusr::MCUSR_SPEC,
        peripheral: crate::atmega328p::WDT,
    }
}
