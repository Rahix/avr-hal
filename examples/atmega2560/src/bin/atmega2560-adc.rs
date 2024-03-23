#![no_std]
#![no_main]

use atmega_hal::adc::channel;
use atmega_hal::clock;
use atmega_hal::delay::Delay;
use atmega_hal::usart::{Baudrate, Usart};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

type Adc = atmega_hal::adc::Adc<clock::MHz16>;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::<clock::MHz16>::new();

    // set up serial interface for text output
    let mut serial = Usart::new(
        dp.USART0,
        pins.pe0,
        pins.pe1.into_output(),
        Baudrate::<clock::MHz16>::new(57600),
    );

    let mut adc = Adc::new(dp.ADC, Default::default());

    let (vbg, gnd) = (
        adc.read_blocking(&channel::Vbg),
        adc.read_blocking(&channel::Gnd),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).unwrap();

    // To store multiple channels in an array, we use the `into_channel()` method.
    let channels: [atmega_hal::adc::Channel; 16] = [
        pins.pf0.into_analog_input(&mut adc).into_channel(),
        pins.pf1.into_analog_input(&mut adc).into_channel(),
        pins.pf2.into_analog_input(&mut adc).into_channel(),
        pins.pf3.into_analog_input(&mut adc).into_channel(),
        pins.pf4.into_analog_input(&mut adc).into_channel(),
        pins.pf5.into_analog_input(&mut adc).into_channel(),
        pins.pf6.into_analog_input(&mut adc).into_channel(),
        pins.pf7.into_analog_input(&mut adc).into_channel(),
        pins.pk0.into_analog_input(&mut adc).into_channel(),
        pins.pk1.into_analog_input(&mut adc).into_channel(),
        pins.pk2.into_analog_input(&mut adc).into_channel(),
        pins.pk3.into_analog_input(&mut adc).into_channel(),
        pins.pk4.into_analog_input(&mut adc).into_channel(),
        pins.pk5.into_analog_input(&mut adc).into_channel(),
        pins.pk6.into_analog_input(&mut adc).into_channel(),
        pins.pk7.into_analog_input(&mut adc).into_channel(),
    ];

    loop {
        for (i, ch) in channels.iter().enumerate() {
            let v = adc.read_blocking(ch);
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).unwrap();
        }

        ufmt::uwriteln!(&mut serial, "").unwrap();
        delay.delay_ms(1000);
    }
}
