#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut led  = dp.pins.d13.into_output().downgrade(); 

    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
        led.toggle();
    }
}