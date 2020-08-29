#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use arduino_uno::wdt;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut watchdog = wdt::Wdt::new(&dp.CPU.mcusr, dp.WDT);
    watchdog.start(wdt::Timeout::Ms8000);

    loop {
        watchdog.feed();
    }
}
