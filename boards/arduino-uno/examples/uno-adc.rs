#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::prelude::*;

// This example opens a serial connection to the host computer.  On most POSIX operating systems (like GNU/Linux or
// OSX), you can interface with the program by running (assuming the device appears as ttyACM0)
//
// $ sudo screen /dev/ttyACM0 9600

#[no_mangle]
pub extern fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
    );

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600,
    );


    // create the Analog Digital Converter
    let mut adc = arduino_uno::adc::Adc::new(dp.ADC, arduino_uno::adc::AdcSettings::default());

    // Convert pin to Analog input
    let mut a0 = pins.a0.into_analog_input(&mut adc);


    ufmt::uwriteln!(&mut serial, "Reading Analog Input on PORT a0\r").unwrap();

    loop {
        // Read the Analog value
        let aread: u16 = nb::block!{adc.read(&mut a0)}.unwrap();

        // Write it to Serial
        ufmt::uwriteln!(&mut serial, "read: {}\r", aread);
    }
}
