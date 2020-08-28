#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::adc;
use arduino_uno::prelude::*;

// This example opens a serial connection to the host computer.  On most POSIX operating systems (like GNU/Linux or
// OSX), you can interface with the program by running (assuming the device appears as ttyACM0)
//
// $ sudo screen /dev/ttyACM0 9600

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial =
        arduino_uno::Serial::new(dp.USART0, pins.d0, pins.d1.into_output(&mut pins.ddr), 9600);

    let mut adc = adc::Adc::new(dp.ADC, Default::default());
    let mut a0 = pins.a0.into_analog_input(&mut adc);

    ufmt::uwriteln!(&mut serial, "Reading analog input on pin A0\r").void_unwrap();

    loop {
        let value: u16 = nb::block!(adc.read(&mut a0)).void_unwrap();
        ufmt::uwriteln!(&mut serial, "read: {}\r", value).void_unwrap();
    }
}
