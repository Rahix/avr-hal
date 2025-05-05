//! This example demonstrates how to fade an RGB LED connected to an Arduino board.
//!
//! Wiring:
//! - Connect the common cathode of the RGB LED to GND.
//! - Connect the red LED anode to pin D6 through a current-limiting resistor.
//! - Connect the green LED anode to pin D5 through a current-limiting resistor.
//! - Connect the blue LED anode to pin D3 through a current-limiting resistor.
//!
//! Note: The current-limiting resistor values depend on the specific RGB LED and the desired brightness.
//! Typically, a resistor value between 220Ω and 1kΩ is suitable.

#![no_std]
#![no_main]

use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::simple_pwm::{Timer3Pwm, Timer4Pwm};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer0 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);
    let timer1 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);

    let mut d6 = pins.d6.into_output().into_pwm(&timer0);
    let mut d5 = pins.d5.into_output().into_pwm(&timer1);
    let mut d3 = pins.d3.into_output().into_pwm(&timer1);

    let max_duty_d6 = d6.get_max_duty();
    let max_duty_d5 = d5.get_max_duty();
    let max_duty_d3 = d3.get_max_duty();

    let delay_time = 10;

    d6.enable();
    d5.enable();
    d3.enable();

    loop {
        // Fade in/out red
        for i in (0..=max_duty_d6).chain((0..=max_duty_d6 - 1).rev()) {
            d6.set_duty(i);
            arduino_hal::delay_ms(delay_time);
        }

        // Fade in/out green
        for i in (0..=max_duty_d5).chain((0..=max_duty_d5 - 1).rev()) {
            d5.set_duty(i);
            arduino_hal::delay_ms(delay_time);
        }

        // Fade in/out blue
        for i in (0..=max_duty_d3).chain((0..=max_duty_d3 - 1).rev()) {
            d3.set_duty(i);
            arduino_hal::delay_ms(delay_time);
        }
    }
}
