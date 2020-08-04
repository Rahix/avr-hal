#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_mega2560::prelude::*;

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();

    let mut pins = arduino_mega2560::Pins::new(
        dp.PORTA,
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
        dp.PORTF,
        dp.PORTG,
        dp.PORTH,
        dp.PORTJ,
        dp.PORTK,
        dp.PORTL,
    );
    let mut delay = arduino_mega2560::Delay::new();

    let mut serial = arduino_mega2560::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Reading analog inputs ...\r").void_unwrap();

    let mut adc = arduino_mega2560::adc::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd): (u16, u16) = (
        nb::block!(adc.read(&mut arduino_mega2560::adc::channel::Vbg)).void_unwrap(),
        nb::block!(adc.read(&mut arduino_mega2560::adc::channel::Gnd)).void_unwrap(),
    );

    ufmt::uwriteln!(&mut serial, "Vbandgap: {}\r", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "GND: {}\r", gnd).void_unwrap();

    let mut a0 = pins.a0.into_analog_input(&mut adc);
    let mut a1 = pins.a1.into_analog_input(&mut adc);
    let mut a2 = pins.a2.into_analog_input(&mut adc);
    let mut a3 = pins.a3.into_analog_input(&mut adc);
    let mut a4 = pins.a4.into_analog_input(&mut adc);
    let mut a5 = pins.a5.into_analog_input(&mut adc);
    let mut a6 = pins.a6.into_analog_input(&mut adc);
    let mut a7 = pins.a7.into_analog_input(&mut adc);
    let mut a8 = pins.a8.into_analog_input(&mut adc);
    let mut a9 = pins.a9.into_analog_input(&mut adc);
    let mut a10 = pins.a10.into_analog_input(&mut adc);
    let mut a11 = pins.a11.into_analog_input(&mut adc);
    let mut a12 = pins.a12.into_analog_input(&mut adc);
    let mut a13 = pins.a13.into_analog_input(&mut adc);
    let mut a14 = pins.a14.into_analog_input(&mut adc);
    let mut a15 = pins.a15.into_analog_input(&mut adc);

    loop {
        let values: [u16; 16] = [
            nb::block!(adc.read(&mut a0)).void_unwrap(),
            nb::block!(adc.read(&mut a1)).void_unwrap(),
            nb::block!(adc.read(&mut a2)).void_unwrap(),
            nb::block!(adc.read(&mut a3)).void_unwrap(),
            nb::block!(adc.read(&mut a4)).void_unwrap(),
            nb::block!(adc.read(&mut a5)).void_unwrap(),
            nb::block!(adc.read(&mut a6)).void_unwrap(),
            nb::block!(adc.read(&mut a7)).void_unwrap(),
            nb::block!(adc.read(&mut a8)).void_unwrap(),
            nb::block!(adc.read(&mut a9)).void_unwrap(),
            nb::block!(adc.read(&mut a10)).void_unwrap(),
            nb::block!(adc.read(&mut a11)).void_unwrap(),
            nb::block!(adc.read(&mut a12)).void_unwrap(),
            nb::block!(adc.read(&mut a13)).void_unwrap(),
            nb::block!(adc.read(&mut a14)).void_unwrap(),
            nb::block!(adc.read(&mut a15)).void_unwrap(),
        ];

        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).void_unwrap();
        }
        ufmt::uwriteln!(&mut serial, "\r").void_unwrap();

        delay.delay_ms(1000);
    }
}
