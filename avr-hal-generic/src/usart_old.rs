//! Serial Implementations

use core::cmp::Ordering;

// Clock is needed because the calculations needs to take core clock into account
#[derive(Debug, Clone, Copy)]
pub struct Baudrate<CLOCK> {
    pub ubrr: u16,
    pub u2x: bool,
    pub _clock: ::core::marker::PhantomData<CLOCK>,
}

impl<CLOCK: crate::clock::Clock> PartialEq for Baudrate<CLOCK> {
    fn eq(&self, other: &Self) -> bool {
        self.compare_value() == other.compare_value()
    }
}

impl<CLOCK: crate::clock::Clock> Eq for Baudrate<CLOCK> {}

impl<CLOCK: crate::clock::Clock> PartialOrd for Baudrate<CLOCK> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare_value().cmp(&other.compare_value()))
    }
}

impl<CLOCK: crate::clock::Clock> Ord for Baudrate<CLOCK> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.compare_value().cmp(&self.compare_value())
    }
}

impl<CLOCK: crate::clock::Clock> From<u32> for Baudrate<CLOCK> {
    fn from(baud: u32) -> Self {
        Baudrate::new(baud)
    }
}

impl<CLOCK: crate::clock::Clock> Baudrate<CLOCK> {
    pub fn new(baud: u32) -> Baudrate<CLOCK> {
        let mut ubrr = (CLOCK::FREQ / 4 / baud - 1) / 2;
        let mut u2x = true;
        debug_assert!(ubrr <= u16::MAX as u32);
        if ubrr > 4095 {
            u2x = false;
            ubrr = (CLOCK::FREQ / 8 / baud - 1) / 2;
        }

        Baudrate {
            ubrr: ubrr as u16,
            u2x: u2x,
            _clock: ::core::marker::PhantomData,
        }
    }

    pub fn with_exact(u2x: bool, ubrr: u16) -> Baudrate<CLOCK> {
        Baudrate {
            ubrr, u2x, _clock: ::core::marker::PhantomData,
        }
    }

    fn compare_value(&self) -> u32 {
        if self.u2x {
            return 8 * (self.ubrr as u32 + 1);
        } else {
            return 16 * (self.ubrr as u32 + 1);
        };
    }
}

pub trait BaudrateExt {
    fn into_baudrate<CLOCK: crate::clock::Clock>(self) -> Baudrate<CLOCK>;
}

impl BaudrateExt for u32 {
    fn into_baudrate<CLOCK: crate::clock::Clock>(self) -> Baudrate<CLOCK> {
        Baudrate::new(self)
    }
}

pub trait BaudrateArduinoExt {
    fn into_baudrate<CLOCK: crate::clock::Clock>(self) -> Baudrate<CLOCK>;
}

impl BaudrateArduinoExt for u32 {
    fn into_baudrate<CLOCK: crate::clock::Clock>(self) -> Baudrate<CLOCK> {
        let br = Baudrate::new(self);

        // hardcoded exception for 57600 for compatibility with the bootloader
        // shipped with the Duemilanove and previous boards and the firmware
        // on the 8U2 on the Uno and Mega 2560.
        //
        // https://github.com/arduino/ArduinoCore-avr/blob/3055c1efa3c6980c864f661e6c8cc5d5ac773af4/cores/arduino/HardwareSerial.cpp#L123-L132
        if CLOCK::FREQ == 16_000_000 && br.ubrr == 34 && br.u2x {
            // (CLOCK::FREQ / 8 / 57600 - 1) / 2 == 16
            Baudrate::with_exact(false, 16)
        } else {
            br
        }
    }
}

/// Implement serial traits for a USART peripheral
#[macro_export]
macro_rules! impl_usart {
    (
        $(#[$usart_attr:meta])*
        pub struct $Usart:ident {
            peripheral: $USART:ty,
            pins: {
                rx: $rxmod:ident::$RX:ident,
                tx: $txmod:ident::$TX:ident,
            },
            register_suffix: $n:expr,
        }
    ) => {$crate::paste::paste! {
        $(#[$usart_attr])*
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
            /// Initialize the USART peripheral
            ///
            /// Please note that not all baudrates will produce a good signal
            /// and setting it too high might make data sent completely unreadable
            /// for the other side.
            pub fn new(
                p: $USART,
                rx: $rxmod::$RX<$crate::port::mode::Input<RX_MODE>>,
                tx: $txmod::$TX<$crate::port::mode::Output>,
                baud: $crate::usart::Baudrate<CLOCK>,
            ) -> $Usart<CLOCK, RX_MODE> {
                let mut usart = $Usart {
                    p,
                    rx,
                    tx,
                    _clock: ::core::marker::PhantomData,
                };
                usart.initialize(baud);
                usart
            }

            fn initialize(&mut self, baud: $crate::usart::Baudrate<CLOCK>) {
                self.p.[<ubrr $n>].write(|w| unsafe { w.bits(baud.ubrr) });
                self.p.[<ucsr $n a>].write(|w| w.[<u2x $n>]().bit(baud.u2x));

                // Enable receiver and transmitter but leave interrupts disabled.
                self.p.[<ucsr $n b>].write(|w| w
                    .[<txen $n>]().set_bit()
                    .[<rxen $n>]().set_bit()
                );

                // Set frame format to 8n1 for now.  At some point, this should be made
                // configurable, similar to what is done in other HALs.
                self.p.[<ucsr $n c>].write(|w| w
                    .[<umsel $n>]().usart_async()
                    .[<ucsz $n>]().chr8()
                    .[<usbs $n>]().stop1()
                    .[<upm $n>]().disabled()
                );
            }

            /// Enable/disable "RX Complete" interrupt
            ///
            /// When this interrupt triggers, new data is available to be read from the
            /// data-register.  The corresponding ISR is `USARTi_RX` (where `i` is this
            /// peripheral's number).  For example, for `USART1` on `ATmega32U4`:
            ///
            /// ```
            /// #[avr_device::interrupt(atmega32u4)]
            /// fn USART1_RX() {
            ///     // ...
            /// }
            /// ```
            pub fn interrupt_rxc(&mut self, state: bool) {
                self.p.[<ucsr $n b>].modify(|_, w| w.[<rxcie $n>]().bit(state));
            }

            /// Enable/disable "USART Data-Register Empty" interrupt
            ///
            /// This interrupt signals that new data can be written to the data-register.  The
            /// corresponding ISR is `USARTi_UDRE` (where `i` is this peripheral's number).  For
            /// example, for `USART1` on `ATmega32U4`:
            ///
            /// ```
            /// #[avr_device::interrupt(atmega32u4)]
            /// fn USART1_UDRE() {
            ///     // ...
            /// }
            /// ```
            pub fn interrupt_udre(&mut self, state: bool) {
                self.p.[<ucsr $n b>].modify(|_, w| w.[<txcie $n>]().bit(state));
            }

            /// Helper method for splitting this read/write object into two halves.
            ///
            /// The two halves returned implement the `Read` and `Write` traits, respectively.
            pub fn split(self) -> ([<Read $Usart>]<CLOCK, RX_MODE>, [<Write $Usart>]<CLOCK>) {
                (
                    [<Read $Usart>] {
                        p: unsafe { ::core::ptr::read(&self.p) },
                        rx: self.rx,
                        _clock: self._clock,
                    },
                    [<Write $Usart>] {
                        p: self.p,
                        tx: self.tx,
                        _clock: self._clock,
                    }
                )
            }
        }

        impl<CLOCK, RX_MODE> $crate::hal::serial::Write<u8> for $Usart<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            type Error = $crate::void::Void;

            fn write(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                // Call flush to make sure the data-register is empty
                self.flush()?;

                self.p.[<udr $n>].write(|w| unsafe { w.bits(byte) });
                Ok(())
            }

            fn flush(&mut self) -> $crate::nb::Result<(), Self::Error> {
                if self.p.[<ucsr $n a>].read().[<udre $n>]().bit_is_clear() {
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
            type Error = $crate::void::Void;

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
            type Error = $crate::void::Void;

            fn read(&mut self) -> $crate::nb::Result<u8, Self::Error> {
                if self.p.[<ucsr $n a>].read().[<rxc $n>]().bit_is_clear() {
                    return Err($crate::nb::Error::WouldBlock);
                }

                Ok(self.p.[<udr $n>].read().bits())
            }
        }

        /// The readable half of the
        $(#[$usart_attr])*
        pub struct [<Read $Usart>]<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            p: $USART,
            rx: $rxmod::$RX<$crate::port::mode::Input<RX_MODE>>,
            _clock: ::core::marker::PhantomData<CLOCK>,
        }

        /// The writable half of the
        $(#[$usart_attr])*
        pub struct [<Write $Usart>]<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
        {
            p: $USART,
            tx: $txmod::$TX<$crate::port::mode::Output>,
            _clock: ::core::marker::PhantomData<CLOCK>,
        }

        impl<CLOCK, RX_MODE> [<Read $Usart>]<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            /// Puts the two "halves" of a split `Read + Write` back together.
            pub fn reunite(self, other: [<Write $Usart>]<CLOCK>) -> $Usart<CLOCK, RX_MODE> {
                $Usart {
                    p: self.p,
                    rx: self.rx,
                    tx: other.tx,
                    _clock: self._clock,
                }
            }
        }

        impl<CLOCK> [<Write $Usart>]<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
        {
            /// Puts the two "halves" of a split `Read + Write` back together.
            pub fn reunite<RX_MODE>(self, other: [<Read $Usart>]<CLOCK, RX_MODE>) -> $Usart<CLOCK, RX_MODE>
            where
                RX_MODE: $crate::port::mode::InputMode,
            {
                other.reunite(self)
            }
        }

        impl<CLOCK> $crate::hal::serial::Write<u8> for [<Write $Usart>]<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
        {
            type Error = $crate::void::Void;

            fn write(&mut self, byte: u8) -> $crate::nb::Result<(), Self::Error> {
                // Call flush to make sure the data-register is empty
                self.flush()?;

                self.p.[<udr $n>].write(|w| unsafe { w.bits(byte) });
                Ok(())
            }

            fn flush(&mut self) -> $crate::nb::Result<(), Self::Error> {
                if self.p.[<ucsr $n a>].read().[<udre $n>]().bit_is_clear() {
                    Err($crate::nb::Error::WouldBlock)
                } else {
                    Ok(())
                }
            }
        }

        impl<CLOCK> $crate::ufmt::uWrite for [<Write $Usart>]<CLOCK>
        where
            CLOCK: $crate::clock::Clock,
        {
            type Error = $crate::void::Void;

            fn write_str(&mut self, s: &str) -> ::core::result::Result<(), Self::Error> {
                use $crate::prelude::*;

                for b in s.as_bytes().iter() {
                    $crate::nb::block!(self.write(*b))?;
                }
                Ok(())
            }
        }

        impl<CLOCK, RX_MODE> $crate::hal::serial::Read<u8> for [<Read $Usart>]<CLOCK, RX_MODE>
        where
            CLOCK: $crate::clock::Clock,
            RX_MODE: $crate::port::mode::InputMode,
        {
            type Error = $crate::void::Void;

            fn read(&mut self) -> $crate::nb::Result<u8, Self::Error> {
                if self.p.[<ucsr $n a>].read().[<rxc $n>]().bit_is_clear() {
                    return Err($crate::nb::Error::WouldBlock);
                }

                Ok(self.p.[<udr $n>].read().bits())
            }
        }
    }}
}
