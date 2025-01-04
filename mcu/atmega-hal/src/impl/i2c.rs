macro_rules! impl_mod_i2c {
    ($($mod:item)*) => {
        pub mod i2c {
            //! I2C
            //!
            //! # Example
            //!
            //! Complete example source code can be found in the repository:
            //! [`atmega2560-i2cdetect.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-i2cdetect.rs)
            //!
            //! ```
            //! let dp = atmega_hal::Peripherals::take().unwrap();
            //! let pins = atmega_hal::pins!(dp);
            //!
            //! let mut i2c = I2c::new(
            //!     dp.TWI,
            //!     pins.pd1.into_pull_up_input(),
            //!     pins.pd0.into_pull_up_input(),
            //!     50_000,
            //! );
            //!
            //! i2c.i2cdetect(&mut serial, atmega_hal::i2c::Direction::Read).unwrap();
            //! ```

            pub use avr_hal_generic::i2c::*;
            use crate::r#impl::{impl_i2c_peripheral};

            $($mod)*
        }

        pub use i2c::I2c;
    }
}
pub(crate) use impl_mod_i2c;

macro_rules! impl_i2c_peripheral {
    (
        i2c_type: $i2c_type:ident,
        peripheral: $($peripheral:ident)::+,
        sda: $($sda:ident)::+,
        scl: $($scl:ident)::+ $(,)?
    ) => {
        pub type $i2c_type<CLOCK> = avr_hal_generic::i2c::I2c<
            hal::Hal,
            $($peripheral)::+,
            hal::port::Pin<hal::port::mode::Input, $($sda)::+>,
            hal::port::Pin<hal::port::mode::Input, $($scl)::+>,
            CLOCK,
        >;
        avr_hal_generic::impl_i2c_twi! {
            hal: hal::Hal,
            peripheral: $($peripheral)::+,
            sda: $($sda)::+,
            scl: $($scl)::+,
        }
    }
}
pub(crate) use impl_i2c_peripheral;
