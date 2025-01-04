#![no_std]
#![no_main]

use arduino_hal::sparkfun::pro_micro as board;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let pins = board::pins!(dp);

    let mut led1 = pins.led_rx.into_output();
    let mut led2 = pins.led_tx.into_output();

    loop {
        led1.set_high();
        led2.set_low();
        board::delay_ms(300);
        led1.set_low();
        led2.set_high();
        board::delay_ms(300);
    }
}
