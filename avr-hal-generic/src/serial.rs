#[derive(Debug, Clone, Copy)]
pub enum Error { }

#[macro_export]
macro_rules! impl_usart {
    (
        pub struct $Usart:ident {
            peripheral: $USART:ty,
            pins: {
                rx: $rxmod:ident::$RX:ident,
                tx: $txmod:ident::$TX:ident,
            },
            registers: {
                control_a: $control_a:ident {
                    data_empty: $dre:ident,
                    recv_complete: $rxc:ident,
                },
                control_b: $control_b:ident {
                    tx_enable: $txen:ident,
                    rx_enable: $rxen:ident,
                },
                control_c: $control_c:ident {
                    mode: $umode:ident,
                    char_size: $csz:ident,
                    stop_bits: $sbs:ident,
                    parity: $par:ident,
                },
                baud: $baud:ident,
                data: $data:ident,
            },
        }
    ) => {
        pub struct $Usart<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            p: $USART,
            rx: $rxmod::$RX<$crate::port::mode::Input<RX_MODE>>,
            tx: $txmod::$TX<$crate::port::mode::Output>,
            _clock: ::core::marker::PhantomData<CLOCK>,
        }

        impl<CLOCK, RX_MODE> $Usart<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            pub fn new(
                p: $USART,
                rx: $rxmod::$RX<$crate::port::mode::Input<RX_MODE>>,
                tx: $txmod::$TX<$crate::port::mode::Output>,
                baud: u32,
            ) -> $Usart<CLOCK, RX_MODE> {
                // Calculate BRR value
                let brr = CLOCK::FREQ / (16 * baud) - 1;
                // Set baudrate
                p.$baud.write(|w| w.bits(brr as u16));
                // Enable receiver and transmitter
                p.$control_b
                    .write(|w| w.$txen().set_bit().$rxen().set_bit());
                // Set frame format (8n1)
                p.$control_c.write(|w| {
                    w.$umode()
                        .usart_async()
                        .$csz()
                        .chr8()
                        .$sbs()
                        .stop1()
                        .$par()
                        .disabled()
                });

                $Usart {
                    p,
                    rx,
                    tx,
                    _clock: ::core::marker::PhantomData,
                }
            }
        }

        impl<CLOCK, RX_MODE> $crate::hal::serial::Write<u8> for $Usart<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            type Error = $crate::serial::Error;

            fn write(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                // Call flush to make sure the data-register is empty
                self.flush()?;

                self.p.$data.write(|w| w.bits(byte));
                Ok(())
            }

            fn flush(&mut self) -> $crate::nb::Result<(), Self::Error> {
                if self.p.$control_a.read().$dre().bit_is_clear() {
                    Err($crate::nb::Error::WouldBlock)
                } else {
                    Ok(())
                }
            }
        }

        impl<CLOCK, RX_MODE> $crate::ufmt::uWrite for $Usart<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            type Error = $crate::serial::Error;

            fn write_str(&mut self, s: &str) -> ::core::result::Result<(), Self::Error> {
                use $crate::prelude::*;

                for b in s.as_bytes().iter() {
                    $crate::nb::block!(self.write(*b))?;
                }
                Ok(())
            }
        }

        impl<CLOCK, RX_MODE> $crate::hal::serial::Read<u8> for $Usart<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            type Error = $crate::serial::Error;

            fn read(&mut self) -> $crate::nb::Result<u8, Self::Error> {
                if self.p.$control_a.read().$rxc().bit_is_clear() {
                    return Err($crate::nb::Error::WouldBlock);
                }

                Ok(self.p.$data.read().bits())
            }
        }
    };
}
