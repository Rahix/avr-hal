/*!
 * Show readouts of all ADC channels.
 *
 * This example displays values for all ADC channels over the serial console.  During startup, it
 * also displays the values for Vbandgap, GND, and a readout of the MCU's temperature sensor.  For
 * the meanings of these values, please reference the ATmega328P datasheet.
 *
 * Connections
 * -----------
 *  - `A0` - `A5`: Connect analog voltages as you like to see them read out.
 */
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
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).unwrap_infallible();

    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    let a2 = pins.a2.into_analog_input(&mut adc);
    let a3 = pins.a3.into_analog_input(&mut adc);
    let a4 = pins.a4.into_analog_input(&mut adc);
    let a5 = pins.a5.into_analog_input(&mut adc);

    loop {
        let values = [
            a0.analog_read(&mut adc),
            a1.analog_read(&mut adc),
            a2.analog_read(&mut adc),
            a3.analog_read(&mut adc),
            a4.analog_read(&mut adc),
            a5.analog_read(&mut adc),
        ];

        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).unwrap_infallible();
        }

        ufmt::uwriteln!(&mut serial, "").unwrap_infallible();
        arduino_hal::delay_ms(1000);
    }
}
