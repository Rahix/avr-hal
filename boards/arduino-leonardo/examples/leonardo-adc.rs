#![no_std]
#![no_main]

use panic_halt as _;
use arduino_leonardo::prelude::*;
use arduino_leonardo::adc;

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    let mut serial = arduino_leonardo::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Reading analog inputs ...\r").void_unwrap();

    let mut adc = adc::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, temp): (u16, u16, u16) = (
        nb::block!(adc.read(&mut adc::channel::Vbg)).void_unwrap(),
        nb::block!(adc.read(&mut adc::channel::Gnd)).void_unwrap(),
        nb::block!(adc.read(&mut adc::channel::Temperature)).void_unwrap(),
    );

    ufmt::uwriteln!(&mut serial, "Vbandgap: {}\r", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "GND: {}\r", gnd).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Temperature Sensor: {}\r", temp).void_unwrap();

    let portf = dp.PORTF.split();
    let mut a0 = portf.pf7.into_analog_input(&mut adc);
    let mut a1 = portf.pf6.into_analog_input(&mut adc);
    let mut a2 = portf.pf5.into_analog_input(&mut adc);
    let mut a3 = portf.pf4.into_analog_input(&mut adc);
    let mut a4 = portf.pf1.into_analog_input(&mut adc);
    let mut a5 = portf.pf0.into_analog_input(&mut adc);

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

        arduino_leonardo::delay_ms(1000);
    }
}
