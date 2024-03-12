#[allow(unused_imports)]
use crate::port;
pub use avr_hal_generic::usart::*;

pub type Usart<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::Usart<crate::Atmega, USART, RX, TX, CLOCK>;
pub type UsartWriter<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::UsartWriter<crate::Atmega, USART, RX, TX, CLOCK>;
pub type UsartReader<USART, RX, TX, CLOCK> =
    avr_hal_generic::usart::UsartReader<crate::Atmega, USART, RX, TX, CLOCK>;

#[cfg(any(
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega328pb",
    feature = "atmega1284p",
    feature = "atmega164pa"
))]
pub type Usart0<CLOCK> = Usart<
    crate::pac::USART0,
    port::Pin<port::mode::Input, port::PD0>,
    port::Pin<port::mode::Output, port::PD1>,
    CLOCK,
>;
#[cfg(any(
    feature = "atmega168",
    feature = "atmega328p",
    feature = "atmega328pb",
    feature = "atmega1284p",
    feature = "atmega164pa"
))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    rx: port::PD0,
    tx: port::PD1,
}

#[cfg(feature = "atmega328pb")]
pub type Usart1<CLOCK> = Usart<
    crate::pac::USART1,
    port::Pin<port::mode::Input, port::PB4>,
    port::Pin<port::mode::Output, port::PB3>,
    CLOCK,
>;
#[cfg(feature = "atmega328pb")]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART1,
    register_suffix: 1,
    rx: port::PB4,
    tx: port::PB3,
}

#[cfg(any(
    feature = "atmega32u4",
    feature = "atmega128a",
    feature = "atmega1280",
    feature = "atmega2560",
    feature = "atmega1284p",
    feature = "atmega164pa"
))]
pub type Usart1<CLOCK> = Usart<
    crate::pac::USART1,
    port::Pin<port::mode::Input, port::PD2>,
    port::Pin<port::mode::Output, port::PD3>,
    CLOCK,
>;
#[cfg(any(
    feature = "atmega32u4",
    feature = "atmega1280",
    feature = "atmega2560",
    feature = "atmega1284p",
    feature = "atmega164pa"
))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART1,
    register_suffix: 1,
    rx: port::PD2,
    tx: port::PD3,
}

#[cfg(any(feature = "atmega128A", feature = "atmega1280", feature = "atmega2560"))]
pub type Usart0<CLOCK> = Usart<
    crate::pac::USART0,
    port::Pin<port::mode::Input, port::PE0>,
    port::Pin<port::mode::Output, port::PE1>,
    CLOCK,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART0,
    register_suffix: 0,
    rx: port::PE0,
    tx: port::PE1,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart2<CLOCK> = Usart<
    crate::pac::USART2,
    port::Pin<port::mode::Input, port::PH0>,
    port::Pin<port::mode::Output, port::PH1>,
    CLOCK,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART2,
    register_suffix: 2,
    rx: port::PH0,
    tx: port::PH1,
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
pub type Usart3<CLOCK> = Usart<
    crate::pac::USART3,
    port::Pin<port::mode::Input, port::PJ0>,
    port::Pin<port::mode::Output, port::PJ1>,
    CLOCK,
>;
#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
avr_hal_generic::impl_usart_traditional! {
    hal: crate::Atmega,
    peripheral: crate::pac::USART3,
    register_suffix: 3,
    rx: port::PJ0,
    tx: port::PJ1,
}

#[cfg(any(feature = "atmega8", feature = "atmega32a"))]
pub type Usart0<CLOCK> = Usart<
    crate::pac::USART,
    port::Pin<port::mode::Input, port::PD0>,
    port::Pin<port::mode::Output, port::PD1>,
    CLOCK,
>;

// TODO: atmega8 USART is different from other atmegas
// implemented so far. It uses the same register address
// for UBRRH and UCSRC, the way to select which register
// to write to, msb has to be 1 (for UCSRC)
// or 0 (for UBRRH). Because of the same address,
// these two are exposed as functions instead of
// fields.
#[cfg(any(feature = "atmega8", feature = "atmega32a"))]
impl
    crate::usart::UsartOps<
        crate::Atmega,
        crate::port::Pin<crate::port::mode::Input, port::PD0>,
        crate::port::Pin<crate::port::mode::Output, port::PD1>,
    > for crate::pac::USART
{
    fn raw_init<CLOCK>(&mut self, baudrate: crate::usart::Baudrate<CLOCK>) {
        // msb of ubrrh has to be 0 to set ubrrh register. (see atmega8 datasheet)
        let ubrrh: u8 = ((baudrate.ubrr >> 8) & 0x0F) as u8;
        let ubrrl: u8 = (baudrate.ubrr & 0xFF) as u8;
        self.ubrrh().write(|w| w.bits(ubrrh));
        self.ubrrl.write(|w| w.bits(ubrrl));
        self.ucsra.write(|w| w.u2x().bit(baudrate.u2x));

        // Enable receiver and transmitter but leave interrupts disabled.
        #[rustfmt::skip]
        self.ucsrb.write(|w| w
            .txen().set_bit()
            .rxen().set_bit()
        );

        // Set frame format to 8n1 for now.  At some point, this should be made
        // configurable, similar to what is done in other HALs.
        #[rustfmt::skip]
        self.ucsrc().write(|w| w
            .ursel().set_bit() // sets the ucsrc instead of ubrrh (ubrrh and ucsrc share same location on ATmega8, see atmega8 datasheet)
            .umsel().usart_async()
            .ucsz().chr8()
            .usbs().stop1()
            .upm().disabled()
        );
    }

    fn raw_deinit(&mut self) {
        // Wait for any ongoing transfer to finish.
        avr_hal_generic::nb::block!(self.raw_flush()).ok();
        self.ucsrb.reset();
    }

    fn raw_flush(&mut self) -> avr_hal_generic::nb::Result<(), core::convert::Infallible> {
        if self.ucsra.read().udre().bit_is_clear() {
            Err(avr_hal_generic::nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    fn raw_write(
        &mut self,
        byte: u8,
    ) -> avr_hal_generic::nb::Result<(), core::convert::Infallible> {
        // Call flush to make sure the data-register is empty
        self.raw_flush()?;

        self.udr.write(|w| w.bits(byte));
        Ok(())
    }

    fn raw_read(&mut self) -> avr_hal_generic::nb::Result<u8, core::convert::Infallible> {
        if self.ucsra.read().rxc().bit_is_clear() {
            return Err(avr_hal_generic::nb::Error::WouldBlock);
        }

        Ok(self.udr.read().bits())
    }

    fn raw_interrupt(&mut self, event: crate::usart::Event, state: bool) {
        match event {
            crate::usart::Event::RxComplete => self.ucsrb.modify(|_, w| w.rxcie().bit(state)),
            crate::usart::Event::TxComplete => self.ucsrb.modify(|_, w| w.txcie().bit(state)),
            crate::usart::Event::DataRegisterEmpty => {
                self.ucsrb.modify(|_, w| w.udrie().bit(state))
            }
        }
    }
}

// TODO: ATmega128A USART1 is also different from other atmegas
// Mainly needed because ubrr1 is split in ubrr1h and ubrr1l
#[cfg(any(feature = "atmega128a"))]
impl
    crate::usart::UsartOps<
        crate::Atmega,
        crate::port::Pin<crate::port::mode::Input, port::PD2>,
        crate::port::Pin<crate::port::mode::Output, port::PD3>,
    > for crate::pac::USART1
{
    fn raw_init<CLOCK>(&mut self, baudrate: crate::usart::Baudrate<CLOCK>) {
        let ubrr1h: u8 = (baudrate.ubrr >> 8) as u8;
        let ubrr1l: u8 = baudrate.ubrr as u8;
        self.ubrr1h.write(|w| w.bits(ubrr1h));
        self.ubrr1l.write(|w| w.bits(ubrr1l));
        self.ucsr1a.write(|w| w.u2x1().bit(baudrate.u2x));

        // Enable receiver and transmitter but leave interrupts disabled.
        #[rustfmt::skip]
        self.ucsr1b.write(|w| w
            .txen1().set_bit()
            .rxen1().set_bit()
        );

        // Set frame format to 8n1 for now.  At some point, this should be made
        // configurable, similar to what is done in other HALs.
        #[rustfmt::skip]
        self.ucsr1c.write(|w| w
            .umsel1().usart_async()
            .ucsz1().chr8()
            .usbs1().stop1()
            .upm1().disabled()
        );
    }

    fn raw_deinit(&mut self) {
        // Wait for any ongoing transfer to finish.
        avr_hal_generic::nb::block!(self.raw_flush()).ok();
        self.ucsr1b.reset();
    }

    fn raw_flush(&mut self) -> avr_hal_generic::nb::Result<(), core::convert::Infallible> {
        if self.ucsr1a.read().udre1().bit_is_clear() {
            Err(avr_hal_generic::nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    fn raw_write(
        &mut self,
        byte: u8,
    ) -> avr_hal_generic::nb::Result<(), core::convert::Infallible> {
        // Call flush to make sure the data-register is empty
        self.raw_flush()?;

        self.udr1.write(|w| w.bits(byte));
        Ok(())
    }

    fn raw_read(&mut self) -> avr_hal_generic::nb::Result<u8, core::convert::Infallible> {
        if self.ucsr1a.read().rxc1().bit_is_clear() {
            return Err(avr_hal_generic::nb::Error::WouldBlock);
        }

        Ok(self.udr1.read().bits())
    }

    fn raw_interrupt(&mut self, event: crate::usart::Event, state: bool) {
        match event {
            crate::usart::Event::RxComplete => self.ucsr1b.modify(|_, w| w.rxcie1().bit(state)),
            crate::usart::Event::TxComplete => self.ucsr1b.modify(|_, w| w.txcie1().bit(state)),
            crate::usart::Event::DataRegisterEmpty => {
                self.ucsr1b.modify(|_, w| w.udrie1().bit(state))
            }
        }
    }
}

// TODO: ATmega128A USART0 is also different from other atmegas
// Mainly needed because ubrr1 is split in ubrr1h and ubrr1l
// For USART0 they are not even close to eachother in memory
#[cfg(any(feature = "atmega128a"))]
impl
    crate::usart::UsartOps<
        crate::Atmega,
        crate::port::Pin<crate::port::mode::Input, port::PE0>,
        crate::port::Pin<crate::port::mode::Output, port::PE1>,
    > for crate::pac::USART0
{
    fn raw_init<CLOCK>(&mut self, baudrate: crate::usart::Baudrate<CLOCK>) {
        let ubrr0h: u8 = (baudrate.ubrr >> 8) as u8;
        let ubrr0l: u8 = baudrate.ubrr as u8;
        self.ubrr0h.write(|w| w.bits(ubrr0h));
        self.ubrr0l.write(|w| w.bits(ubrr0l));
        self.ucsr0a.write(|w| w.u2x0().bit(baudrate.u2x));

        // Enable receiver and transmitter but leave interrupts disabled.
        self.ucsr0b.write(|w| w.txen0().set_bit().rxen0().set_bit());

        // Set frame format to 8n1 for now.  At some point, this should be made
        // configurable, similar to what is done in other HALs.
        #[rustfmt::skip]
        self.ucsr0c.write(|w| w
            .umsel0().usart_async()
            .ucsz0().chr8()
            .usbs0().stop1()
            .upm0().disabled()
        );
    }

    fn raw_deinit(&mut self) {
        // Wait for any ongoing transfer to finish.
        avr_hal_generic::nb::block!(self.raw_flush()).ok();
        self.ucsr0b.reset();
    }

    fn raw_flush(&mut self) -> avr_hal_generic::nb::Result<(), core::convert::Infallible> {
        if self.ucsr0a.read().udre0().bit_is_clear() {
            Err(avr_hal_generic::nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }

    fn raw_write(
        &mut self,
        byte: u8,
    ) -> avr_hal_generic::nb::Result<(), core::convert::Infallible> {
        // Call flush to make sure the data-register is empty
        self.raw_flush()?;

        self.udr0.write(|w| w.bits(byte));
        Ok(())
    }

    fn raw_read(&mut self) -> avr_hal_generic::nb::Result<u8, core::convert::Infallible> {
        if self.ucsr0a.read().rxc0().bit_is_clear() {
            return Err(avr_hal_generic::nb::Error::WouldBlock);
        }

        Ok(self.udr0.read().bits())
    }

    fn raw_interrupt(&mut self, event: crate::usart::Event, state: bool) {
        match event {
            crate::usart::Event::RxComplete => self.ucsr0b.modify(|_, w| w.rxcie0().bit(state)),
            crate::usart::Event::TxComplete => self.ucsr0b.modify(|_, w| w.txcie0().bit(state)),
            crate::usart::Event::DataRegisterEmpty => {
                self.ucsr0b.modify(|_, w| w.udrie0().bit(state))
            }
        }
    }
}
