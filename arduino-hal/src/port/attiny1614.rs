pub use avrmodern_hal::port::{mode, Pin, PinMode, PinOps};

avr_hal_generic::renamed_pins! {
    pub struct Pins {
        pub a0: avrmodern_hal::port::PA0 = pa0,
        pub a1: avrmodern_hal::port::PA1 = pa1,
        pub a2: avrmodern_hal::port::PA2 = pa2,
        pub a3: avrmodern_hal::port::PA3 = pa3,
        pub a4: avrmodern_hal::port::PA4 = pa4,
        pub a5: avrmodern_hal::port::PA5 = pa5,
        pub a6: avrmodern_hal::port::PA6 = pa6,
        pub a7: avrmodern_hal::port::PA7 = pa7,

        pub b0: avrmodern_hal::port::PB0 = pb0,
        pub b1: avrmodern_hal::port::PB1 = pb1,
        pub b2: avrmodern_hal::port::PB2 = pb2,
        pub b3: avrmodern_hal::port::PB3 = pb3,
    }

    impl Pins {
        type Pin = Pin;
        type McuPins = avrmodern_hal::Pins;
    }
}
