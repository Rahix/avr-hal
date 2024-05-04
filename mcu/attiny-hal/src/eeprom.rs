//! EEPROM
//!
//! # Example
//!
//! For full source code, please refer to the ATmega EEPROM example:
//! [`atmega2560-eeprom.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-eeprom.rs)
//!
//! ```
//! const BOOT_COUNT_OFFSET: u16 = 0;
//!
//! let dp = attiny_hal::Peripherals::take().unwrap();
//! let mut eeprom = Eeprom::new(dp.EEPROM);
//!
//! let mut boot_count = eeprom.read_byte(BOOT_COUNT_OFFSET);
//! boot_count = boot_count.wrapping_add(1);
//! eeprom.write_byte(BOOT_COUNT_OFFSET, boot_count);
//!
//! ufmt::uwriteln!(&mut serial, "Boot count: {}", boot_count).unwrap();
//! ```

pub use avr_hal_generic::eeprom::{EepromOps, OutOfBoundsError};

pub type Eeprom = avr_hal_generic::eeprom::Eeprom<crate::Attiny, crate::pac::EEPROM>;

#[cfg(feature = "attiny2313")]
avr_hal_generic::impl_eeprom_attiny! {
    hal: crate::Attiny,
    peripheral: crate::pac::EEPROM,
    capacity: 128,
    addr_width: u8,
    set_address: |peripheral, address| {
        peripheral.eear().write(|w| w.bits(address));
    },
}

#[cfg(any(feature = "attiny167", feature = "attiny85"))]
avr_hal_generic::impl_eeprom_attiny! {
    hal: crate::Attiny,
    peripheral: crate::pac::EEPROM,
    capacity: 512,
    addr_width: u16,
    set_address: |peripheral, address| {
        peripheral.eear().write(|w| w.bits(address));
    },
}

#[cfg(feature = "attiny88")]
avr_hal_generic::impl_eeprom_attiny! {
    hal: crate::Attiny,
    peripheral: crate::pac::EEPROM,
    capacity: 64,
    addr_width: u8,
    set_address: |peripheral, address| {
        peripheral.eearl().write(|w| w.bits(address));
    },
}
