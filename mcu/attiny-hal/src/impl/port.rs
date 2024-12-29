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
            //! For full source code, please refer to the ATmega port example:
            //! [`atmega2560-blink.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-blink.rs)
            //!
            //! ```no_run
            //! use attiny_hal::prelude::*;
            #![doc = concat!("use attiny_hal::", stringify!($hal), " as hal;")]
            //!
            //! type Clock = attiny_hal::clock::MHz8;
            //! let mut delay = attiny_hal::delay::Delay::<Clock>::new();
            //! 
            //! let dp = hal::Peripherals::take().unwrap();
            //! let pins = hal::pins!(dp);
            //!
            $(
                $(
                    #![doc = paste!{ concat!(
                        "let mut ", stringify!([< led_p $name:lower $pin >]), " = pins.", stringify!([< p $name:lower $pin >]), ".into_output();",
                    ) }]
                )+
            )+
            //!
            //! loop {
            $(
                $(
                    #![doc = paste!{ concat!(
                        "    ", stringify!([< led_p $name:lower $pin >]), ".toggle();",
                    ) }]
                )+
            )+
            //!     delay.delay_ms(1000u16);
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
        pub use {pac::Peripherals, port::Pins, port::pins};
    }
}

pub(crate) use impl_mod_port;
