#[cfg(feature = "_mcu-atmega")]
macro_rules! impl_spi_atmega {
    (
        board: $($board:ident)::+ $(,)?
    ) => {
        pub use $($board)::+::hal::spi::*;

        pub type Spi = $($board)::+::hal::spi::Spi;

}

}

#[cfg(feature = "_mcu-atmega")]
pub(crate) use impl_spi_atmega;
