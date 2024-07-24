#[allow(unused_imports)]
pub use avr_hal_generic::wdt::{Timeout, WdtOps};

pub type Wdt = avr_hal_generic::wdt::Wdt<crate::Atmega, crate::pac::WDT>;

#[cfg(not(any(feature = "atmega8", feature = "atmega32a", feature = "atmega128a")))]
avr_hal_generic::impl_wdt! {
    hal: crate::Atmega,
    peripheral: crate::pac::WDT,
    mcusr: crate::pac::cpu::MCUSR,
    wdtcsr_name: wdtcsr,
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

#[cfg(any(feature = "atmega8", feature = "atmega32a", feature = "atmega128a"))]
avr_hal_generic::impl_wdt! {
    hal: crate::Atmega,
    peripheral: crate::pac::WDT,
    mcusr: crate::pac::cpu::MCUCSR,
    wdtcsr_name: wdtcr,
    timeout: |to, w| match to {
        Timeout::Ms16 => w.wdpl().cycles_16k(),
        Timeout::Ms32 => w.wdpl().cycles_32k(),
        Timeout::Ms64 => w.wdpl().cycles_64k(),
        Timeout::Ms125 => w.wdpl().cycles_128k(),
        Timeout::Ms250 => w.wdpl().cycles_256k(),
        Timeout::Ms500 => w.wdpl().cycles_512k(),
        Timeout::Ms1000 => w.wdpl().cycles_1024k(),
        Timeout::Ms2000 => w.wdpl().cycles_2048k(),
        Timeout::Ms4000 => panic!(), // Does not exist for ATmega8 ...
        Timeout::Ms8000 => panic!() // Does not exist for ATmega8 ...
    },
}
