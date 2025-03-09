macro_rules! impl_mod_i2c {
    (
        hal: crate::$hal:ident,
        interfaces: {$(
            $interface:ident: {
                peripheral: $peripheral:ident,
                sda: $sda:ident,
                scl: $scl:ident,
            },
        )+},
    ) => {
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
            use crate::$hal as hal;

            $(
                pub type $interface<CLOCK> = avr_hal_generic::i2c::I2c<
                    hal::Hal,
                    hal::pac::$peripheral,
                    hal::port::Pin<hal::port::mode::Input, hal::port::$sda>,
                    hal::port::Pin<hal::port::mode::Input, hal::port::$scl>,
                    CLOCK,
                >;
                avr_hal_generic::impl_i2c_twi! {
                    hal: hal::Hal,
                    peripheral: hal::pac::$peripheral,
                    sda: hal::port::$sda,
                    scl: hal::port::$scl,
                }
            )+
        }

        pub use i2c::I2c;
    }
}
pub(crate) use impl_mod_i2c;

