macro_rules! impl_eeprom {
    (
        board: $($board:ident)::+  $(,)?
    ) => {
        pub use $($board)::+::hal::eeprom::{Eeprom, EepromOps, OutOfBoundsError};

}

}

pub(crate) use impl_eeprom;
