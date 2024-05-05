/*!
 *
 * Code sample to work with 74HC595 shift register,
 * wired as in https://lastminuteengineers.com/74hc595-shift-register-arduino-tutorial/
 *
 * Connections
 * -----------
 *
 * - D4: SER (14)
 * - D5: RCLK (12)
 * - D6: SRCLK (11)
 */

#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::{
    hal::port::{PD4, PD5, PD6},
    port::{mode::Output, Pin},
};

fn shift_out(data_pin: &mut Pin<Output, PD4>, clock_pin: &mut Pin<Output, PD6>, data: &u8) {
    for i in 0..8 {
        let n = data & (1 << i);

        if n == 0 {
            data_pin.set_low();
        } else {
            data_pin.set_high();
        }

        clock_pin.set_high();
        clock_pin.set_low();
    }
}

fn update_shift_register(
    data_pin: &mut Pin<Output, PD4>,
    latch_pin: &mut Pin<Output, PD5>,
    clock_pin: &mut Pin<Output, PD6>,
    data: &u8,
) {
    latch_pin.set_low();

    shift_out(data_pin, clock_pin, data);

    latch_pin.set_high();
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut data_pin = pins.d4.into_output();
    let mut latch_pin = pins.d5.into_output();
    let mut clock_pin = pins.d6.into_output();

    loop {
        let mut data: u8 = 0;

        update_shift_register(&mut data_pin, &mut latch_pin, &mut clock_pin, &data);
        arduino_hal::delay_ms(500);

        for i in 0..8 {
            data |= 1 << i;

            update_shift_register(&mut data_pin, &mut latch_pin, &mut clock_pin, &data);
            arduino_hal::delay_ms(500);
        }
    }
}
