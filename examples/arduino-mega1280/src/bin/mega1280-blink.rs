#![no_std]
#![no_main]

use arduino_hal::arduino::mega1280 as board;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let pins = board::pins!(dp);

    let mut led = pins.d13.into_output().downgrade();

    loop {
        led.toggle();
        board::delay_ms(1000);
    }
}
