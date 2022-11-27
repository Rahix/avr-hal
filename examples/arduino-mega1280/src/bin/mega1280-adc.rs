#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use arduino_hal::adc;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).void_unwrap();

    // To store multiple channels in an array, we use the `into_channel()` method.
    let channels: [adc::Channel; 16] = [
        pins.a0.into_analog_input(&mut adc).into_channel(),
        pins.a1.into_analog_input(&mut adc).into_channel(),
        pins.a2.into_analog_input(&mut adc).into_channel(),
        pins.a3.into_analog_input(&mut adc).into_channel(),
        pins.a4.into_analog_input(&mut adc).into_channel(),
        pins.a5.into_analog_input(&mut adc).into_channel(),
        pins.a6.into_analog_input(&mut adc).into_channel(),
        pins.a7.into_analog_input(&mut adc).into_channel(),
        pins.a8.into_analog_input(&mut adc).into_channel(),
        pins.a9.into_analog_input(&mut adc).into_channel(),
        pins.a10.into_analog_input(&mut adc).into_channel(),
        pins.a11.into_analog_input(&mut adc).into_channel(),
        pins.a12.into_analog_input(&mut adc).into_channel(),
        pins.a13.into_analog_input(&mut adc).into_channel(),
        pins.a14.into_analog_input(&mut adc).into_channel(),
        pins.a15.into_analog_input(&mut adc).into_channel(),
    ];

    loop {
        for (i, ch) in channels.iter().enumerate() {
            let v = adc.read_blocking(ch);
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).void_unwrap();
        }

        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}
