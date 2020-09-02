#![no_std]
#![no_main]

use arduino_uno::adc;
use arduino_uno::prelude::*;
use panic_halt as _;

// This example opens a serial connection to the host computer.  On most POSIX operating systems
// (like GNU/Linux or OSX), you can interface with the program by running (assuming the device
// appears as ttyACM0)
//
// $ sudo screen /dev/ttyACM0 9600

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial =
        arduino_uno::Serial::new(dp.USART0, pins.d0, pins.d1.into_output(&mut pins.ddr), 9600);

    let mut adc = adc::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd): (u16, u16) = (
        nb::block!(adc.read(&mut adc::channel::Vbg)).void_unwrap(),
        nb::block!(adc.read(&mut adc::channel::Gnd)).void_unwrap(),
    );

    ufmt::uwriteln!(&mut serial, "Vbandgap: {}\r", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "GND: {}\r", gnd).void_unwrap();

    let mut a0 = pins.a0.into_analog_input(&mut adc);
    let mut a1 = pins.a1.into_analog_input(&mut adc);
    let mut a2 = pins.a2.into_analog_input(&mut adc);
    let mut a3 = pins.a3.into_analog_input(&mut adc);
    let mut a4 = pins.a4.into_analog_input(&mut adc);
    let mut a5 = pins.a5.into_analog_input(&mut adc);

    loop {
        let values: [u16; 6] = [
            nb::block!(adc.read(&mut a0)).void_unwrap(),
            nb::block!(adc.read(&mut a1)).void_unwrap(),
            nb::block!(adc.read(&mut a2)).void_unwrap(),
            nb::block!(adc.read(&mut a3)).void_unwrap(),
            nb::block!(adc.read(&mut a4)).void_unwrap(),
            nb::block!(adc.read(&mut a5)).void_unwrap(),
        ];

        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).void_unwrap();
        }
        ufmt::uwriteln!(&mut serial, "\r").void_unwrap();

        arduino_uno::delay_ms(1000);
    }
}
