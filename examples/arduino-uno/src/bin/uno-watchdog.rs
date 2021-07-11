#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use arduino_hal::hal::wdt;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    led.set_high();

    for i in 0..20 {
        led.toggle();
        arduino_hal::delay_ms(100);
    }

    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms2000).unwrap();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
        watchdog.feed();
    }
}
