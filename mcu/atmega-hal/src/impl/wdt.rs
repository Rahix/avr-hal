#![allow(unused_macros)]

macro_rules! impl_mod_wdt {
    ($($mod:item)*) => {
        pub mod wdt {
            #[allow(unused_imports)]
            use crate::r#impl::{impl_wdt_peripheral_ms2000, impl_wdt_peripheral_ms8000, impl_wdt_peripheral};

            pub use avr_hal_generic::wdt::{Timeout, WdtOps};

            $($mod)*
        }

        pub use wdt::Wdt;
    }
}
pub(crate) use impl_mod_wdt;

macro_rules! impl_wdt_peripheral {
    (
        mcusr: $($mcusr:ident)::+,
        wdtcsr_name: $wdtcsr_name:ident,
        timeout: |$to:ident, $w:ident| $to_match:expr $(,)?
) => {

        avr_hal_generic::impl_wdt! {
            hal: hal::Hal,
            peripheral: hal::pac::WDT,
            mcusr: $($mcusr)::+,
            wdtcsr_name: $wdtcsr_name,
            timeout: |$to, $w| $to_match,
        }

        pub type Wdt = avr_hal_generic::wdt::Wdt<hal::Hal, hal::pac::WDT>;
    };
}
pub(crate) use impl_wdt_peripheral;

macro_rules! impl_wdt_peripheral_ms8000 {
    (
        mcusr: $($mcusr:ident)::+,
        wdtcsr_name: $wdtcsr_name:ident,
) => {
        impl_wdt_peripheral! {
            mcusr: $($mcusr)::+,
            wdtcsr_name: $wdtcsr_name,
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

    };
}
pub(crate) use impl_wdt_peripheral_ms8000;

macro_rules! impl_wdt_peripheral_ms2000 {
    (
        mcusr: $($mcusr:ident)::+,
        wdtcsr_name: $wdtcsr_name:ident,
    ) => {
        impl_wdt_peripheral! {
            mcusr: $($mcusr)::+,
            wdtcsr_name: $wdtcsr_name,
            timeout: |to, w| match to {
                Timeout::Ms16 => w.wdpl().cycles_16k(),
                Timeout::Ms32 => w.wdpl().cycles_32k(),
                Timeout::Ms64 => w.wdpl().cycles_64k(),
                Timeout::Ms125 => w.wdpl().cycles_128k(),
                Timeout::Ms250 => w.wdpl().cycles_256k(),
                Timeout::Ms500 => w.wdpl().cycles_512k(),
                Timeout::Ms1000 => w.wdpl().cycles_1024k(),
                Timeout::Ms2000 => w.wdpl().cycles_2048k(),
                Timeout::Ms4000 => panic!(), // Does not exist on this MCU ...
                Timeout::Ms8000 => panic!() // Does not exist on this MCU ...
                },
        }
    };
}
pub(crate) use impl_wdt_peripheral_ms2000;
