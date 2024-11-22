#![allow(unused_macros)]

macro_rules! impl_mod_port {
    (use crate::$hal:ident as hal; $($mod:item)*) => {
        pub mod port {
            //! Port
            //!
            //! # Example
            //!
            //! Complete example source code can be found in the repository:
            //! [`atmega2560-blink.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-blink.rs)
            //!
            //! ```
            //! let dp = atmega_hal::Peripherals::take().unwrap();
            //! let pins = atmega_hal::pins!(dp);
            //!
            //! let mut led = pins.pb7.into_output();
            //!
            //! loop {
            //!     led.toggle();
            //!     delay_ms(1000);
            //! }
            //! ```
            pub use avr_hal_generic::port::{mode, PinMode, PinOps};

            #[allow(unused_imports)]
            use crate::r#impl::{impl_port_peripheral_b8_c7_d8,impl_port_peripheral_a8_b8_c8_d8,impl_port_peripheral_a8_b8_c8_d8_e8_f8_g6_h8_j8_k8_l8};

            use crate::$hal as hal;
            $($mod)*
        }

        pub use pac::Peripherals;
        pub use port::{Pins, pins};
    }
}
pub(crate) use impl_mod_port;

macro_rules! impl_port_peripheral_b8_c7_d8 {
    () => {
        avr_hal_generic::impl_port_traditional! {
            enum Ports {
                B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
                C: hal::pac::PORTC = [0, 1, 2, 3, 4, 5, 6],
                D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
            }
        }
    }
}
pub(crate) use impl_port_peripheral_b8_c7_d8;

macro_rules! impl_port_peripheral_a8_b8_c8_d8 {
    () => {
        avr_hal_generic::impl_port_traditional! {
            enum Ports {
                A: hal::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
                B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
                C: hal::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
                D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
            }
        }
    }
}
pub(crate) use impl_port_peripheral_a8_b8_c8_d8;

macro_rules! impl_port_peripheral_a8_b8_c8_d8_e8_f8_g6_h8_j8_k8_l8 {
    () => {
        avr_hal_generic::impl_port_traditional! {
            enum Ports {
                A: hal::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
                B: hal::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
                C: hal::pac::PORTC = [0, 1, 2, 3, 4, 5, 6, 7],
                D: hal::pac::PORTD = [0, 1, 2, 3, 4, 5, 6, 7],
                E: hal::pac::PORTE = [0, 1, 2, 3, 4, 5, 6, 7],
                F: hal::pac::PORTF = [0, 1, 2, 3, 4, 5, 6, 7],
                G: hal::pac::PORTG = [0, 1, 2, 3, 4, 5],
                H: hal::pac::PORTH = [0, 1, 2, 3, 4, 5, 6, 7],
                J: hal::pac::PORTJ = [0, 1, 2, 3, 4, 5, 6, 7],
                K: hal::pac::PORTK = [0, 1, 2, 3, 4, 5, 6, 7],
                L: hal::pac::PORTL = [0, 1, 2, 3, 4, 5, 6, 7],
                            }
        }
    }
}
pub(crate) use impl_port_peripheral_a8_b8_c8_d8_e8_f8_g6_h8_j8_k8_l8;
