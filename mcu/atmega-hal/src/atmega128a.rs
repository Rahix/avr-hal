pub use avr_device::atmega128a as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    hal: crate::atmega128a,
    pins: {
        PF0: (hal::pac::adc::admux::MUX_A::ADC0),
        PF1: (hal::pac::adc::admux::MUX_A::ADC1),
        PF2: (hal::pac::adc::admux::MUX_A::ADC2),
        PF3: (hal::pac::adc::admux::MUX_A::ADC3),
        PF4: (hal::pac::adc::admux::MUX_A::ADC4),
        PF5: (hal::pac::adc::admux::MUX_A::ADC5),
        PF6: (hal::pac::adc::admux::MUX_A::ADC6),
        PF7: (hal::pac::adc::admux::MUX_A::ADC7),
    },
    channels: {
        #[cfg(feature = "enable-extra-adc")]
        ADC6: hal::pac::adc::admux::MUX_A::ADC6,
        #[cfg(feature = "enable-extra-adc")]
        ADC7: hal::pac::adc::admux::MUX_A::ADC7,
        Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
        Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
    },
    impl!: impl_adc_admux,
}

impl_mod_eeprom! {
    hal: crate::atmega128a,
    capacity: 4096,
    addr_width: u16,
    addr_reg: eear,
    impl!: avr_hal_generic::impl_eeprom_atmega_old,
}

impl_mod_i2c! {
    hal: crate::atmega128a,
    interfaces: {
        I2c: {
            peripheral: TWI,
            sda: PD1,
            scl: PD0,
        },
    },
}

impl_mod_port! {
    use crate::atmega128a as hal;

    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: hal::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
            B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            C: hal::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
            D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
            E: hal::pac::PORTE = [0, 1, 2, 3, 4, 5, 6, 7],
            F: hal::pac::PORTF = [0, 1, 2, 3, 4, 5, 6, 7],
            G: hal::pac::PORTG = [0, 1, 2, 3, 4],
        }
    }

    #[macro_export]
    macro_rules! atmega128a_pins {
        ($p:expr) => {
            $crate::atmega128a::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF, $p.PORTG)
        };
    }

    pub use atmega128a_pins as pins;
}

impl_mod_spi! {
    hal: crate::atmega128a,
    interfaces: {
        Spi: {
            peripheral: SPI,
            sclk: PB1,
            mosi: PB2,
            miso: PB3,
            cs: PB0,
        },
    },
}

impl_mod_usart! {
    hal: crate::atmega128a,
    interfaces: {
        Usart0: {
            peripheral: USART0,
            rx: PE0,
            tx: PE1,
            impl!: crate::atmega128a::impl_usart_atmega128a {
                register_suffix: 0,
            },
        },
        Usart1: {
            peripheral: USART1,
            rx: PD2,
            tx: PD3,
            impl!: crate::atmega128a::impl_usart_atmega128a {
                register_suffix: 1,
            },
        },
    },
}

impl_mod_wdt! {
    use crate::atmega128a as hal;
    impl_wdt_peripheral_ms2000! {
        mcusr: hal::pac::cpu::MCUCSR,
        wdtcsr_name: wdtcr,
    }
}

macro_rules! impl_usart_atmega128a {
    (
        hal: crate::$hal:ident,
        peripheral: $peripheral:ident,
        rx: $rx:ident,
        tx: $tx:ident,
        register_suffix: $register_suffix:literal,
    ) => {
        paste! {
            impl
            avr_hal_generic::usart::UsartOps<
                    crate::$hal::Hal,
                    crate::$hal::port::Pin<crate::$hal::port::mode::Input, crate::$hal::port::$rx>,
                    crate::$hal::port::Pin<crate::$hal::port::mode::Output, crate::$hal::port::$tx>,
                > for crate::$hal::pac::$peripheral
            {
                fn raw_init<CLOCK>(&mut self, baudrate: Baudrate<CLOCK>) {
                    let [< ubrr $register_suffix h >]: u8 = (baudrate.ubrr >> 8) as u8;
                    let [< ubrr $register_suffix l >]: u8 = baudrate.ubrr as u8;
                    self.[< ubrr $register_suffix h >].write(|w| w.bits([< ubrr $register_suffix h >]));
                    self.[< ubrr $register_suffix l >].write(|w| w.bits([< ubrr $register_suffix l >]));
                    self.[< ucsr $register_suffix a >].write(|w| w.[< u2x $register_suffix  >]().bit(baudrate.u2x));
        
                    // Enable receiver and transmitter but leave interrupts disabled.
                    #[rustfmt::skip]
                    self.[< ucsr $register_suffix b >].write(|w| w
                        .[< txen $register_suffix  >]().set_bit()
                        .[< rxen $register_suffix  >]().set_bit()
                    );
        
                    // Set frame format to [< 8n $register_suffix  >] for now.  At some point, this should be made
                    // configurable, similar to what is done in other HALs.
                    #[rustfmt::skip]
                    self.[< ucsr $register_suffix c >].write(|w| w
                        .[< umsel $register_suffix  >]().usart_async()
                        .[< ucsz $register_suffix  >]().chr8()
                        .[< usbs $register_suffix  >]().stop1()
                        .[< upm $register_suffix  >]().disabled()
                    );
                }
        
                fn raw_deinit(&mut self) {
                    // Wait for any ongoing transfer to finish.
                    avr_hal_generic::nb::block!(self.raw_flush()).ok();
                    self.[< ucsr $register_suffix b >].reset();
                }
        
                fn raw_flush(&mut self) -> avr_hal_generic::nb::Result<(), core::convert::Infallible> {
                    if self.[< ucsr $register_suffix a >].read().[< udre $register_suffix  >]().bit_is_clear() {
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
        
                    self.[< udr $register_suffix  >].write(|w| w.bits(byte));
                    Ok(())
                }
        
                fn raw_read(&mut self) -> avr_hal_generic::nb::Result<u8, core::convert::Infallible> {
                    if self.[< ucsr $register_suffix a >].read().[< rxc $register_suffix  >]().bit_is_clear() {
                        return Err(avr_hal_generic::nb::Error::WouldBlock);
                    }
        
                    Ok(self.[< udr $register_suffix  >].read().bits())
                }
        
                fn raw_interrupt(&mut self, event: Event, state: bool) {
                    match event {
                        Event::RxComplete => self.[< ucsr $register_suffix b >].modify(|_, w| w.[< rxcie $register_suffix  >]().bit(state)),
                        Event::TxComplete => self.[< ucsr $register_suffix b >].modify(|_, w| w.[< txcie $register_suffix  >]().bit(state)),
                        Event::DataRegisterEmpty => self.[< ucsr $register_suffix b >].modify(|_, w| w.[< udrie $register_suffix  >]().bit(state)),
                    }
                }
            }
        }
    };
}

pub(crate) use impl_usart_atmega128a;