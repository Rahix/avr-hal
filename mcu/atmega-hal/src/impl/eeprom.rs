#![allow(unused_macros)]

macro_rules! impl_mod_eeprom {
    (
        hal: crate::$hal:ident,
        capacity: $capacity:expr,
        addr_width: $addr_width:ty,
        addr_reg: $addr_reg:ident,
        impl!: $($impl_macro:ident)::+ $(,)?
    ) => {
        pub mod eeprom {
            //! EEPROM
            //!
            //! # Example
            //!
            //! Complete example source code can be found in the repository:
            //! [`atmega2560-eeprom.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-eeprom.rs)
            //!
            //! ```
            //! const BOOT_COUNT_OFFSET: u16 = 0;
            //!
            //! let dp = atmega_hal::Peripherals::take().unwrap();
            //! let mut eeprom = Eeprom::new(dp.EEPROM);
            //!
            //! let mut boot_count = eeprom.read_byte(BOOT_COUNT_OFFSET);
            //! boot_count = boot_count.wrapping_add(1);
            //! eeprom.write_byte(BOOT_COUNT_OFFSET, boot_count);
            //!
            //! ufmt::uwriteln!(&mut serial, "Boot count: {}", boot_count).unwrap();
            //! ```
            pub use avr_hal_generic::eeprom::{EepromOps, OutOfBoundsError};

            pub type Eeprom =
                avr_hal_generic::eeprom::Eeprom<crate::$hal::Hal, crate::$hal::pac::EEPROM>;

            $($impl_macro)::+! {
                hal: crate::$hal::Hal,
                peripheral: crate::$hal::pac::EEPROM,
                capacity: $capacity,
                addr_width: $addr_width,
                set_address: |peripheral, address| {
                    peripheral.$addr_reg.write(|w| w.bits(address));
                },
            }
        }

        pub use eeprom::Eeprom;
    }
}
pub(crate) use impl_mod_eeprom;
