#![allow(unused_macros)]

macro_rules! impl_mod_port {
    (
        hal: crate::$hal:ident,
        ports: {
            $($name:ident: [$($pin:literal),+],)+
        },
        impl!: $($impl_macro:ident)::+ $({
            $($arg_name:ident: $arg_value:expr,)*
        })?,
    ) => {
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
            use avr_hal_generic::paste::paste;
            use crate::$hal as hal;

            paste! {
                pub use avr_hal_generic::port::{mode, PinMode, PinOps};
                $($impl_macro)::+! {
                    enum Ports {
                        $($name: hal::pac::[< PORT $name >] = [$($pin),+],)+
                    }
                }

                #[macro_export]
                macro_rules! [< $hal _pins >] {
                    ($p:expr) => {
                        $crate::$hal::port::Pins::new($($p.[< PORT $name >],)+)
                    }
                }

                pub use [< $hal _pins >] as pins;
            }
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
                B: [0, 1, 2, 3, 4, 5, 6, 7],
                C: [0, 1, 2, 3, 4, 5, 6],
                D: [0, 1, 2, 3, 4, 5, 6, 7],
            }
        }
    };
}
pub(crate) use impl_port_peripheral_b8_c7_d8;

macro_rules! impl_port_peripheral_a8_b8_c8_d8 {
    () => {
        avr_hal_generic::impl_port_traditional! {
            enum Ports {
                A: [0, 1, 2, 3, 4, 5, 6, 7],
                B: [0, 1, 2, 3, 4, 5, 6, 7],
                C: [0, 1, 2, 3, 4, 5, 6, 7],
                D: [0, 1, 2, 3, 4, 5, 6, 7],
            }
        }
    };
}
pub(crate) use impl_port_peripheral_a8_b8_c8_d8;

macro_rules! impl_port_peripheral_a8_b8_c8_d8_e8_f8_g6_h8_j8_k8_l8 {
    () => {
        avr_hal_generic::impl_port_traditional! {
            enum Ports {
                A: [0, 1, 2, 3, 4, 5, 6, 7],
                B: [0, 1, 2, 3, 4, 5, 6, 7],
                C: [0, 1, 2, 3, 4, 5, 6, 7],
                D: [0, 1, 2, 3, 4, 5, 6, 7],
                E: [0, 1, 2, 3, 4, 5, 6, 7],
                F: [0, 1, 2, 3, 4, 5, 6, 7],
                G: [0, 1, 2, 3, 4, 5],
                H: [0, 1, 2, 3, 4, 5, 6, 7],
                J: [0, 1, 2, 3, 4, 5, 6, 7],
                K: [0, 1, 2, 3, 4, 5, 6, 7],
                L: [0, 1, 2, 3, 4, 5, 6, 7],
                            }
        }
    };
}
pub(crate) use impl_port_peripheral_a8_b8_c8_d8_e8_f8_g6_h8_j8_k8_l8;
