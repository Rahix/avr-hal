#![no_std]
#![no_main]

use attiny_hal::pac::Peripherals;
use attiny_hal::wdt::Timeout;
use attiny_hal::Wdt;
use panic_halt as _;

#[attiny_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    // Configure PB1 as output for LED
    let mut led = pins.pb1.into_output();

    // Initialize watchdog timer with 1 second timeout
    let mut wdt = Wdt::new(dp.WDT, &dp.CPU.mcusr);
    wdt.start(Timeout::Ms1000).unwrap();

    for _ in 1..5 {
        led.set_low();

        // Wait a bit
        attiny_hal::delay_ms(800);

        // Reset watchdog timer to prevent reset
        wdt.feed();

        // Blink LED
        led.set_high();
        attiny_hal::delay_ms(100);
    }

    loop {}
}
