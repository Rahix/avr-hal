#![no_std]
#![no_main]

extern crate panic_halt;
use sparkfun_pro_micro::prelude::*;

#[sparkfun_pro_micro::entry]
fn main() -> ! {
    let dp = sparkfun_pro_micro::Peripherals::take().unwrap();

    let mut pins = sparkfun_pro_micro::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut serial = sparkfun_pro_micro::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    ufmt::uwriteln!(&mut serial, "Reading analog inputs ...\r").void_unwrap();

    let mut adc = sparkfun_pro_micro::adc::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, temp): (u16, u16, u16) = (
        nb::block!(adc.read(&mut sparkfun_pro_micro::adc::channel::Vbg)).void_unwrap(),
        nb::block!(adc.read(&mut sparkfun_pro_micro::adc::channel::Gnd)).void_unwrap(),
        nb::block!(adc.read(&mut sparkfun_pro_micro::adc::channel::Temperature)).void_unwrap(),
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

        sparkfun_pro_micro::delay_ms(1000);
    }
}
