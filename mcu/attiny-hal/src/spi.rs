//! SPI
//!
//! # Example
//!
//! For full source code, please refer to the ATmega SPI example:
//! [`atmega2560-spi-feedback.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-spi-feedback.rs)
//!
//! ```
//! let dp = attiny_hal::Peripherals::take().unwrap();
//! let pins = attiny_hal::pins!(dp);
//!
//! let (mut spi, mut cs) = spi::Spi::new(
//!     dp.SPI,
//!     pins.pa4.into_output(),
//!     pins.pa6.into_output(),
//!     pins.pa5.into_pull_up_input(),
//!     pins.pa3.into_output(),
//!     spi::Settings::default(),
//! );
//!
//! let data_out = b"Hello World!";
//! let mut data_in = [0u8; 12];
//!
//! cs.set_low().unwrap();
//! spi.transfer(&mut data_in, data_out).unwrap();
//! cs.set_high().unwrap();
//!
//! ufmt::uwriteln!(&mut serial, "data: {:?}", data_in).unwrap();
//! ```

#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::spi::*;

#[cfg(feature = "attiny88")]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Attiny,
    crate::pac::SPI,
    port::PB5,
    port::PB3,
    port::PB4,
    port::PB2,
>;
#[cfg(feature = "attiny88")]
avr_hal_generic::impl_spi! {
    hal: crate::Attiny,
    peripheral: crate::pac::SPI,
    sclk: port::PB5,
    mosi: port::PB3,
    miso: port::PB4,
    cs: port::PB2,
}

#[cfg(feature = "attiny167")]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Attiny,
    crate::pac::SPI,
    port::PA5,
    port::PA4,
    port::PA2,
    port::PA6,
>;
#[cfg(feature = "attiny167")]
avr_hal_generic::impl_spi! {
    hal: crate::Attiny,
    peripheral: crate::pac::SPI,
    sclk: port::PA5,
    mosi: port::PA4,
    miso: port::PA2,
    cs: port::PA6,
}


#[cfg(feature = "attiny85")]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Attiny,
    crate::pac::USI,
    port::PB2,
    port::PB1,
    port::PB0,
    port::PB4,
    >;
#[cfg(feature = "attiny85")]
impl crate::spi::SpiOps<crate::Attiny, port::PB2, port::PB1, port::PB0, port::PB4> for crate::pac::USI {
    fn raw_setup(&mut self, _settings: &Settings) {
        self.usicr.write(|w| {
            w.usiwm().three_wire();
            w.usics().ext_pos();
            w.usiclk().set_bit()
        });
    }

    fn raw_release(&mut self) {
        self.usicr.write(|w| {
            w.usiwm().disabled()
        });
    }

    fn raw_check_iflag(&self) -> bool {
        self.usisr.read().usioif().bit_is_set()
    }

    fn raw_read(&self) -> u8 {
        // TODO how to determine if its read fully?
        // USIOIF tells if 8 cycles has completed, should we check first?
        self.usibr.read().bits()
    }

    fn raw_write(&mut self, byte: u8) {
        self.usidr.write(|w| {
            w.bits(byte)
        });

        self.usisr.write(|w| {
            w.usioif().set_bit()
        });

        while self.usisr.read().usioif().bit_is_clear() {
            self.usicr.write(|w| {
                // XXX WM and CS also need to be written to for it to work on my end
                w.usiwm().three_wire();
                w.usics().ext_pos();
                w.usiclk().set_bit();
                w.usitc().set_bit()
            });
            avr_device::asm::nop();
            avr_device::asm::nop();
            avr_device::asm::nop();
        }
    }
}
