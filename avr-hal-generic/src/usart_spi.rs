//! MSPIM Implimentation
use crate::{port::PinOps, spi};

// This module just implements a macro for SpiOps, since underlyingly, the Spi type can still be used since it just needs SpiOps

/// Dummy Pin for MPSPIM
pub struct UsartSPIDummyPin;

impl PinOps for UsartSPIDummyPin {
    type Dynamic = Self;

    fn into_dynamic(self) -> Self::Dynamic {
        self
    }

    unsafe fn out_set(&mut self) {}

    unsafe fn out_clear(&mut self) {}

    unsafe fn out_toggle(&mut self) {}

    unsafe fn out_get(&self) -> bool {
        false
    }

    unsafe fn in_get(&self) -> bool {
        true
    }

    unsafe fn make_output(&mut self) {}

    unsafe fn make_input(&mut self, _pull_up: bool) {}
}

pub type UsartSpi<H, USART, SCLKPIN, MOSIPIN, MISOPIN> =
    spi::Spi<H, USART, SCLKPIN, MOSIPIN, MISOPIN, UsartSPIDummyPin>;

// Implement SpiOps trait for USART
#[macro_export]
macro_rules! add_usart_spi {
    (
        hal: $HAL:ty,
        peripheral: $USART_SPI:ty,
        register_suffix: $n:expr,
        sclk: $sclkpin:ty,
        mosi: $mosipin:ty,
        miso: $misopin:ty,
    ) => {
        $crate::paste::paste! {
            // This is quite a messy way to get the doc string working properly... but it works!
            #[doc = concat!("**Clock:** `", stringify!($sclkpin), "`<br>**MOSI:** `", stringify!($mosipin), "`<br> **MISO:** `", stringify!($misopin), "`")]
            pub type [<Usart $n Spi>] = avr_hal_generic::usart_spi::UsartSpi<$HAL, $USART_SPI, $sclkpin, $mosipin, $misopin>;

            impl $crate::spi::SpiOps<$HAL, $sclkpin, $mosipin, $misopin, $crate::usart_spi::UsartSPIDummyPin> for $USART_SPI {
                fn raw_setup(&mut self, settings: &$crate::spi::Settings) {
                    use $crate::hal::spi;

                    // Setup control registers
                    // We start by setting the UBBRn to 0
                    self.[<ubrr $n>].write(|w| unsafe {w.bits(0)});

                    // We have to translate the character size register into the 2 bits which are the MSB/LSB and the phase
                    // 5 Bit Char = MSB and 1st
                    // 6 Bit Char = MSB and 2nd
                    // 7 Bit Char = LSB and 1st
                    // 8 Bit Char = LSB and 2nd
                    self.[<ucsr $n c>].write(|w| {
                        w.[<umsel $n>]().spi_master();

                        match settings.data_order {
                            $crate::spi::DataOrder::MostSignificantFirst => match settings.mode.phase {
                                spi::Phase::CaptureOnFirstTransition => w.[<ucsz $n>]().chr5(),
                                spi::Phase::CaptureOnSecondTransition => w.[<ucsz $n>]().chr6(),
                            },
                            $crate::spi::DataOrder::LeastSignificantFirst => match settings.mode.phase {
                                spi::Phase::CaptureOnFirstTransition => w.[<ucsz $n>]().chr7(),
                                spi::Phase::CaptureOnSecondTransition => w.[<ucsz $n>]().chr8(),
                            },
                        };

                        match settings.mode.polarity {
                            spi::Polarity::IdleLow => w.[<ucpol $n>]().clear_bit(),
                            spi::Polarity::IdleHigh => w.[<ucpol $n>]().set_bit(),
                        }
                    });

                    // Enable receiver and transmitter, and also the rec interrupt.
                    self.[<ucsr $n b>].write(|w| w
                        .[<txen $n>]().set_bit()
                        .[<rxen $n>]().set_bit()
                    );

                    // Set the clock divider for SPI clock.
                    self.[<ubrr $n>].write(|w| {
                        match settings.clock {
                            $crate::spi::SerialClockRate::OscfOver2 => w.bits(0),
                            $crate::spi::SerialClockRate::OscfOver4 => w.bits(1),
                            $crate::spi::SerialClockRate::OscfOver8 => w.bits(3),
                            $crate::spi::SerialClockRate::OscfOver16 => w.bits(7),
                            $crate::spi::SerialClockRate::OscfOver32 => w.bits(15),
                            $crate::spi::SerialClockRate::OscfOver64 => w.bits(31),
                            $crate::spi::SerialClockRate::OscfOver128 => w.bits(63),
                        }
                    });
                }

                fn raw_release(&mut self) {
                    self.[<ucsr $n c>].write(|w| w.[<umsel $n>]().usart_async());
                    self.[<ucsr $n b>].reset();
                }

                fn raw_check_iflag(&self) -> bool {
                    self.[<ucsr $n a>].read().[<rxc $n>]().bit_is_set()
                }

                fn raw_read(&self) -> u8 {
                    self.[<udr $n>].read().bits()
                }

                fn raw_write(&mut self, byte: u8) {
                    self.[<udr $n>].write(|w| unsafe { w.bits(byte) });
                }

                fn raw_transaction(&mut self, byte: u8) -> u8 {
                    self.raw_write(byte);
                    while !self.raw_check_iflag() {}
                    self.raw_read()
                }
            }
        }
    };
}
