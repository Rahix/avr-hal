#![no_std]
#![no_main]

use arduino_hal::arduino::nano_v2 as board;
use board::hal::wdt;
use board::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let pins = board::pins!(dp);
    let mut serial = board::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();
    led.set_high();

    ufmt::uwriteln!(&mut serial, "Setup started...").unwrap_infallible();

    for _ in 0..20 {
        ufmt::uwrite!(&mut serial, ".").unwrap_infallible();
        led.toggle();
        board::delay_ms(100);
    }
    ufmt::uwriteln!(&mut serial, "\nEnabling watchdog...").unwrap_infallible();

    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms4000).unwrap();

    ufmt::uwriteln!(&mut serial, "\nWatchdog on watch...").unwrap_infallible();

    loop {
        ufmt::uwriteln!(&mut serial, "\nWaiting...").unwrap_infallible();

        led.toggle();
        board::delay_ms(1000);
        //watchdog.feed();
    }
}
