pub use avr_device::atmega128a as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_adc! {
    use crate::atmega128a as hal;
    impl_adc_channels_extra!();
    impl_adc!();

    avr_hal_generic::impl_adc! {
        hal: hal::Hal,
        peripheral: hal::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| { apply_settings(peripheral, settings) },
        channel_id: hal::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            hal::port::PF0: (hal::pac::adc::admux::MUX_A::ADC0),
            hal::port::PF1: (hal::pac::adc::admux::MUX_A::ADC1),
            hal::port::PF2: (hal::pac::adc::admux::MUX_A::ADC2),
            hal::port::PF3: (hal::pac::adc::admux::MUX_A::ADC3),
            hal::port::PF4: (hal::pac::adc::admux::MUX_A::ADC4),
            hal::port::PF5: (hal::pac::adc::admux::MUX_A::ADC5),
            hal::port::PF6: (hal::pac::adc::admux::MUX_A::ADC6),
            hal::port::PF7: (hal::pac::adc::admux::MUX_A::ADC7),
        },
        channels: {
            channel::Vbg: hal::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: hal::pac::adc::admux::MUX_A::ADC_GND,
        },
    }
}

impl_mod_eeprom! {
    hal: crate::atmega128a,
    capacity: 4096,
    addr_width: u16,
    addr_reg: eear,
    variant: impl_eeprom_atmega_old,
}

impl_mod_i2c! {
    use crate::atmega128a as hal;
    impl_i2c_peripheral! {
        i2c_type: I2c,
        peripheral: hal::pac::TWI,
        sda: hal::port::PD1,
        scl: hal::port::PD0,
    }
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
    use crate::atmega128a as hal;
    impl_spi_peripheral! {
        spi: Spi,
        peripheral: hal::pac::SPI,
        sclk: hal::port::PB1,
        mosi: hal::port::PB2,
        miso: hal::port::PB3,
        cs: hal::port::PB0,
    }
}

impl_mod_usart! {
    use crate::atmega128a as hal;
    impl_usart_peripheral_type! {
        peripheral: hal::pac::USART0,
        rx: hal::port::PE0,
        tx: hal::port::PE1,
        usart_type: Usart0,
    }

    impl_usart_peripheral_type! {
        peripheral: hal::pac::USART1,
        rx: hal::port::PD2,
        tx: hal::port::PD3,
        usart_type: Usart1,
    }

    // TODO: ATmega128A USART1 is also different from other atmegas
    // Mainly needed because ubrr1 is split in ubrr1h and ubrr1l
    impl
        avr_hal_generic::usart::UsartOps<
            hal::Hal,
            hal::port::Pin<hal::port::mode::Input, hal::port::PD2>,
            hal::port::Pin<hal::port::mode::Output, hal::port::PD3>,
        > for hal::pac::USART1
    {
        fn raw_init<CLOCK>(&mut self, baudrate: Baudrate<CLOCK>) {
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

        fn raw_interrupt(&mut self, event: Event, state: bool) {
            match event {
                Event::RxComplete => self.ucsr1b.modify(|_, w| w.rxcie1().bit(state)),
                Event::TxComplete => self.ucsr1b.modify(|_, w| w.txcie1().bit(state)),
                Event::DataRegisterEmpty => self.ucsr1b.modify(|_, w| w.udrie1().bit(state)),
            }
        }
    }

    // TODO: ATmega128A USART0 is also different from other atmegas
    // Mainly needed because ubrr1 is split in ubrr1h and ubrr1l
    // For USART0 they are not even close to eachother in memory
    impl
        avr_hal_generic::usart::UsartOps<
            hal::Hal,
            hal::port::Pin<hal::port::mode::Input, hal::port::PE0>,
            hal::port::Pin<hal::port::mode::Output, hal::port::PE1>,
        > for hal::pac::USART0
    {
        fn raw_init<CLOCK>(&mut self, baudrate: Baudrate<CLOCK>) {
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

        fn raw_interrupt(&mut self, event: Event, state: bool) {
            match event {
                Event::RxComplete => self.ucsr0b.modify(|_, w| w.rxcie0().bit(state)),
                Event::TxComplete => self.ucsr0b.modify(|_, w| w.txcie0().bit(state)),
                Event::DataRegisterEmpty => self.ucsr0b.modify(|_, w| w.udrie0().bit(state)),
            }
        }
    }
}

impl_mod_wdt! {
    use crate::atmega128a as hal;
    impl_wdt_peripheral_ms2000! {
        mcusr: hal::pac::cpu::MCUCSR,
        wdtcsr_name: wdtcr,
    }
}

