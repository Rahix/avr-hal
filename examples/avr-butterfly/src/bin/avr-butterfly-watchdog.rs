#![no_std]
#![no_main]

use atmega_hal::delay::Delay;
use atmega_hal::usart::{Baudrate, Usart};
use atmega_hal::wdt;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

// Define core clock in the root crate
type CoreClock = atmega_hal::clock::MHz8;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::<crate::CoreClock>::new();

    // setup the serial connection for the output.
    let mut serial = Usart::new(
        dp.USART0,
        pins.pe0,
        pins.pe1.into_output(),
        Baudrate::<crate::CoreClock>::new(57600),
    );

    ufmt::uwrite!(&mut serial, "\r\nSetup started...").unwrap();

    for _ in 0..20 {
        ufmt::uwrite!(&mut serial, ".").unwrap();
        delay.delay_ms(100);
    }
    ufmt::uwriteln!(&mut serial, "\r\nEnabling watchdog...\r").unwrap();

    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
    watchdog.start(wdt::Timeout::Ms2000).unwrap();

    ufmt::uwriteln!(&mut serial, "Watchdog on watch...\r").unwrap();

    ufmt::uwrite!(&mut serial, "\rWaiting...").unwrap();
    loop {
        ufmt::uwrite!(&mut serial, ".").unwrap();
        delay.delay_ms(1000);
        // watchdog.feed();
    }
}
