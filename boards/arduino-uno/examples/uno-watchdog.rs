#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut watchdog = arduino_uno::wdt::Wdt::new(&dp.CPU.mcusr, dp.WDT);
    watchdog.start(arduino_uno::wdt::WatchdogTimeOutPeriod::Ms8000);

    loop {
        watchdog.feed();
    }
}
