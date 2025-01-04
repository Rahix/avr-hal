#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::arduino::diecimila as board;

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
