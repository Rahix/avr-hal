pub use attiny_hal::attiny85 as hal;

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
        board: crate::adafruit::trinket
    }
}

pub mod port {
    use crate::adafruit::trinket as board;

    pub use board::hal::port::{mode, Pin, PinMode, PinOps};

    avr_hal_generic::renamed_pins! {
        pub struct Pins {
            /// `#0`: `PB0`, `DI`(SPI), `SDA`(I2C)
            pub d0: board::hal::port::PB0 = pb0,
            /// `#1`: `PB1`, `DO`(SPI), Builtin LED
            pub d1: board::hal::port::PB1 = pb1,
            /// `#2`: `PB2`, `SCK`(SPI), `SCL`(I2C)
            pub d2: board::hal::port::PB2 = pb2,
            /// `#3`: `PB3`
            pub d3: board::hal::port::PB3 = pb3,
            /// `#4`: `PB4`
            pub d4: board::hal::port::PB4 = pb4,
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
    macro_rules! adafruit_trinket_pins {
        ($p:expr) => {
            $crate::adafruit::trinket::Pins::with_mcu_pins($crate::adafruit::trinket::hal::pins!($p))
        };
    }

    pub use adafruit_trinket_pins as pins;
}

pub mod eeprom {
    use crate::r#impl::impl_eeprom;
    impl_eeprom! {
        board:     crate::adafruit::trinket
    }
}

pub mod simple_pwm {
    use crate::r#impl::impl_simple_pwm;
    impl_simple_pwm! {
        board:     crate::adafruit::trinket
    }
}

pub use {
    clock::DefaultClock,
    delay::{delay_ms, delay_us, Delay},
    eeprom::Eeprom,
    port::{Pins, pins},
};
