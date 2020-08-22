#![no_std]
#![no_main]

extern crate panic_halt;
use sparkfun_pro_micro::prelude::*;

#[sparkfun_pro_micro::entry]
fn main() -> ! {
    let dp = sparkfun_pro_micro::Peripherals::take().unwrap();

    let mut pins = sparkfun_pro_micro::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
    );

    let mut led0 = pins.led_rx.into_output(&mut pins.ddr);
    let mut led1 = pins.led_tx.into_output(&mut pins.ddr);
    let mut led2 = pins.d13.into_output(&mut pins.ddr);

    led0.set_high().void_unwrap();
    led1.set_high().void_unwrap();
    led2.set_high().void_unwrap();

    let mut leds = [
        led0.downgrade(),
        led1.downgrade(),
        led2.downgrade(),
    ];

    loop {
        for i in 0..3 {
            leds[i].toggle().void_unwrap();
            leds[(i+2)%3].toggle().void_unwrap();
            sparkfun_pro_micro::delay_ms(200);
        }
    }
}
