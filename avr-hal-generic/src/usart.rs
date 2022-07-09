//! HAL abstractions for USART/Serial
//!
//! Check the documentation of [`Usart`] for details.

use core::cmp::Ordering;
use core::marker;
use void::ResultVoidExt;

use crate::port;

/// Representation of a USART baudrate
///
/// Precalculated parameters for configuring a certain USART baudrate.
#[derive(Debug, Clone, Copy)]
pub struct Baudrate<CLOCK> {
    /// Value of the `UBRR#` register
    pub ubrr: u16,
    /// Value of the `U2X#` bit
    pub u2x: bool,
    /// The baudrate calculation depends on the configured clock rate, thus a `CLOCK` generic
    /// parameter is needed.
    pub _clock: marker::PhantomData<CLOCK>,
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
    /// Calculate parameters for a certain baudrate at a certain `CLOCK` speed.
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
            u2x,
            _clock: ::core::marker::PhantomData,
        }
    }

    /// Construct a `Baudrate` from given `UBRR#` and `U2X#` values.
    ///
    /// This provides exact control over the resulting clock speed.
    pub fn with_exact(u2x: bool, ubrr: u16) -> Baudrate<CLOCK> {
        Baudrate {
            ubrr,
            u2x,
            _clock: ::core::marker::PhantomData,
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

/// Provide a `into_baudrate()` method for integers.
///
/// This extension trait allows conveniently initializing a baudrate by using
///
/// ```
/// let mut serial = arduino_uno::Serial::new(
///     dp.USART0,
///     pins.d0,
///     pins.d1.into_output(&mut pins.ddr),
///     57600.into_baudrate(),
/// );
/// ```
///
/// instead of having to call [`Baudrate::new(57600)`](Baudrate::new).
pub trait BaudrateExt {
    /// Calculate baudrate parameters from this number.
    fn into_baudrate<CLOCK: crate::clock::Clock>(self) -> Baudrate<CLOCK>;
}

impl BaudrateExt for u32 {
    fn into_baudrate<CLOCK: crate::clock::Clock>(self) -> Baudrate<CLOCK> {
        Baudrate::new(self)
    }
}

/// Same as [`BaudrateExt`] but accounts for an errata of certain Arduino boards:
///
/// The affected boards where this trait should be used instead are:
///
/// - Duemilanove
/// - Uno
/// - Mega 2560
pub trait BaudrateArduinoExt {
    /// Calculate baudrate parameters from this number (with Arduino errata).
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

/// Events/Interrupts for USART peripherals
#[repr(u8)]
pub enum Event {
    /// A complete byte was received.
    ///
    /// Corresponds to the `USART_RX` or `USART#_RX` interrupt.  Please refer to the datasheet for
    /// your MCU for details.
    RxComplete,

    /// A complete byte was sent.
    ///
    /// Corresponds to the `USART_TX` or `USART#_TX` interrupt.  Please refer to the datasheet for
    /// your MCU for details.
    TxComplete,

    /// All data from the USART data register was transmitted.
    ///
    /// Corresponds to the `USART_UDRE` or `USART#_UDRE` interrupt.  Please refer to the datasheet
    /// for your MCU for details.
    DataRegisterEmpty,
}

/// Internal trait for low-level USART peripherals.
///
/// This trait defines the common interface for all USART peripheral variants.  It is used as an
/// intermediate abstraction ontop of which the [`Usart`] API is built.  **Prefer using the
/// [`Usart`] API instead of this trait.**
pub trait UsartOps<H, RX, TX> {
    /// Enable & initialize this USART peripheral to the given baudrate.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_init<CLOCK>(&mut self, baudrate: Baudrate<CLOCK>);
    /// Disable this USART peripheral such that the pins can be used for other purposes again.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_deinit(&mut self);

    /// Flush all remaining data in the TX buffer.
    ///
    /// This operation must be non-blocking and return [`nb::Error::WouldBlock`] if not all data
    /// was flushed yet.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_flush(&mut self) -> nb::Result<(), void::Void>;
    /// Write a byte to the TX buffer.
    ///
    /// This operation must be non-blocking and return [`nb::Error::WouldBlock`] until the byte is
    /// enqueued.  The operation should not wait for the byte to have actually been sent.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_write(&mut self, byte: u8) -> nb::Result<(), void::Void>;
    /// Read a byte from the RX buffer.
    ///
    /// This operation must be non-blocking and return [`nb::Error::WouldBlock`] if no incoming
    /// byte is available.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_read(&mut self) -> nb::Result<u8, void::Void>;

    /// Enable/Disable a certain interrupt.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_interrupt(&mut self, event: Event, state: bool);
}

/// USART/Serial driver
///
/// # Example
/// (This example is taken from Arduino Uno)
/// ```
/// let dp = arduino_uno::Peripherals::take().unwrap();
/// let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
/// let mut serial = arduino_uno::Serial::new(
///     dp.USART0,
///     pins.d0,
///     pins.d1.into_output(&mut pins.ddr),
///     57600.into_baudrate(),
/// );
///
/// ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
///
/// loop {
///     let b = nb::block!(serial.read()).void_unwrap();
///     ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
/// }
/// ```
pub struct Usart<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> {
    p: USART,
    rx: RX,
    tx: TX,
    _clock: marker::PhantomData<CLOCK>,
    _h: marker::PhantomData<H>,
}

impl<H, USART, RXPIN, TXPIN, CLOCK>
    Usart<
        H,
        USART,
        port::Pin<port::mode::Input, RXPIN>,
        port::Pin<port::mode::Output, TXPIN>,
        CLOCK,
    >
where
    USART: UsartOps<H, port::Pin<port::mode::Input, RXPIN>, port::Pin<port::mode::Output, TXPIN>>,
    RXPIN: port::PinOps,
    TXPIN: port::PinOps,
{
    /// Initialize a USART peripheral on the given pins.
    ///
    /// Note that the RX and TX pins are hardwired for each USART peripheral and you *must* pass
    /// the correct ones.  This is enforced at compile time.
    pub fn new<IMODE: port::mode::InputMode>(
        p: USART,
        rx: port::Pin<port::mode::Input<IMODE>, RXPIN>,
        tx: port::Pin<port::mode::Output, TXPIN>,
        baudrate: Baudrate<CLOCK>,
    ) -> Self {
        let mut usart = Self {
            p,
            rx: rx.forget_imode(),
            tx,
            _clock: marker::PhantomData,
            _h: marker::PhantomData,
        };
        usart.p.raw_init(baudrate);
        usart
    }
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> Usart<H, USART, RX, TX, CLOCK> {
    /// Deinitialize/disable this peripheral and release the pins.
    pub fn release(mut self) -> (USART, RX, TX) {
        self.p.raw_deinit();
        (self.p, self.rx, self.tx)
    }

    /// Block until all remaining data has been transmitted.
    pub fn flush(&mut self) {
        nb::block!(self.p.raw_flush()).void_unwrap()
    }

    /// Transmit a byte.
    ///
    /// This method will block until the byte has been enqueued for transmission but **not** until
    /// it was entirely sent.
    pub fn write_byte(&mut self, byte: u8) {
        nb::block!(self.p.raw_write(byte)).void_unwrap()
    }

    /// Receive a byte.
    ///
    /// This method will block until a byte could be received.
    pub fn read_byte(&mut self) -> u8 {
        nb::block!(self.p.raw_read()).void_unwrap()
    }

    /// Enable the interrupt for [`Event`].
    pub fn listen(&mut self, event: Event) {
        self.p.raw_interrupt(event, true);
    }

    /// Disable the interrupt for [`Event`].
    pub fn unlisten(&mut self, event: Event) {
        self.p.raw_interrupt(event, false);
    }

    /// Split this USART into a [`UsartReader`] and a [`UsartWriter`].
    ///
    /// This allows concurrently receiving and transmitting data from different contexts.
    pub fn split(
        self,
    ) -> (
        UsartReader<H, USART, RX, TX, CLOCK>,
        UsartWriter<H, USART, RX, TX, CLOCK>,
    ) {
        (
            UsartReader {
                p: unsafe { core::ptr::read(&self.p) },
                rx: self.rx,
                _tx: marker::PhantomData,
                _clock: marker::PhantomData,
                _h: marker::PhantomData,
            },
            UsartWriter {
                p: self.p,
                tx: self.tx,
                _rx: marker::PhantomData,
                _clock: marker::PhantomData,
                _h: marker::PhantomData,
            },
        )
    }
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> ufmt::uWrite for Usart<H, USART, RX, TX, CLOCK> {
    type Error = void::Void;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for b in s.as_bytes().iter() {
            self.write_byte(*b);
        }
        Ok(())
    }
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> hal::serial::Write<u8>
    for Usart<H, USART, RX, TX, CLOCK>
{
    type Error = void::Void;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.p.raw_write(byte)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.p.raw_flush()
    }
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> hal::serial::Read<u8>
    for Usart<H, USART, RX, TX, CLOCK>
{
    type Error = void::Void;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.p.raw_read()
    }
}

/// Writer half of a [`Usart`] peripheral.
///
/// Created by calling [`Usart::split`].  Splitting a peripheral into reader and writer allows
/// concurrently receiving and transmitting data from different contexts.
///
/// The writer half most notably implements [`embedded_hal::serial::Write`] and [`ufmt::uWrite`]
/// for transmitting data.
pub struct UsartWriter<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> {
    p: USART,
    tx: TX,
    _rx: marker::PhantomData<RX>,
    _clock: marker::PhantomData<CLOCK>,
    _h: marker::PhantomData<H>,
}

/// Reader half of a [`Usart`] peripheral.
///
/// Created by calling [`Usart::split`].  Splitting a peripheral into reader and writer allows
/// concurrently receiving and transmitting data from different contexts.
///
/// The reader half most notably implements [`embedded_hal::serial::Read`] for receiving data.
pub struct UsartReader<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> {
    p: USART,
    rx: RX,
    _tx: marker::PhantomData<TX>,
    _clock: marker::PhantomData<CLOCK>,
    _h: marker::PhantomData<H>,
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> UsartWriter<H, USART, RX, TX, CLOCK> {
    /// Merge this `UsartWriter` with a [`UsartReader`] back into a single [`Usart`] peripheral.
    pub fn reunite(
        self,
        other: UsartReader<H, USART, RX, TX, CLOCK>,
    ) -> Usart<H, USART, RX, TX, CLOCK> {
        Usart {
            p: self.p,
            rx: other.rx,
            tx: self.tx,
            _clock: marker::PhantomData,
            _h: marker::PhantomData,
        }
    }
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> UsartReader<H, USART, RX, TX, CLOCK> {
    /// Merge this `UsartReader` with a [`UsartWriter`] back into a single [`Usart`] peripheral.
    pub fn reunite(
        self,
        other: UsartWriter<H, USART, RX, TX, CLOCK>,
    ) -> Usart<H, USART, RX, TX, CLOCK> {
        Usart {
            p: self.p,
            rx: self.rx,
            tx: other.tx,
            _clock: marker::PhantomData,
            _h: marker::PhantomData,
        }
    }
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> ufmt::uWrite
    for UsartWriter<H, USART, RX, TX, CLOCK>
{
    type Error = void::Void;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for b in s.as_bytes().iter() {
            nb::block!(self.p.raw_write(*b)).void_unwrap()
        }
        Ok(())
    }
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> hal::serial::Write<u8>
    for UsartWriter<H, USART, RX, TX, CLOCK>
{
    type Error = void::Void;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.p.raw_write(byte)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.p.raw_flush()
    }
}

impl<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK> hal::serial::Read<u8>
    for UsartReader<H, USART, RX, TX, CLOCK>
{
    type Error = void::Void;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.p.raw_read()
    }
}

#[macro_export]
macro_rules! impl_usart_traditional {
    (
        hal: $HAL:ty,
        peripheral: $USART:ty,
        register_suffix: $n:expr,
        rx: $rxpin:ty,
        tx: $txpin:ty,
    ) => {
        $crate::paste::paste! {
            impl $crate::usart::UsartOps<
                $HAL,
                $crate::port::Pin<$crate::port::mode::Input, $rxpin>,
                $crate::port::Pin<$crate::port::mode::Output, $txpin>,
            > for $USART {
                fn raw_init<CLOCK>(&mut self, baudrate: $crate::usart::Baudrate<CLOCK>) {
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

                fn raw_deinit(&mut self) {
                    // Wait for any ongoing transfer to finish.
                    $crate::nb::block!(self.raw_flush()).ok();
                    self.[<ucsr $n b>].reset();
                }

                fn raw_flush(&mut self) -> $crate::nb::Result<(), $crate::void::Void> {
                    if self.[<ucsr $n a>].read().[<udre $n>]().bit_is_clear() {
                        Err($crate::nb::Error::WouldBlock)
                    } else {
                        Ok(())
                    }
                }

                fn raw_write(&mut self, byte: u8) -> $crate::nb::Result<(), $crate::void::Void> {
                    // Call flush to make sure the data-register is empty
                    self.raw_flush()?;

                    self.[<udr $n>].write(|w| unsafe { w.bits(byte) });
                    Ok(())
                }

                fn raw_read(&mut self) -> $crate::nb::Result<u8, $crate::void::Void> {
                    if self.[<ucsr $n a>].read().[<rxc $n>]().bit_is_clear() {
                        return Err($crate::nb::Error::WouldBlock);
                    }

                    Ok(self.[<udr $n>].read().bits())
                }

                fn raw_interrupt(&mut self, event: $crate::usart::Event, state: bool) {
                    match event {
                        $crate::usart::Event::RxComplete =>
                            self.[<ucsr $n b>].modify(|_, w| w.[<rxcie $n>]().bit(state)),
                        $crate::usart::Event::TxComplete =>
                            self.[<ucsr $n b>].modify(|_, w| w.[<txcie $n>]().bit(state)),
                        $crate::usart::Event::DataRegisterEmpty =>
                            self.[<ucsr $n b>].modify(|_, w| w.[<udrie $n>]().bit(state)),
                    }
                }
            }
        }
    };
}
