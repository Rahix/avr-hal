#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 1 is also connected to the onboard LED
    let mut led = pins.p1.into_output();
    led.set_high();

    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}
