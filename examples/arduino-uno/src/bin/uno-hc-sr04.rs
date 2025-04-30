/*!
 * Example for using the HC-SR04 ultrasonic distance sensor.
 *
 * This example prints out the distance reported by the sensor over the serial console.
 *
 * Sensor Datasheet: https://www.electroschematics.com/hc-sr04-datasheet/
 *
 * Connections
 * -----------
 *   - `D2`: HC-SR04 `TRIG`
 *   - `D3`: HC-SR04 `ECHO`
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut trig = pins.d2.into_output();
    let echo = pins.d3; // pin is input by default

    // Starting and initializing the timer with prescaling 64.
    // it gives one clock count every 4 µs.
    // since the clock register size is 16 bits, the timer is full every
    // 1/(16e6/64)*2^16 ≈ 260 ms
    let timer1 = dp.TC1;
    timer1.tccr1b().write(|w| w.cs1().prescale_64());

    'outer: loop {
        // the timer is reinitialized with value 0.
        timer1.tcnt1().write(|w| w.set(0));

        // the trigger must be set to high under 10 µs as per the HC-SR04 datasheet
        trig.set_high();
        arduino_hal::delay_us(10);
        trig.set_low();

        while echo.is_low() {
            // exiting the loop if the timer has reached 200 ms.
            // 0.2s/4µs = 50000
            if timer1.tcnt1().read().bits() >= 50000 {
                // jump to the beginning of the outer loop if no obstacle is detected
                ufmt::uwriteln!(
                    &mut serial,
                    "Nothing was detected and jump to outer loop.\r"
                )
                .unwrap_infallible();
                continue 'outer;
            }
        }
        // Restarting the timer
        timer1.tcnt1().write(|w| w.set(0));

        // Wait for the echo to get low again
        while echo.is_high() {}

        // 1 count == 4 µs, so the value is multiplied by 4.
        // 1/58 ≈ (34000 cm/s) * 1µs / 2
        // when no object is detected, instead of keeping the echo pin completely low,
        // some HC-SR04 labeled sensor holds the echo pin in high state for very long time,
        // thus overflowing the u16 value when multiplying the timer1 value with 4.
        // overflow during runtime causes panic! so it must be handled
        let temp_timer = timer1.tcnt1().read().bits().saturating_mul(4);
        let value = match temp_timer {
            u16::MAX => {
                ufmt::uwriteln!(
                    &mut serial,
                    "Nothing was detected and jump to outer loop.\r"
                )
                .unwrap_infallible();
                continue 'outer;
            }
            _ => temp_timer / 58,
        };

        // Await 100 ms before sending the next trig
        // 0.1s/4µs = 25000
        while timer1.tcnt1().read().bits() < 25000 {}

        ufmt::uwriteln!(
            &mut serial,
            "Hello, we are {} cms away from target!\r",
            value
        )
        .unwrap_infallible();
    }
}
