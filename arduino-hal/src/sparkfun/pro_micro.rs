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
        board: crate::sparkfun::pro_micro
    }
}

pub mod port {
    use crate::sparkfun::pro_micro as board;
    pub use board::hal::port::{mode, Pin, PinMode, PinOps};

    avr_hal_generic::renamed_pins! {
        /// Pins of the **SparkFun ProMicro**.
        ///
        /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
        pub struct Pins {
            /// `RX`
            ///
            /// `RX` (UART)
            pub rx: board::hal::port::PD2 = pd2,
            /// `TX`
            ///
            /// `TX` (UART)
            pub tx: board::hal::port::PD3 = pd3,
            /// `D2` / `SDA`
            ///
            /// `SDA`: i2c/twi data
            pub d2: board::hal::port::PD1 = pd1,
            /// `D3` / `SCL`
            ///
            /// `SCL`: i2c/twi clock
            pub d3: board::hal::port::PD0 = pd0,
            /// `D4`
            pub d4: board::hal::port::PD4 = pd4,
            /// `D5`
            pub d5: board::hal::port::PC6 = pc6,
            /// `D6`
            pub d6: board::hal::port::PD7 = pd7,
            /// `D7`
            pub d7: board::hal::port::PE6 = pe6,
            /// `D8`
            pub d8: board::hal::port::PB4 = pb4,
            /// `D9`
            pub d9: board::hal::port::PB5 = pb5,
            /// `D10`
            pub d10: board::hal::port::PB6 = pb6,
            /// `LED_RX`
            ///
            /// Led for indicating inbound data (yellow).  Also the CS pin for SPI.
            pub led_rx: board::hal::port::PB0 = pb0,
            /// `LED_TX`
            ///
            /// Led for indicating outbound data (green).
            pub led_tx: board::hal::port::PD5 = pd5,
            /// `D15`, `SCK`
            ///
            /// ICSP SCLK pin
            pub d15: board::hal::port::PB1 = pb1,
            /// `D14`, `MISO`
            ///
            /// ICSP MISO pin
            pub d16: board::hal::port::PB3 = pb3,
            /// `D16`, `MOSI`
            ///
            /// ICSP MOSI pin
            pub d14: board::hal::port::PB2 = pb2,
            /// `A0`
            ///
            /// `ADC7` channel
            pub a0: board::hal::port::PF7 = pf7,
            /// `A1`
            ///
            /// `ADC6` channel
            pub a1: board::hal::port::PF6 = pf6,
            /// `A2`
            ///
            /// `ADC5` channel
            pub a2: board::hal::port::PF5 = pf5,
            /// `A3`
            ///
            /// `ADC4` channel
            pub a3: board::hal::port::PF4 = pf4,
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
    macro_rules! sparkfun_pro_micro_pins {
        ($p:expr) => {
            $crate::sparkfun::pro_micro::Pins::with_mcu_pins($crate::sparkfun::pro_micro::hal::pins!($p))
        };
    }

    pub use sparkfun_pro_micro_pins as pins;
}

pub mod adc {
    use crate::r#impl::impl_adc_atmega;
    impl_adc_atmega! {
        board: crate::sparkfun::pro_micro
    }
}

pub mod i2c {
    use crate::r#impl::impl_i2c_atmega;
    impl_i2c_atmega! {
        board: crate::sparkfun::pro_micro
    }
}

pub mod spi {
    use crate::r#impl::impl_spi_atmega;
    impl_spi_atmega! {
        board: crate::sparkfun::pro_micro
    }
}

pub mod usart {
    use crate::r#impl::impl_usart_atmega;
    impl_usart_atmega! {
        board: crate::sparkfun::pro_micro
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
    macro_rules! sparkfun_pro_micro_default_serial {
        ($p:expr, $pins:expr, $baud:expr) => {
            $crate::sparkfun::pro_micro::Usart::new(
                $p.USART1,
                $pins.rx,
                $pins.tx.into_output(),
                $crate::sparkfun::pro_micro::hal::usart::BaudrateExt::into_baudrate($baud),
            )
        };
    }
    pub use sparkfun_pro_micro_default_serial as default_serial;
}

pub mod eeprom {
    use crate::r#impl::impl_eeprom;
    impl_eeprom! {
        board: crate::sparkfun::pro_micro
    }
}

pub mod simple_pwm {
    use crate::r#impl::impl_simple_pwm;
    impl_simple_pwm! {
        board: crate::sparkfun::pro_micro
    }
}

pub mod prelude {
    use crate::sparkfun::pro_micro as board;
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
    usart::{Usart, default_serial},
};
