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

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).void_unwrap();

    // To store multiple channels in an array, we use the `into_channel()` method.
    let channels: [adc::Channel; 6] = [
        pins.a0.into_analog_input(&mut adc).into_channel(),
        pins.a1.into_analog_input(&mut adc).into_channel(),
        pins.a2.into_analog_input(&mut adc).into_channel(),
        pins.a3.into_analog_input(&mut adc).into_channel(),
        pins.a4.into_analog_input(&mut adc).into_channel(),
        pins.a5.into_analog_input(&mut adc).into_channel(),
    ];

    loop {
        avr_portable::report_adc(&mut serial, &mut adc, &channels);

        // Arduino Nano has two more ADC pins A6 and A7.  Accessing them works a bit different from
        // the other pins as they are not normal IO pins.  The code below shows how it works.
        let (a6, a7) = (
            adc.read_blocking(&adc::channel::ADC6),
            adc.read_blocking(&adc::channel::ADC7),
        );
        ufmt::uwrite!(&mut serial, "A6: {} A7: {}", a6, a7).void_unwrap();

        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}
