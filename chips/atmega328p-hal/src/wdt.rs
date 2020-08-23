//! Provides a system reset when a counter reaches a given time-out value.
//!
//! # Example
//! ```
//! let mut watchdog = Wdt::new(dp.CPU, dp.WDT);
//! watchdog.disable();
//! watchdog.start(WatchdogTimeOutPeriod::Ms8000);
//!
//! loop {
//!     watchdog.feed();
//! }
//! ```

use avr_hal::hal::watchdog::*;

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
    cpu: crate::atmega328p::CPU,
    peripheral: crate::atmega328p::WDT,
}

impl Wdt {
    pub fn new(cpu: crate::atmega328p::CPU, peripheral: crate::atmega328p::WDT) -> Self {
        let watchdog = Wdt { cpu, peripheral };
        watchdog
    }
}

impl WatchdogEnable for Wdt {
    type Time = WatchdogTimeOutPeriod;

    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        // Disable interrupts while starting the watchdog timer
        avr_hal::avr_device::interrupt::free(|_| {
            // Reset the watchdog timer
            self.feed();
            // Reset the watchdog reset flag in the mcu status register
            self.cpu.mcusr.modify(|_, w| w.wdrf().clear_bit());
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
                    WatchdogTimeOutPeriod::Ms4000 => w.wdpl().cycles_256k().wdph().set_bit(),
                    WatchdogTimeOutPeriod::Ms8000 => w.wdpl().cycles_4k_1024k().wdph().set_bit(),
                })
                .wde()
                .set_bit()
            });
        });
    }
}

impl Watchdog for Wdt {
    #[inline]
    fn feed(&mut self) {
        unsafe { llvm_asm!("WDR") }
    }
}

impl WatchdogDisable for Wdt {
    fn disable(&mut self) {
        // Disable interrupts while disabling the watchdog timer
        avr_hal::avr_device::interrupt::free(|_| {
            // Reset the watchdog timer
            self.feed();
            // Reset the watchdog reset flag in the mcu status register
            self.cpu.mcusr.modify(|_, w| w.wdrf().clear_bit());
            // Enable watchdog configuration mode
            self.peripheral
                .wdtcsr
                .modify(|_, w| w.wdce().set_bit().wde().set_bit());
            // Disable watchdog
            self.peripheral.wdtcsr.reset();
        });
    }
}
