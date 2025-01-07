#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led1 = pins.led_rx.into_output();
    let mut led2 = pins.led_tx.into_output();

    loop {
        led1.set_high();
        led2.set_low();
        arduino_hal::delay_ms(300);
        led1.set_low();
        led2.set_high();
        arduino_hal::delay_ms(300);
    }
}
