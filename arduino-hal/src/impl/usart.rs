#[cfg(feature = "_mcu-atmega")]
macro_rules! impl_usart_atmega {
    (
        board: $($board:ident)::+ $(,)?
    ) => {
        pub use $($board)::+::hal::usart::{Baudrate, UsartOps};

        pub type Usart<USART, RX, TX> = $($board)::+::hal::usart::Usart<USART, RX, TX, $($board)::+::clock::DefaultClock>;
        pub type UsartWriter<USART, RX, TX> =
            $($board)::+::hal::usart::UsartWriter<USART, RX, TX, $($board)::+::clock::DefaultClock>;
        pub type UsartReader<USART, RX, TX> =
            $($board)::+::hal::usart::UsartReader<USART, RX, TX, $($board)::+::clock::DefaultClock>;
    }

}

#[cfg(feature = "_mcu-atmega")]
pub(crate) use impl_usart_atmega;

