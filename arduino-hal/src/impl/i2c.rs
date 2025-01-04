#[cfg(feature = "_mcu-atmega")]
macro_rules! impl_i2c_atmega {
    (
        board: $($board:ident)::+ $(,)?
    ) => {
        pub use $($board)::+::hal::i2c::*;

        pub type I2c = $($board)::+::hal::i2c::I2c<$($board)::+::clock::DefaultClock>;

}

}

#[cfg(feature = "_mcu-atmega")]
pub(crate) use impl_i2c_atmega;
