/*!
  # A light sensor application

  This is a simple circuit with a light dependent resistor (light sensor).
  According to the brightness the lightsensor is seeing a sentences like: "This is a well lit room!" are emitted using the serial USB connection.

  # The hardware

    * a ldr sensor
    * a 10k Ohm resistor
    * some patch cables/connections
    * The ports on the board (GND, A5, 5V)

  # The board layout

  A5----------+
              |
  GND---10K---+--+
                  LDR
  5V-------------+

  A connection from ground to the 10K resistor, to the light defined resistor, to 5V. Additionally the Analog port 5 of the arduino needs to be connected between the 10K and light defined resistor.
!*/

#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    // setup the pins
    let pins = arduino_hal::pins!(dp);
    // setup the serial connection for the output.
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    // setup the analog digital converter
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    // in this example we only need pin a5
    let a5 = pins.a5.into_analog_input(&mut adc);

    loop {
        // read the voltage at a5 and convert it to an integer between 0 and 1023
        let sensor_value = a5.analog_read(&mut adc);

        // convert the number to human readable words
        let worded = match sensor_value {
            x if x > 950 => "bright",
            x if x > 750 => "well lit",
            x if x > 450 => "lit",
            x if x > 250 => "dimmly lit",
            x if x > 50 => "rather dark",
            x if x <= 50 => "very dark",
            _ => "invalid", // to satisfy the compiler ()
        };

        // output to the serial
        ufmt::uwrite!(&mut serial, "This is a {} room! – ", worded).void_unwrap();
        ufmt::uwrite!(&mut serial, "Raw value: {} ", sensor_value).void_unwrap();
        ufmt::uwriteln!(&mut serial, "").void_unwrap();

        // wait for half a second then measure again
        arduino_hal::delay_ms(500);
    }
}
