//! Example for using an Arduno Uno with a 16-channel PCA9685 to control
//! servo motors.
//!
//! Makes the servo on channel 0 turn clockwise, and the servo on channel 1 turn
//! counterclockwise.
//!
//! Connections:
//! ```
//! GND <-> GND
//! A4 <-> SDA
//! A5 <-> SCL
//! 5V <-> VCC
//!
//! Two servo motors connected to channel 0 and channel 1.
//! ```
#![no_std]
#![no_main]

use panic_halt as _;
use pwm_pca9685::{Address, Channel, Pca9685};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        100_000,
    );

    // We use 0x40 as an address as that is the first default address.
    let mut pwm = Pca9685::new(i2c, Address::from(0x40)).unwrap();
    // This results in 60 Hz.
    pwm.set_prescale(100).unwrap();
    pwm.enable().unwrap();
    // Turn all channels on with a setting of "0".
    pwm.set_channel_on(Channel::All, 0).unwrap();

    // These need to be tweaked based on your own servo.
    let servo_min = 130; // Minimum pulse length out of 4096.
    let servo_max = 610; // Maximum pulse length out of 4096.

    let mut current = servo_min;
    let mut factor = 1i16;

    loop {
        // Blink the LED to indicate that everything is working properly.
        led.toggle();
        arduino_hal::delay_ms(500);
        led.toggle();
        arduino_hal::delay_ms(500);

        pwm.set_channel_off(Channel::C0, current).unwrap();
        pwm.set_channel_off(Channel::C1, servo_min + (servo_max - current))
            .unwrap();

        if current == servo_max {
            factor -= 1;
        } else if current == servo_min {
            factor = 1;
        }
        current = (current as i16 + factor) as u16;
    }
}
