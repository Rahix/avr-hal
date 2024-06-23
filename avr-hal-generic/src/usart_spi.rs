//! MSPIM Implimentation
use crate::spi;

// This module just impliments a macro for SpiOps, since underlyingly, the Spi type can still be used since it just needs SpiOps

pub type UsartSpi<H, USART, SCLKPIN, MOSIPIN, MISOPIN, CSPIN> =
    spi::Spi<H, USART, SCLKPIN, MOSIPIN, MISOPIN, CSPIN>;

// Impliment SpiOps trait for USART
#[macro_export]
macro_rules! impl_usart_spi {
    (
        hal: $HAL:ty,
        peripheral: $USART_SPI:ty,
        register_suffix: $n:expr,
        sclk: $sclkpin:ty,
        mosi: $mosipin:ty,
        miso: $misopin:ty,
        cs: $cspin:ty,
    ) => {
        $crate::paste::paste! {
            impl $crate::spi::SpiOps<$HAL, $sclkpin, $mosipin, $misopin, $cspin> for $USART_SPI {
                fn raw_setup(&mut self, settings: &Settings) {
                    use $crate::hal::spi;

                    // Setup control registers
                    // We start by setting the UBBRn to 0
                    self.[<ubrr $n>].write(|w| w.bits(0));

                    // We have to translate the character size register into the 2 bits which are the MSB/LSB and the phase
                    // 5 Bit Char = MSB and 1st
                    // 6 Bit Char = MSB and 2nd
                    // 7 Bit Char = LSB and 1st
                    // 8 Bit Char = LSB and 2nd
                    self.[<ucsr $n c>].write(|w| {
                        w.[<umsel $n>]().spi_master();

                        match settings.data_order {
                            DataOrder::MostSignificantFirst => match settings.mode.phase {
                                spi::Phase::CaptureOnFirstTransition => w.[<ucsz $n>]().chr5(),
                                spi::Phase::CaptureOnSecondTransition => w.[<ucsz $n>]().chr6(),
                            },
                            DataOrder::LeastSignificantFirst => match settings.mode.phase {
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
                        .[<rxcie $n>]().set_bit()
                    );

                    // Set the baudrate of the UBRRn, idk what it should be set to, so for now, it'll be set to 0
                    self.[<ubrr $n>].write(|w| w.bits(0));
                }

                fn raw_release(&mut self) {
                    // Probably a better way to "release" the SPI interface, but from the datasheet, this is what they suggest, so ig it works
                    self.[<ucsr $n c>].write(|w| w.[<umsel $n>]().usart_async());
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
