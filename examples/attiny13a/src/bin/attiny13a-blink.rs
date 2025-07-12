#![no_std]
#![no_main]

use attiny_hal::pac::Peripherals;
use panic_halt as _;

#[attiny_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    // Configure PB1 as output for LED
    let mut led = pins.pb1.into_output();

    loop {
        led.set_high();
        attiny_hal::delay_ms(500);
        led.set_low();
        attiny_hal::delay_ms(500);
    }
}
