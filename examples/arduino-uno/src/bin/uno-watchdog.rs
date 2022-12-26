/*!
 * Demonstration of setting up the watchdog timer.
 *
 * A watchdog timer is used to ensure the firmware did not lock itself up.  This works by requiring
 * the firmware to periodically "feed" the watchdog.  If it fails to do so for a certain
 * (configurable) timeout, the watchdog will reset the device.
 */
#![no_std]
#![no_main]

use arduino_hal::hal::wdt;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    led.set_high();

    for _i in 0..20 {
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
