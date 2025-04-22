#[allow(unused_imports)]
pub use avr_hal_generic::wdt::{Timeout, WdtOps};

pub type Wdt = avr_hal_generic::wdt::Wdt<crate::Attiny, crate::pac::WDT>;
