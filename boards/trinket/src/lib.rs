#![no_std]

pub extern crate attiny85_hal as hal;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use hal::entry;
/// See [`avr_device::interrupt`](https://docs.rs/avr-device/latest/avr_device/attr.interrupt.html).
#[cfg(feature = "rt")]
pub use hal::interrupt;

pub use attiny85_hal::attiny85;
pub use crate::attiny85::Peripherals;
pub use attiny85_hal::prelude;

pub type Delay = hal::delay::Delay<hal::clock::MHz8>;

pub use crate::pins::*;
mod pins {
    use attiny85_hal::port::PortExt;

    avr_hal_generic::impl_board_pins! {
        #[port_defs]
        use attiny85_hal::port;

        /// Generic DDR (not strictly necessary for ATtiny85)
        pub struct DDR {
            portb: crate::attiny85::PORTB,
        }

        /// Reexport of the pins with names as on the Trinket board
        pub struct Pins {
            /// `#0`: `PB0`, `DI`(SPI), `SDA`(I2C)
            pub d0: portb::pb0::PB0,
            /// `#1`: `PB1`, `DO`(SPI), Builtin LED
            pub d1: portb::pb1::PB1,
            /// `#2`: `PB2`, `SCK`(SPI), `SCL`(I2C)
            pub d2: portb::pb2::PB2,
            /// `#3`: `PB3`
            pub d3: portb::pb3::PB3,
            /// `#4`: `PB4`
            pub d4: portb::pb4::PB4,
        }
    }
}
