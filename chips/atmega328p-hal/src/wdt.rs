//! WatchDog Timer Implementation

pub use avr_hal_generic::wdt::watchdog::*;
pub use avr_hal_generic::wdt::*;

avr_hal::impl_wdt! {

    pub enum WatchdogTimeOutPeriod {
        /// 16 milliseconds
        Ms16,
        /// 32 milliseconds
        Ms32,
        /// 64 milliseconds
        Ms64,
        /// 125 milliseconds
        Ms125,
        /// 250 milliseconds
        Ms250,
        /// 500 milliseconds
        Ms500,
        /// 1 second
        Ms1000,
        /// 2 seconds
        Ms2000,
        /// 4 seconds
        Ms4000,
        /// 8 seconds
        Ms8000,
    }

    pub struct Wdt {
        mcu_status_register: crate::atmega328p::cpu::MCUSR,
        peripheral: crate::atmega328p::WDT,
        prescaler_bits: {
            WatchdogTimeOutPeriod::Ms16 => wdpl().cycles_2k_512k(),
            WatchdogTimeOutPeriod::Ms32 => wdpl().cycles_4k_1024k(),
            WatchdogTimeOutPeriod::Ms64 => wdpl().cycles_8k(),
            WatchdogTimeOutPeriod::Ms125 => wdpl().cycles_16k(),
            WatchdogTimeOutPeriod::Ms250 => wdpl().cycles_32k(),
            WatchdogTimeOutPeriod::Ms500 => wdpl().cycles_64k(),
            WatchdogTimeOutPeriod::Ms1000 => wdpl().cycles_128k(),
            WatchdogTimeOutPeriod::Ms2000 => wdpl().cycles_256k(),
            WatchdogTimeOutPeriod::Ms4000 => wdpl().cycles_2k_512k().wdph().set_bit(),
            WatchdogTimeOutPeriod::Ms8000 => wdpl().cycles_4k_1024k().wdph().set_bit()
        }
    }
}
