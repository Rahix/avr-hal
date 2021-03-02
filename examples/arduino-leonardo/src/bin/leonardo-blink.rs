#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut leds = [
        pins.led_rx.into_output().downgrade(),
        pins.led_tx.into_output().downgrade(),
        pins.d13.into_output().downgrade(),
    ];

    // RX & TX LEDs are active low and the LED on D13 is active high.  Thus invert LED13 here so
    // they are all in the same "state":
    leds[0].set_high();
    leds[1].set_high();
    leds[2].set_low();

    loop {
        for i in 0..3 {
            leds[i].toggle();
            arduino_hal::delay_ms(100);
            leds[i].toggle();
        }
    }
}
