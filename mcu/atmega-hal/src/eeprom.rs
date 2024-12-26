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

pub type Eeprom = avr_hal_generic::eeprom::Eeprom<crate::Atmega, crate::pac::EEPROM>;

///////////////////////////////////////////////////////////
#[cfg(feature = "atmega48p")]
avr_hal_generic::impl_eeprom_atmega! {
    hal: crate::Atmega,
    peripheral: crate::pac::EEPROM,
    capacity: 256,
    addr_width: u8,
    set_address: |peripheral, address| {
        peripheral.eearl.write(|w| w.bits(address));
    },
}

#[cfg(any(feature = "atmega168", feature = "atmega164pa"))]
avr_hal_generic::impl_eeprom_atmega! {
    hal: crate::Atmega,
    peripheral: crate::pac::EEPROM,
    capacity: 512,
    addr_width: u16,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(any(
    feature = "atmega328pb",
    feature = "atmega328p",
    feature = "atmega32u4"
))]
avr_hal_generic::impl_eeprom_atmega! {
    hal: crate::Atmega,
    peripheral: crate::pac::EEPROM,
    capacity: 1024,
    addr_width: u16,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(any(
    feature = "atmega2560",
    feature = "atmega1280",
    feature = "atmega1284p"
))]
avr_hal_generic::impl_eeprom_atmega! {
    hal: crate::Atmega,
    peripheral: crate::pac::EEPROM,
    capacity: 4096,
    addr_width: u16,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(any(feature = "atmega8"))]
avr_hal_generic::impl_eeprom_atmega_old! {
    hal: crate::Atmega,
    peripheral: crate::pac::EEPROM,
    capacity: 512,
    addr_width: u16,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(any(feature = "atmega32a"))]
avr_hal_generic::impl_eeprom_atmega_old! {
    hal: crate::Atmega,
    peripheral: crate::pac::EEPROM,
    capacity: 1024,
    addr_width: u16,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}

#[cfg(any(feature = "atmega128a",))]
avr_hal_generic::impl_eeprom_atmega_old! {
    hal: crate::Atmega,
    peripheral: crate::pac::EEPROM,
    capacity: 4096,
    addr_width: u16,
    set_address: |peripheral, address| {
        peripheral.eear.write(|w| w.bits(address));
    },
}
