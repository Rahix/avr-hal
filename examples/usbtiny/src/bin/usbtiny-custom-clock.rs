#![no_std]
#![no_main]

use arduino_hal::clock::MHz8;
use arduino_hal::hal::delay::Delay;
use embedded_hal::delay::DelayNs;
use panic_halt as _;


#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins: arduino_hal::Pins = arduino_hal::pins!(dp);


    let mut led = pins.d1.into_output();
    let mut delay: Delay<MHz8> = Delay::new();

    loop {
        led.toggle();
        delay.delay_ms(1);
    }
}
