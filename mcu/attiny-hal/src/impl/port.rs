macro_rules! impl_mod_port {
    (use crate::$hal:ident as hal; $($mod:item)*) => {
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
            //! let mut led = pins.pb2.into_output();
            //!
            //! loop {
            //!     led.toggle();
            //!     delay.delay_ms(1000u16);
            //! }
            //! ```

            use crate::$hal as hal;
            $($mod)*
        }

        pub use {pac::Peripherals, port::Pins, port::pins};
    }
}

pub(crate) use impl_mod_port;
