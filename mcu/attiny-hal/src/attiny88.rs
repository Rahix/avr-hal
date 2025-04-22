pub mod adc {
    pub use crate::periphals::adc::*;

    use crate::port;

    avr_hal_generic::impl_adc! {
        hal: crate::Attiny,
        peripheral: crate::pac::ADC,
        settings: AdcSettings,
        apply_settings: |peripheral, settings| {
            apply_clock(peripheral, settings);
            peripheral.admux.write(|w| match settings.ref_voltage {
                ReferenceVoltage::AVcc => w.refs0().avcc(),
                ReferenceVoltage::Internal1_1 => w.refs0().internal(),
            });
        },
        channel_id: crate::pac::adc::admux::MUX_A,
        set_channel: |peripheral, id| {
            peripheral.admux.modify(|_, w| w.mux().variant(id));
        },
        pins: {
            port::PC0: (crate::pac::adc::admux::MUX_A::ADC0, didr0::adc0d),
            port::PC1: (crate::pac::adc::admux::MUX_A::ADC1, didr0::adc1d),
            port::PC2: (crate::pac::adc::admux::MUX_A::ADC2, didr0::adc2d),
            port::PC3: (crate::pac::adc::admux::MUX_A::ADC3, didr0::adc3d),
            port::PC4: (crate::pac::adc::admux::MUX_A::ADC4, didr0::adc4d),
            port::PC5: (crate::pac::adc::admux::MUX_A::ADC5, didr0::adc5d),
            port::PA0: (crate::pac::adc::admux::MUX_A::ADC6, didr0::adc6d),
            port::PA1: (crate::pac::adc::admux::MUX_A::ADC7, didr0::adc7d),
        },
        channels: {
            channel::Vbg: crate::pac::adc::admux::MUX_A::ADC_VBG,
            channel::Gnd: crate::pac::adc::admux::MUX_A::ADC_GND,
            channel::Temperature: crate::pac::adc::admux::MUX_A::TEMPSENS,
        },
    }
}

pub mod eeprom {
    pub use crate::periphals::eeprom::*;

    avr_hal_generic::impl_eeprom_attiny! {
        hal: crate::Attiny,
        peripheral: crate::pac::EEPROM,
        capacity: 64,
        addr_width: u8,
        set_address: |peripheral, address| {
            peripheral.eearl.write(|w| w.bits(address));
        },
    }
}

pub mod port {
    pub use crate::periphals::port::*;

    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: crate::pac::PORTA = [0, 1, 2, 3],
            B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            C: crate::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
            D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
        }
    }
}

pub mod simple_pwm {
    pub use crate::periphals::simple_pwm::*;

    use crate::port::*;

    avr_hal_generic::impl_simple_pwm! {
        /// Use `TC1` for PWM (pins `PB1`, 'PB2')
        ///
        /// # Example
        /// ```
        /// let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
        ///
        /// let mut d9 = pins.d9.into_output().into_pwm(&mut timer1);
        /// let mut d10 = pins.d10.into_output().into_pwm(&mut timer1);
        ///
        /// d9.set_duty(128);
        /// d9.enable();
        /// ```
        pub struct Timer1Pwm {
            timer: crate::pac::TC1,
            init: |tim, prescaler| {
                tim.tccr1a.modify(|_, w| w.wgm1().bits(0b01));
                tim.tccr1b.modify(|_, w| w.wgm1().bits(0b01));

                tim.tccr1b.modify(|_r, w| match prescaler {
                    Prescaler::Direct => w.cs1().direct(),
                    Prescaler::Prescale8 => w.cs1().prescale_8(),
                    Prescaler::Prescale64 => w.cs1().prescale_64(),
                    Prescaler::Prescale256 => w.cs1().prescale_256(),
                    Prescaler::Prescale1024 => w.cs1().prescale_1024(),
                });
            },
            pins: {
                PB1: {
                    ocr: ocr1a,
                    into_pwm: |tim| if enable {
                        tim.tccr1a.modify(|_, w| w.com1a().bits(0b10));
                    } else {
                        tim.tccr1a.modify(|_, w| w.com1a().disconnected());
                    },
                },

                PB2: {
                    ocr: ocr1b,
                    into_pwm: |tim| if enable {
                        tim.tccr1a.modify(|_, w| w.com1b().bits(0b10));
                    } else {
                        tim.tccr1a.modify(|_, w| w.com1b().disconnected());
                    },
                },
            },
        }
    }
}

pub mod spi {
    pub use crate::periphals::spi::*;

    use crate::port;

    pub type Spi = avr_hal_generic::spi::Spi<
        crate::Attiny,
        crate::pac::SPI,
        port::PB5,
        port::PB3,
        port::PB4,
        port::PB2,
    >;

    avr_hal_generic::impl_spi! {
        hal: crate::Attiny,
        peripheral: crate::pac::SPI,
        sclk: port::PB5,
        mosi: port::PB3,
        miso: port::PB4,
        cs: port::PB2,
    }
}

pub mod wdt {
    pub use crate::periphals::wdt::*;

    avr_hal_generic::impl_wdt! {
        hal: crate::Attiny,
        peripheral: crate::pac::WDT,
        mcusr: crate::pac::cpu::MCUSR,
        wdtcsr_name: wdtcsr,
        timeout: |to, w| match to {
            Timeout::Ms16 => w.wdpl().cycles_2k_512k(),
            Timeout::Ms32 => w.wdpl().cycles_4k_1024k(),
            Timeout::Ms64 => w.wdpl().cycles_8k(),
            Timeout::Ms125 => w.wdpl().cycles_16k(),
            Timeout::Ms250 => w.wdpl().cycles_32k(),
            Timeout::Ms500 => w.wdpl().cycles_64k(),
            Timeout::Ms1000 => w.wdpl().cycles_128k(),
            Timeout::Ms2000 => w.wdpl().cycles_256k(),
            Timeout::Ms4000 => w.wdph().set_bit().wdpl().cycles_2k_512k(),
            Timeout::Ms8000 => w.wdph().set_bit().wdpl().cycles_4k_1024k(),
        },
    }
}
