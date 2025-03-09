#![allow(unused_macros)]

macro_rules! impl_mod_usart {
    (
        hal: crate::$hal:ident,
        interfaces: {$(
            $interface:ident: {
                peripheral: $peripheral:ident,
                rx: $rx:ident,
                tx: $tx:ident,
                impl!: $($impl_macro:ident)::+ $({
                    $($arg_name:ident: $arg_value:expr,)*
                })?,
            },
        )+},
    ) => {
        pub mod usart {
            //! USART
            //!
            //! # Example
            //!
            //! Complete example source code can be found in the repository:
            //! [`atmega2560-usart.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-usart.rs)
            //!
            //! *Note: [ufmt](https://crates.io/crates/ufmt/) is used instead of `core::fmt` because
            //! `core::fmt` code quickly grows too large for AVR platforms.*
            //!
            //! ```
            //! let dp = atmega_hal::Peripherals::take().unwrap();
            //! let pins = atmega_hal::pins!(dp);
            //!
            //! let mut serial = Usart::new(
            //!     dp.USART0,
            //!     pins.pe0,
            //!     pins.pe1.into_output(),
            //!     Baudrate::<crate::CoreClock>::new(57600),
            //! );
            //!
            //! ufmt::uwriteln!(&mut serial, "Hello from ATmega!").unwrap();
            //!
            //! loop {
            //!     // Read a byte from the serial connection
            //!     let b = nb::block!(serial.read()).unwrap();
            //!     // Answer
            //!     ufmt::uwriteln!(&mut serial, "Got {}!", b).unwrap();
            //! }
            //! ```

            #[allow(unused_imports)]
            use avr_hal_generic::paste::paste;
            
            pub type Usart<USART, RX, TX, CLOCK> =
                avr_hal_generic::usart::Usart<crate::$hal::Hal, USART, RX, TX, CLOCK>;
            pub type UsartWriter<USART, RX, TX, CLOCK> =
                avr_hal_generic::usart::UsartWriter<crate::$hal::Hal, USART, RX, TX, CLOCK>;
            pub type UsartReader<USART, RX, TX, CLOCK> =
                avr_hal_generic::usart::UsartReader<crate::$hal::Hal, USART, RX, TX, CLOCK>;

            pub use avr_hal_generic::usart::*;

            $(
                pub type $interface<CLOCK> = Usart<
                    crate::$hal::pac::$peripheral,
                    crate::$hal::port::Pin<crate::$hal::port::mode::Input, crate::$hal::port::$rx>,
                    crate::$hal::port::Pin<crate::$hal::port::mode::Output, crate::$hal::port::$tx>,
                    CLOCK,
                >;

                $($impl_macro)::+! {
                    hal: crate::$hal,
                    peripheral: $peripheral,
                    rx: $rx,
                    tx: $tx ,
                    $($($arg_name: $arg_value,)*)?
                }
            )+
        }
        pub use usart::Usart;
    }
}
pub(crate) use impl_mod_usart;

macro_rules! impl_usart_traditional {
    (
        hal: crate::$hal:ident,
        peripheral: $peripheral:ident,
        rx: $rx:ident,
        tx: $tx:ident,
        register_suffix: $register_suffix:expr,
    ) => {
        avr_hal_generic::impl_usart_traditional! {
            hal: crate::$hal::Hal,
            peripheral: crate::$hal::pac::$peripheral,
            register_suffix: $register_suffix,
            rx: crate::$hal::port::$rx,
            tx: crate::$hal::port::$tx,
        }
    }
}
pub(crate) use impl_usart_traditional;

// TODO: atmega8 USART is different from other atmegas
// implemented so far. It uses the same register address
// for UBRRH and UCSRC, the way to select which register
// to write to, msb has to be 1 (for UCSRC)
// or 0 (for UBRRH). Because of the same address,
// these two are exposed as functions instead of
// fields.
macro_rules! impl_usart_ubrrh_ucsrc {
    (
        hal: crate::$hal:ident,
        peripheral: $peripheral:ident,
        rx: $rx:ident,
        tx: $tx:ident,
    ) => {
        impl
        avr_hal_generic::usart::UsartOps<
                crate::$hal::Hal,
                crate::$hal::port::Pin<crate::$hal::port::mode::Input, crate::$hal::port::$rx>,
                crate::$hal::port::Pin<crate::$hal::port::mode::Output, crate::$hal::port::$tx>,
            > for crate::$hal::pac::$peripheral
        {
            fn raw_init<CLOCK>(&mut self, baudrate: avr_hal_generic::usart::Baudrate<CLOCK>) {
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

            fn raw_interrupt(&mut self, event: avr_hal_generic::usart::Event, state: bool) {
                match event {
                    avr_hal_generic::usart::Event::RxComplete => {
                        self.ucsrb.modify(|_, w| w.rxcie().bit(state))
                    }
                    avr_hal_generic::usart::Event::TxComplete => {
                        self.ucsrb.modify(|_, w| w.txcie().bit(state))
                    }
                    avr_hal_generic::usart::Event::DataRegisterEmpty => {
                        self.ucsrb.modify(|_, w| w.udrie().bit(state))
                    }
                }
            }
        }
    };
}
pub(crate) use impl_usart_ubrrh_ucsrc;
