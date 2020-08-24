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
//! let mut watchdog = Wdt::new(&dp.CPU.mcusr, dp.WDT);
//! watchdog.disable();
//! watchdog.start(WatchdogTimeOutPeriod::Ms8000);
//!
//! loop {
//!     watchdog.feed();
//! }
//! ```

use crate::atmega328p::{cpu, WDT};
use avr_device::generic::Reg;
use avr_hal::hal::watchdog::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WatchdogTimeOutPeriod {
    Ms16,
    Ms32,
    Ms64,
    Ms125,
    Ms250,
    Ms500,
    Ms1000,
    Ms2000,
    Ms4000,
    Ms8000,
}

pub struct Wdt {
    peripheral: WDT,
}

impl Wdt {
    /// Initializes a Wdt.
    ///
    /// If a prior reset was provided by the watchdog, the WDRF in MCUSR would be set, so WDRF is
    /// also cleared to allow for re-enabling the watchdog.
    pub fn new(mcu_status_register: &Reg<cpu::mcusr::MCUSR_SPEC>, peripheral: WDT) -> Self {
        mcu_status_register.modify(|_, w| w.wdrf().clear_bit());
        Wdt { peripheral }
    }
}

impl WatchdogEnable for Wdt {
    type Time = WatchdogTimeOutPeriod;

    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        // The sequence for changing time-out configuration is as follows:
        //
        //     1. In the same operation, write a logic one to the Watchdog change enable bit (WDCE)
        //        and WDE. A logic one must be written to WDE regardless of the previous value of
        //        the WDE bit.
        //     2. Within the next four clock cycles, write the WDE and Watchdog prescaler bits (WDP)
        //        as desired, but with the WDCE bit cleared. This must be done in one operation.
        avr_hal::avr_device::interrupt::free(|_| {
            // Reset the watchdog timer
            self.feed();
            // Enable watchdog configuration mode
            self.peripheral
                .wdtcsr
                .modify(|_, w| w.wdce().set_bit().wde().set_bit());
            // Enable watchdog and set interval
            self.peripheral.wdtcsr.write(|w| {
                (match period.into() {
                    WatchdogTimeOutPeriod::Ms16 => w.wdpl().cycles_2k_512k(),
                    WatchdogTimeOutPeriod::Ms32 => w.wdpl().cycles_4k_1024k(),
                    WatchdogTimeOutPeriod::Ms64 => w.wdpl().cycles_8k(),
                    WatchdogTimeOutPeriod::Ms125 => w.wdpl().cycles_16k(),
                    WatchdogTimeOutPeriod::Ms250 => w.wdpl().cycles_32k(),
                    WatchdogTimeOutPeriod::Ms500 => w.wdpl().cycles_64k(),
                    WatchdogTimeOutPeriod::Ms1000 => w.wdpl().cycles_128k(),
                    WatchdogTimeOutPeriod::Ms2000 => w.wdpl().cycles_256k(),
                    WatchdogTimeOutPeriod::Ms4000 => w.wdpl().cycles_2k_512k().wdph().set_bit(),
                    WatchdogTimeOutPeriod::Ms8000 => w.wdpl().cycles_4k_1024k().wdph().set_bit(),
                })
                .wde()
                .set_bit()
                .wdce()
                .clear_bit()
            });
        });
    }
}

impl Watchdog for Wdt {
    #[inline]
    fn feed(&mut self) {
        avr_device::asm::wdr();
    }
}

impl WatchdogDisable for Wdt {
    fn disable(&mut self) {
        // The sequence for clearing WDE is as follows:
        //
        //     1. In the same operation, write a logic one to the Watchdog change enable bit (WDCE)
        //        and WDE. A logic one must be written to WDE regardless of the previous value of
        //        the WDE bit.
        //     2. Within the next four clock cycles, clear the WDE and WDCE bits.
        //        This must be done in one operation.
        avr_hal::avr_device::interrupt::free(|_| {
            // Reset the watchdog timer
            self.feed();
            // Enable watchdog configuration mode
            self.peripheral
                .wdtcsr
                .modify(|_, w| w.wdce().set_bit().wde().set_bit());
            // Disable watchdog
            self.peripheral.wdtcsr.reset();
        });
    }
}
