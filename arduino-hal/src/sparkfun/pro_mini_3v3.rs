pub use atmega_hal::atmega328p as hal;

pub use hal::{pac, Peripherals};

pub mod clock {

    //! MCU core clock support.
    //!
    //! This module contains common definitions to abtract over the MCU core clock speed.  `avr-hal`
    //! does not support changing the clock-speed at runtime.
    //!
    //! Most items in this module are re-exported from [`avr_hal_generic::clock`].
    pub use avr_hal_generic::clock::*;

    pub type DefaultClock = avr_hal_generic::clock::MHz8;
}

pub mod delay {
    use crate::r#impl::impl_delay;
    impl_delay! {
        board: crate::sparkfun::pro_mini_3v3
    }
}

pub mod port {
    use crate::arduino::r#impl::impl_port_uno;
    impl_port_uno! {
        board: crate::sparkfun::pro_mini_3v3
    }

    /// Convenience macro to instantiate the [`Pins`] struct for this board.
    ///
    /// # Example
    /// ```no_run
    /// let dp = arduino_hal::Peripherals::take().unwrap();
    /// let pins = arduino_hal::pins!(dp);
    /// ```
    #[macro_export]
    macro_rules! sparkfun_pro_mini_3v3_pins {
        ($p:expr) => {
            $crate::sparkfun::pro_mini_3v3::Pins::with_mcu_pins($crate::sparkfun::pro_mini_3v3::hal::pins!($p))
        };
    }

    pub use sparkfun_pro_mini_3v3_pins as pins;
}

pub mod adc {
    use crate::r#impl::impl_adc_atmega;
    impl_adc_atmega! {
        board: crate::sparkfun::pro_mini_3v3
    }
}

pub mod i2c {
    use crate::r#impl::impl_i2c_atmega;
    impl_i2c_atmega! {
        board: crate::sparkfun::pro_mini_3v3
    }
}

pub mod spi {
    use crate::r#impl::impl_spi_atmega;
    impl_spi_atmega! {
        board: crate::sparkfun::pro_mini_3v3
    }
}

pub mod usart {
    use crate::r#impl::impl_usart_atmega;
    impl_usart_atmega! {
        board: crate::sparkfun::pro_mini_3v3
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
    macro_rules! sparkfun_pro_mini_3v3_default_serial {
        ($p:expr, $pins:expr, $baud:expr) => {
            $crate::Usart::new(
                $p.USART0,
                $pins.d0,
                $pins.d1.into_output(),
                $crate::hal::usart::BaudrateExt::into_baudrate($baud),
            )
        };
    }
    pub use sparkfun_pro_mini_3v3_default_serial as default_serial;
}

pub mod eeprom {
    use crate::r#impl::impl_eeprom;
    impl_eeprom! {
        board: crate::sparkfun::pro_mini_3v3
    }
}

pub mod simple_pwm {
    use crate::r#impl::impl_simple_pwm;
    impl_simple_pwm! {
        board: crate::sparkfun::pro_mini_3v3
    }
}

pub mod prelude {
    use crate::sparkfun::pro_mini_3v3 as board;
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
    usart::{Usart,default_serial},
};
