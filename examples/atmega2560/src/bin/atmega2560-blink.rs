#![no_std]
#![no_main]

use panic_halt as _;
use atmega_hal::delay::Delay;
use embedded_hal::delay::DelayNs;

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz16;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    let mut delay = Delay::<crate::CoreClock>::new();

    let mut led = pins.pb7.into_output();

    loop {
        led.toggle();
        delay.delay_ms(1000);
    }
}
