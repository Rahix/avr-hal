pub use atmega_hal::port::{mode, Pin, PinMode, PinOps};

avr_hal_generic::renamed_pins! {
    /// Pins of the **Circuit Playground Classic**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    pub struct Pins {
        /// `#11`: `PB7`
        pub d11: atmega_hal::port::PB7 = pb7,
        /// `#13`: `PC7`, Builtin LED
        pub d13: atmega_hal::port::PC7 = pc7,
    }

    impl Pins {
        type Pin = Pin;
        type McuPins = atmega_hal::Pins;
    }
}
