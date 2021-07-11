#[allow(unused_imports)]
pub use avr_hal_generic::wdt::{Timeout, WdtOps};

pub type Wdt = avr_hal_generic::wdt::Wdt<crate::Atmega, crate::pac::WDT>;

avr_hal_generic::impl_wdt! {
    hal: crate::Atmega,
    peripheral: crate::pac::WDT,
    mcusr: crate::pac::cpu::MCUSR,
    timeout: |to, w| match to {
        Timeout::Ms16 => w.wdpl().cycles_2k_512k(),
        Timeout::Ms32 => w.wdpl().cycles_4k_1024k(),
        Timeout::Ms64 => w.wdpl().cycles_8k(),
        Timeout::Ms125 => w.wdpl().cycles_16k(),
        Timeout::Ms250 => w.wdpl().cycles_32k(),
        Timeout::Ms500 => w.wdpl().cycles_64k(),
        Timeout::Ms1000 => w.wdpl().cycles_128k(),
        Timeout::Ms2000 => w.wdpl().cycles_256k(),
        Timeout::Ms4000 => w.wdph().set_bit().wdpl().cycles_2k_512k(),
        Timeout::Ms8000 => w.wdph().set_bit().wdpl().cycles_4k_1024k(),
    },
}
