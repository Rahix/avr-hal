macro_rules! impl_mod_spi {
    (
        hal: crate::$hal:ident,
        sclk: $sclk:ident,
        mosi: $mosi:ident,
        miso: $miso:ident,
        cs: $cs:ident $(,)?
    ) => {
        pub mod spi {
            //! SPI
            //!
            //! # Example
            //!
            //! For full source code, please refer to the ATmega SPI example:
            //! [`atmega2560-spi-feedback.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-spi-feedback.rs)
            //!
            //! ```no_run
            #![doc = concat!("use attiny_hal::", stringify!($hal), " as hal;")]
            //!
            //! use embedded_hal::digital::OutputPin;
            //! use embedded_hal::spi::SpiBus;
            //! 
            //! let dp = hal::Peripherals::take().unwrap();
            //! let pins = hal::pins!(dp);
            //!
            #![doc = paste!{ concat!(
                "let (mut spi, mut cs) = hal::spi::Spi::new(\n",
                "    dp.SPI,\n",
                "    pins.", stringify!([< $sclk:lower >]), ".into_output(),\n",
                "    pins.", stringify!([< $mosi:lower >]), ".into_output(),\n",
                "    pins.", stringify!([< $miso:lower >]), ".into_pull_up_input(),\n",
                "    pins.", stringify!([< $cs:lower >]), ".into_output(),\n",
                "    hal::spi::Settings::default(),\n",
                ");\n",
            ) }]
            //!
            //! let data_out = b"Hello World!";
            //! let mut data_in = [0u8; 12];
            //!
            //! cs.set_low().unwrap();
            //! spi.transfer(&mut data_in, data_out).unwrap();
            //! cs.set_high().unwrap();
            //! ```

            pub use avr_hal_generic::spi::*;
            use avr_hal_generic::paste::paste;
            use crate::$hal as hal;

            pub type Spi = avr_hal_generic::spi::Spi<
                crate::$hal::Hal,
                crate::$hal::pac::SPI,
                hal::port::$sclk,
                hal::port::$mosi,
                hal::port::$miso,
                hal::port::$cs,
            >;

            avr_hal_generic::impl_spi! {
                hal: crate::$hal::Hal,
                peripheral: crate::$hal::pac::SPI,
                sclk: hal::port::$sclk,
                mosi: hal::port::$mosi,
                miso: hal::port::$miso,
                cs: hal::port::$cs,
            }
        }
        pub use spi::Spi;
    }
}

pub(crate) use impl_mod_spi;