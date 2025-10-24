/*!
  # A light sensor application

  This example uses the AVR butterfly onboard ldr sensor.
  According to the brightness the lightsensor is seeing a sentences like:
  "This is a well lit room!" are emitted using the serial USB connection.

!*/

#![no_std]
#![no_main]

use atmega_hal::delay::Delay;
use atmega_hal::usart::{Baudrate, Usart};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

// Define core clock in the root crate
type CoreClock = atmega_hal::clock::MHz8;
// Use it as follows in the rest of the project
type Adc = atmega_hal::adc::Adc<crate::CoreClock>;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();

    // double set in 4 cycles to disable JTAG and make shared ADC pins usable
    dp.JTAG.mcucr.write(|w| w.jtd().set_bit());
    dp.JTAG.mcucr.write(|w| w.jtd().set_bit());

    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::<crate::CoreClock>::new();

    // setup the serial connection for the output.
    let mut serial = Usart::new(
        dp.USART0,
        pins.pe0,
        pins.pe1.into_output(),
        Baudrate::<crate::CoreClock>::new(57600),
    );
    // setup the analog digital converter
    let mut adc = Adc::new(dp.ADC, Default::default());
    // in this example we only need pin pf2 which is connected to ldr sensor_2
    let pf2 = pins.pf2.into_analog_input(&mut adc);

    loop {
        // read the voltage at pf2 and convert it to an integer between 0 and 1023
        let sensor_value = pf2.analog_read(&mut adc);

        // convert the number to human readable words
        let worded = match sensor_value {
            x if x > 950 => "very dark",
            x if x > 800 => "rather dark",
            x if x > 650 => "dimmly lit",
            x if x > 400 => "lit",
            x if x > 100 => "well lit",
            x if x <= 100 => "bright",
            _ => "invalid", // to satisfy the compiler ()
        };

        // output to the serial
        ufmt::uwrite!(&mut serial, "This is a {} room! – ", worded).unwrap();
        ufmt::uwrite!(&mut serial, "Raw value: {} ", sensor_value).unwrap();
        ufmt::uwriteln!(&mut serial, "\r").unwrap();

        // wait for half a second then measure again
        delay.delay_ms(500);
    }
}
