pub use atmega_hal::atmega32u4 as hal;

pub use hal::{pac, Peripherals};

pub mod clock {

    //! MCU core clock support.
    //!
    //! This module contains common definitions to abtract over the MCU core clock speed.  `avr-hal`
    //! does not support changing the clock-speed at runtime.
    //!
    //! Most items in this module are re-exported from [`avr_hal_generic::clock`].
    pub use avr_hal_generic::clock::*;

    pub type DefaultClock = avr_hal_generic::clock::MHz16;
}

pub mod delay {
    use crate::r#impl::impl_delay;
    impl_delay! {
        board: crate::arduino::leonardo
    }
}

pub mod port {
    use crate::arduino::leonardo as board;

    pub use board::hal::port::{mode, Pin, PinMode, PinOps};

    avr_hal_generic::renamed_pins! {
        /// Pins of the **Arduino Leonardo**.
        ///
        /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
        pub struct Pins {
            /// `D0` / `RX`
            ///
            /// * `RX` (UART)
            /// * `INT2`: External Interrupt
            pub d0: board::hal::port::PD2 = pd2,
            /// `D1` / `TX`
            ///
            /// * `TX` (UART)
            /// * `INT3`: External Interrupt
            pub d1: board::hal::port::PD3 = pd3,
            /// `D2` / `SDA`
            ///
            /// * `SDA`: i2c/twi data
            /// * `INT1`: External Interrupt
            pub d2: board::hal::port::PD1 = pd1,
            /// `D3` / `SCL`
            ///
            /// * **PWM**: [atmega32u4_hal::timer::Timer0Pwm]
            /// * `SCL`: i2c/twi clock
            /// * `INT0`: External Interrupt
            /// * `OC0B`: Output Compare Channel `B` for Timer/Counter0
            pub d3: board::hal::port::PD0 = pd0,
            /// `D4`
            pub d4: board::hal::port::PD4 = pd4,
            /// `D5`
            ///
            /// * **PWM**: [atmega32u4_hal::timer::Timer3Pwm]
            /// * `OC3A`: Output Compare Channel `A` for Timer/Counter3
            /// * `#OC4A`: Inverted Output Compare Channel `A` for Timer/Counter4 (Not implemented)
            pub d5: board::hal::port::PC6 = pc6,
            /// `D6`
            ///
            /// * **PWM**: [atmega32u4_hal::timer::Timer4Pwm]
            /// * `OC4D`: Output Compare Channel `D` for Timer/Counter4
            pub d6: board::hal::port::PD7 = pd7,
            /// `D7`
            ///
            /// * `INT6`: External Interrupt
            pub d7: board::hal::port::PE6 = pe6,
            /// `D8`
            pub d8: board::hal::port::PB4 = pb4,
            /// `D9`
            ///
            /// * **PWM**: [atmega32u4_hal::timer::Timer1Pwm]
            /// * `OC1A`: Output Compare Channel `A` for Timer/Counter1
            /// * `#OC4B`: Inverted Output Compare Channel `B` for Timer/Counter4 (Not implemented)
            pub d9: board::hal::port::PB5 = pb5,
            /// `D10`
            ///
            /// * **PWM**: [atmega32u4_hal::timer::Timer1Pwm]
            /// * `OC1B`: Output Compare Channel `B` for Timer/Counter1
            /// * `OC4B`: Output Compare Channel `B` for Timer/Counter4 (Not implemented)
            pub d10: board::hal::port::PB6 = pb6,
            /// `D11`
            ///
            /// * **PWM**: [atmega32u4_hal::timer::Timer0Pwm]
            /// * `OC0A`: Output Compare Channel `B` for Timer/Counter0
            /// * `OC1C`: Output Compare Channel `C` for Timer/Counter1
            pub d11: board::hal::port::PB7 = pb7,
            /// `D12`
            ///
            /// * `#OC4D`: Inverted Output Compare Channel `D` for Timer/Counter4 (Not implemented)
            pub d12: board::hal::port::PD6 = pd6,
            /// `D13` / `LED_BUILTIN`
            ///
            /// * Onboard LED
            /// * **PWM**: [atmega32u4_hal::timer::Timer4Pwm]
            /// * `OC4A`: Output Compare Channel `A` for Timer/Counter4
            pub d13: board::hal::port::PC7 = pc7,
            /// `RX`
            ///
            /// Led for indicating inbound data.  Also the CS pin.
            pub led_rx: board::hal::port::PB0 = pb0,
            /// `TX`
            ///
            /// Led for indicating outbound data
            pub led_tx: board::hal::port::PD5 = pd5,
            /// `SCLK`
            ///
            /// ICSP SCLK pin
            pub sck: board::hal::port::PB1 = pb1,
            /// `MOSI`
            ///
            /// ICSP MOSI pin
            pub mosi: board::hal::port::PB2 = pb2,
            /// `MISO`
            ///
            /// ICSP MISO pin
            pub miso: board::hal::port::PB3 = pb3,
            /// `A0`
            ///
            /// * `ADC7` channel
            pub a0: board::hal::port::PF7 = pf7,
            /// `A1`
            ///
            /// * `ADC6` channel
            pub a1: board::hal::port::PF6 = pf6,
            /// `A2`
            ///
            /// * `ADC5` channel
            pub a2: board::hal::port::PF5 = pf5,
            /// `A3`
            ///
            /// * `ADC4` channel
            pub a3: board::hal::port::PF4 = pf4,
            /// `A4`
            ///
            /// * `ADC1` channel
            pub a4: board::hal::port::PF1 = pf1,
            /// `A5`
            ///
            /// * `ADC0` channel
            pub a5: board::hal::port::PF0 = pf0,
        }

        impl Pins {
            type Pin = Pin;
            type McuPins = board::hal::Pins;
        }
    }

    /// Convenience macro to instantiate the [`Pins`] struct for this board.
    ///
    /// # Example
    /// ```no_run
    /// let dp = arduino_hal::Peripherals::take().unwrap();
    /// let pins = arduino_hal::pins!(dp);
    /// ```
    #[macro_export]
    macro_rules! arduino_leonardo_pins {
        ($p:expr) => {
            $crate::arduino::leonardo::Pins::with_mcu_pins($crate::arduino::leonardo::hal::pins!($p))
        };
    }

    pub use arduino_leonardo_pins as pins;
}

pub mod adc {
    use crate::r#impl::impl_adc_atmega;
    impl_adc_atmega! {
        board:     crate::arduino::leonardo

    }
}

pub mod i2c {
    use crate::r#impl::impl_i2c_atmega;
    impl_i2c_atmega! {
        board:     crate::arduino::leonardo

    }
}

pub mod spi {
    use crate::r#impl::impl_spi_atmega;
    impl_spi_atmega! {
        board:     crate::arduino::leonardo

    }
}

pub mod usart {
    use crate::r#impl::impl_usart_atmega;
    impl_usart_atmega! {
        board:     crate::arduino::leonardo

    }

    /// Convenience macro to instantiate the [`Usart`] driver for this board.
    ///
    /// # Example
    /// ```no_run
    /// let dp = arduino_hal::Peripherals::take().unwrap();
    /// let pins = arduino_hal::pins!(dp);
    /// let serial = arduino_hal::default_serial!(dp, pins, 57600);
    /// ```
    #[macro_export]
    macro_rules! arduino_leonardo_default_serial {
        ($p:expr, $pins:expr, $baud:expr) => {
            $crate::arduino::leonardo::Usart::new(
                $p.USART1,
                $pins.d0,
                $pins.d1.into_output(),
                $crate::arduino::leonardo::hal::usart::BaudrateExt::into_baudrate($baud),
            )
        };
    }
    pub use arduino_leonardo_default_serial as default_serial;
}

pub mod eeprom {
    use crate::r#impl::impl_eeprom;
    impl_eeprom! {
        board:     crate::arduino::leonardo

    }
}

pub mod simple_pwm {
    use crate::r#impl::impl_simple_pwm;
    impl_simple_pwm! {
        board:     crate::arduino::leonardo

    }
}

pub mod prelude {
    use crate::arduino::leonardo as board;
    pub use board::hal::usart::BaudrateExt as _;
    pub use atmega_hal::prelude::*;
}

pub use {
    adc::Adc,
    clock::DefaultClock,
    delay::{delay_ms, delay_us, Delay},
    eeprom::Eeprom,
    i2c::I2c,
    port::{Pins, pins},
    spi::Spi,
    usart::{Usart, default_serial}
};
