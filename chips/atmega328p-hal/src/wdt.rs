//! WatchDog Timer Implementation

pub use avr_hal_generic::wdt::watchdog::*;
pub use avr_hal_generic::wdt::*;

avr_hal::impl_wdt! {
    pub struct Wdt {
        mcu_status_register: crate::atmega328p::cpu::MCUSR,
        peripheral: crate::atmega328p::WDT,
    }
}
