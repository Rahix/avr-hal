macro_rules! impl_mod_spi {
    (
        hal: crate::$hal:ident,
        interfaces: {
            $(
                $name:ident: {
                    peripheral: $peripheral:ident,
                    sclk: $sclk:ident,
                    mosi: $mosi:ident,
                    miso: $miso:ident,
                    cs: $cs:ident,
                    impl!: $($impl_macro:ident)::+,
                },
            )+
        },
    ) => {
        pub mod spi {
            //! SPI
            //!
            //! # Example
            //!
            //! For full source code, please refer to the ATmega SPI example:
            //! [`atmega2560-spi-feedback.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-spi-feedback.rs)
            //!
            //! ```
            //! let dp = attiny_hal::Peripherals::take().unwrap();
            //! let pins = attiny_hal::pins!(dp);
            //!
            //! let (mut spi, mut cs) = spi::Spi::new(
            //!     dp.SPI,
            //!     pins.pa4.into_output(),
            //!     pins.pa6.into_output(),
            //!     pins.pa5.into_pull_up_input(),
            //!     pins.pa3.into_output(),
            //!     spi::Settings::default(),
            //! );
            //!
            //! let data_out = b"Hello World!";
            //! let mut data_in = [0u8; 12];
            //!
            //! cs.set_low().unwrap();
            //! spi.transfer(&mut data_in, data_out).unwrap();
            //! cs.set_high().unwrap();
            //!
            //! ufmt::uwriteln!(&mut serial, "data: {:?}", data_in).unwrap();
            //! ```

            pub use avr_hal_generic::spi::*;
            use crate::$hal as hal;

            $(
                pub type $name = avr_hal_generic::spi::Spi<
                    crate::$hal::Hal,
                    crate::$hal::pac::$peripheral,
                    hal::port::$sclk,
                    hal::port::$mosi,
                    hal::port::$miso,
                    hal::port::$cs,
                >;
    
                $($impl_macro)::+! {
                    hal: crate::$hal::Hal,
                    peripheral: crate::$hal::pac::$peripheral,
                    sclk: hal::port::$sclk,
                    mosi: hal::port::$mosi,
                    miso: hal::port::$miso,
                    cs: hal::port::$cs,
                }
            )+
        }
        pub use spi::{$($name),+};
    }
}

pub(crate) use impl_mod_spi;
