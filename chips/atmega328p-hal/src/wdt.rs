//! WatchDog Timer Implementation

pub use avr_hal_generic::wdt::*;

avr_hal_generic::impl_wdt! {
    pub enum Timeout {
        /// 16 milliseconds
        Ms16 { wdpl().cycles_2k_512k() },
        /// 32 milliseconds
        Ms32 { wdpl().cycles_4k_1024k() },
        /// 64 milliseconds
        Ms64 { wdpl().cycles_8k() },
        /// 125 milliseconds
        Ms125 { wdpl().cycles_16k() },
        /// 250 milliseconds
        Ms250 { wdpl().cycles_32k() },
        /// 500 milliseconds
        Ms500 { wdpl().cycles_64k() },
        /// 1 second
        Ms1000 { wdpl().cycles_128k() },
        /// 2 seconds
        Ms2000 { wdpl().cycles_256k() },
        /// 4 seconds
        Ms4000 { wdph().set_bit().wdpl().cycles_2k_512k() },
        /// 8 seconds
        Ms8000 { wdph().set_bit().wdpl().cycles_4k_1024k() },
    }

    pub struct Wdt {
        mcu_status_register: crate::atmega328p::cpu::MCUSR,
        peripheral: crate::atmega328p::WDT,
    }
}
