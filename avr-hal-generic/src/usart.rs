//! Serial Implementations

use core::cmp::Ordering;
use core::marker;
use void::ResultVoidExt;

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

pub trait UsartOps<RX, TX> {
    /// Enable & initialize this USART peripheral to the given baudrate.
    fn init<CLOCK>(&mut self, baudrate: Baudrate<CLOCK>);
    /// Disable this USART peripheral such that the pins can be used for other purposes again.
    fn deinit(&mut self);

    /// Flush all remaining data in the TX buffer.
    ///
    /// This operation must be non-blocking and return [`nb::Error::WouldBlock`] if not all data
    /// was flushed yet.
    fn flush(&mut self) -> nb::Result<(), void::Void>;
    /// Write a byte to the TX buffer.
    ///
    /// This operation must be non-blocking and return [`nb::Error::WouldBlock`] until the byte is
    /// enqueued.  The operation should not wait for the byte to have actually been sent.
    fn write(&mut self, byte: u8) -> nb::Result<(), void::Void>;
    /// Read a byte from the RX buffer.
    ///
    /// This operation must be non-blocking and return [`nb::Error::WouldBlock`] if no incoming
    /// byte is available.
    fn read(&mut self) -> nb::Result<u8, void::Void>;
}

pub struct Usart<USART: UsartOps<RX, TX>, RX, TX, CLOCK> {
    p: USART,
    rx: RX,
    tx: TX,
    _clock: marker::PhantomData<CLOCK>,
}

impl<USART: UsartOps<RX, TX>, RX, TX, CLOCK> Usart<USART, RX, TX, CLOCK> {
    pub fn new(p: USART, rx: RX, tx: TX, baudrate: Baudrate<CLOCK>) -> Self {
        let mut usart = Self {
            p,
            rx,
            tx,
            _clock: marker::PhantomData,
        };
        usart.p.init(baudrate);
        usart
    }

    pub fn release(mut self) -> (USART, RX, TX) {
        self.p.deinit();
        (self.p, self.rx, self.tx)
    }

    pub fn flush(&mut self) {
        nb::block!(self.p.flush()).void_unwrap()
    }

    pub fn write_byte(&mut self, byte: u8) {
        nb::block!(self.p.write(byte)).void_unwrap()
    }

    pub fn read_byte(&mut self) -> u8 {
        nb::block!(self.p.read()).void_unwrap()
    }
}

impl<USART: UsartOps<RX, TX>, RX, TX, CLOCK> ufmt::uWrite for Usart<USART, RX, TX, CLOCK> {
    type Error = void::Void;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for b in s.as_bytes().iter() {
            self.write_byte(*b);
        }
        Ok(())
    }
}

impl<USART: UsartOps<RX, TX>, RX, TX, CLOCK> hal::serial::Write<u8>
    for Usart<USART, RX, TX, CLOCK>
{
    type Error = void::Void;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.p.write(byte)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.p.flush()
    }
}

impl<USART: UsartOps<RX, TX>, RX, TX, CLOCK> hal::serial::Read<u8> for Usart<USART, RX, TX, CLOCK> {
    type Error = void::Void;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.p.read()
    }
}

#[macro_export]
macro_rules! impl_usart_traditional {
    (
        peripheral: $USART:ty,
        register_suffix: $n:expr,
        rx: $rxmod:ident::$RX:ident,
        tx: $txmod:ident::$TX:ident,
    ) => {
        $crate::paste::paste! {
            impl $crate::usart::UsartOps<
                $rxmod::$RX<$crate::port::mode::Input<$crate::port::mode::Floating>>,
                $txmod::$TX<$crate::port::mode::Output>,
            > for $USART {
                fn init<CLOCK>(&mut self, baudrate: $crate::usart::Baudrate<CLOCK>) {
                    self.[<ubrr $n>].write(|w| unsafe { w.bits(baudrate.ubrr) });
                    self.[<ucsr $n a>].write(|w| w.[<u2x $n>]().bit(baudrate.u2x));

                    // Enable receiver and transmitter but leave interrupts disabled.
                    self.[<ucsr $n b>].write(|w| w
                        .[<txen $n>]().set_bit()
                        .[<rxen $n>]().set_bit()
                    );

                    // Set frame format to 8n1 for now.  At some point, this should be made
                    // configurable, similar to what is done in other HALs.
                    self.[<ucsr $n c>].write(|w| w
                        .[<umsel $n>]().usart_async()
                        .[<ucsz $n>]().chr8()
                        .[<usbs $n>]().stop1()
                        .[<upm $n>]().disabled()
                    );
                }

                fn deinit(&mut self) {
                    // Wait for any ongoing transfer to finish.
                    $crate::nb::block!(self.flush()).ok();
                    self.[<ucsr $n b>].reset();
                }

                fn flush(&mut self) -> $crate::nb::Result<(), $crate::void::Void> {
                    if self.[<ucsr $n a>].read().[<udre $n>]().bit_is_clear() {
                        Err($crate::nb::Error::WouldBlock)
                    } else {
                        Ok(())
                    }
                }

                fn write(&mut self, byte: u8) -> $crate::nb::Result<(), $crate::void::Void> {
                    // Call flush to make sure the data-register is empty
                    self.flush()?;

                    self.[<udr $n>].write(|w| unsafe { w.bits(byte) });
                    Ok(())
                }

                fn read(&mut self) -> $crate::nb::Result<u8, $crate::void::Void> {
                    if self.[<ucsr $n a>].read().[<rxc $n>]().bit_is_clear() {
                        return Err($crate::nb::Error::WouldBlock);
                    }

                    Ok(self.[<udr $n>].read().bits())
                }
            }
        }
    };
}
