macro_rules! impl_mod_wdt {
    (
        hal: $($hal:ident)::+, 
        wdtcsr_name: $wdtcsr_name:ident $(,)?
    ) => {
        pub mod wdt {
            pub use avr_hal_generic::wdt::{Timeout, WdtOps};

            avr_hal_generic::impl_wdt! {
                hal: $($hal)::+::Hal,
                peripheral: $($hal)::+::pac::WDT,
                mcusr: $($hal)::+::pac::cpu::MCUSR,
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

            pub type Wdt = avr_hal_generic::wdt::Wdt<$($hal)::+::Hal, $($hal)::+::pac::WDT>;
        }
        pub use wdt::Wdt;
    };
}

pub(crate) use impl_mod_wdt;
