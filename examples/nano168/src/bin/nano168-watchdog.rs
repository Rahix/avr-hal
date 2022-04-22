#![no_std]
#![no_main]

use arduino_hal::hal::wdt;
use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();
    led.set_high();

    ufmt::uwriteln!(&mut serial, "Setup started...").void_unwrap();

    for _i in 0..20 {
        ufmt::uwrite!(&mut serial, ".").void_unwrap();
        led.toggle();
        arduino_hal::delay_ms(100);
    }
    ufmt::uwriteln!(&mut serial, "\nEnabling watchdog...").void_unwrap();

    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms4000).unwrap();

    ufmt::uwriteln!(&mut serial, "\nWatchdog on watch...").void_unwrap();

    loop {
        ufmt::uwriteln!(&mut serial, "\nWaiting...").void_unwrap();

        led.toggle();
        arduino_hal::delay_ms(1000);
        //watchdog.feed();
    }
}
