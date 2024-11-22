#![allow(unused_macros)]

macro_rules! impl_mod_spi {
    ($($mod:item)*) => {
        pub mod spi {
            //! SPI
            //!
            //! # Example
            //!
            //! Complete example source code can be found in the repository
            //! [`atmega2560-spi-feedback.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-spi-feedback.rs)
            //!
            //! ```
            //! let dp = atmega_hal::Peripherals::take().unwrap();
            //! let pins = atmega_hal::pins!(dp);
            //!
            //! let (mut spi, mut cs) = spi::Spi::new(
            //!     dp.SPI,
            //!     pins.pb1.into_output(),
            //!     pins.pb2.into_output(),
            //!     pins.pb3.into_pull_up_input(),
            //!     pins.pb0.into_output(),
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
            use crate::r#impl::impl_spi_peripheral;

            $($mod)*
        }
        pub use spi::Spi;
    }
}
pub(crate) use impl_mod_spi;

macro_rules! impl_spi_peripheral {
    (
        spi: $spi:ident,
        peripheral: $($peripheral:ident)::+,
        sclk: $($sclk:ident)::+,
        mosi: $($mosi:ident)::+,
        miso: $($miso:ident)::+,
        cs: $($cs:ident)::+ $(,)?
    ) => {
        pub type $spi = avr_hal_generic::spi::Spi<
            hal::Hal,
            $($peripheral)::+,
            $($sclk)::+,
            $($mosi)::+,
            $($miso)::+,
            $($cs)::+,
        >;

        avr_hal_generic::impl_spi! {
            hal: hal::Hal,
            peripheral: $($peripheral)::+,
            sclk: $($sclk)::+,
            mosi: $($mosi)::+,
            miso: $($miso)::+,
            cs: $($cs)::+,
        }

    }
}
pub(crate) use impl_spi_peripheral;
