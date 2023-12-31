#![no_std]

//! If you want library routines to be portable between different AVR implementations,
//! it is best to use types from [avr_hal_generic] instead of [arduino_hal]

use avr_hal_generic::adc::AdcChannel;
use embedded_hal::spi;
use ufmt::uWrite;
pub use void::ResultVoidErrExt as _;
pub use void::ResultVoidExt as _;

pub fn report<IO>(serial: &mut IO)
where
    IO: embedded_hal::serial::Read<u8, Error = void::Void>
        + embedded_hal::serial::Write<u8, Error = void::Void>
        + uWrite<Error = void::Void>,
{
    let _ = serial.read();
    // Read a byte from the serial connection
    let b = nb::block!(serial.read()).void_unwrap();

    nb::block!(serial.write(b'z')).void_unwrap();

    // Answer
    ufmt::uwriteln!(serial, "Got {}!\r", b).void_unwrap();
}

pub fn report_adc_single<
    W: uWrite<Error = void::Void>,
    H,
    ADCOPS: avr_hal_generic::adc::AdcOps<H>,
    CLOCK: avr_hal_generic::clock::Clock,
    PIN: AdcChannel<H, ADCOPS>,
>(
    serial: &mut W,
    adc: &mut avr_hal_generic::adc::Adc<H, ADCOPS, CLOCK>,
    i: usize,
    analog_pin: &PIN,
) {
    let v = adc.read_blocking(analog_pin);
    ufmt::uwrite!(serial, "A{}: {} ", i, v).void_unwrap();
}

pub fn report_adc_multi<
    W: uWrite<Error = void::Void>,
    H,
    ADCOPS: avr_hal_generic::adc::AdcOps<H>,
    CLOCK: avr_hal_generic::clock::Clock,
>(
    serial: &mut W,
    adc: &mut avr_hal_generic::adc::Adc<H, ADCOPS, CLOCK>,
    channels: &[avr_hal_generic::adc::Channel<H, ADCOPS>],
) {
    for (i, ch) in channels.iter().enumerate() {
        let v = adc.read_blocking(ch);
        ufmt::uwrite!(serial, "A{}: {} ", i, v).void_unwrap();
    }

    ufmt::uwriteln!(serial, "").void_unwrap();
}

pub fn spi_loopback<SPI>(spi: &mut SPI, val: u8) -> u8
where
    SPI: spi::FullDuplex<u8, Error = void::Void>,
{
    // Send a byte
    nb::block!(spi.send(val)).void_unwrap();
    // Because MISO is connected to MOSI, the read data should be the same
    nb::block!(spi.read()).void_unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
