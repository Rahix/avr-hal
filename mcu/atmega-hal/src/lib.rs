#![no_std]
#![feature(asm_experimental_arch)]

//! `atmega-hal`
//! =============
//! Common HAL (hardware abstraction layer) for ATmega* microcontrollers.
//!
//! **Note**: This version of the documentation was built for
#![cfg_attr(feature = "atmega48p", doc = "**ATmega48P**.")]
#![cfg_attr(feature = "atmega16", doc = "**ATmega16**.")]
#![cfg_attr(feature = "atmega164pa", doc = "**ATmega164PA**.")]
#![cfg_attr(feature = "atmega168", doc = "**ATmega168**.")]
#![cfg_attr(feature = "atmega328p", doc = "**ATmega328P**.")]
#![cfg_attr(feature = "atmega328pb", doc = "**ATmega328PB**.")]
#![cfg_attr(feature = "atmega32a", doc = "**ATmega32a**.")]
#![cfg_attr(feature = "atmega32u4", doc = "**ATmega32U4**.")]
#![cfg_attr(feature = "atmega2560", doc = "**ATmega2560**.")]
#![cfg_attr(feature = "atmega128a", doc = "**ATmega128A**.")]
#![cfg_attr(feature = "atmega1280", doc = "**ATmega1280**.")]
#![cfg_attr(feature = "atmega1284p", doc = "**ATmega1284P**.")]
#![cfg_attr(feature = "atmega8", doc = "**ATmega8**.")]
#![cfg_attr(feature = "atmega88p", doc = "**ATmega88P**.")]
//! This means that only items which are available for this MCU are visible.  If you are using
//! a different chip, try building the documentation locally with:
//!
//! ```text
//! cargo doc --features <your-mcu> --open
//! ```

#[cfg(all(
    not(feature = "device-selected"),
    not(feature = "disable-device-selection-error")
))]
compile_error!(
    "This crate requires you to specify your target chip as a feature.

    Please select one of the following

    * atmega48p
    * atmega16
    * atmega164pa
    * atmega168
    * atmega328p
    * atmega328pb
    * atmega32u4
    * atmega128a
    * atmega1280
    * atmega2560
    * atmega1284p
    * atmega8
    * atmega88p
    "
);

/// Reexport of `atmega1280` from `avr-device`
///
#[cfg(feature = "atmega1280")]
pub use avr_device::atmega1280 as pac;
/// Reexport of `atmega1284p` from `avr-device`
///
#[cfg(feature = "atmega1284p")]
pub use avr_device::atmega1284p as pac;
/// Reexport of `atmega128a` from `avr-device`
///
#[cfg(feature = "atmega128a")]
pub use avr_device::atmega128a as pac;
/// Reexport of `atmega16` from `avr-device`
///
#[cfg(feature = "atmega16")]
pub use avr_device::atmega16 as pac;
/// Reexport of `atmega164pa` from `avr-device`
///
#[cfg(feature = "atmega164pa")]
pub use avr_device::atmega164pa as pac;
/// Reexport of `atmega168` from `avr-device`
///
#[cfg(feature = "atmega168")]
pub use avr_device::atmega168 as pac;
/// Reexport of `atmega2560` from `avr-device`
///
#[cfg(feature = "atmega2560")]
pub use avr_device::atmega2560 as pac;
/// Reexport of `atmega328p` from `avr-device`
///
#[cfg(feature = "atmega328p")]
pub use avr_device::atmega328p as pac;
/// Reexport of `atmega328pb` from `avr-device`
///
#[cfg(feature = "atmega328pb")]
pub use avr_device::atmega328pb as pac;
/// Reexport of `atmega32a` from `avr-device`
///
#[cfg(feature = "atmega32a")]
pub use avr_device::atmega32a as pac;
/// Reexport of `atmega32u4` from `avr-device`
///
#[cfg(feature = "atmega32u4")]
pub use avr_device::atmega32u4 as pac;
/// Reexport of `atmega48p` from `avr-device`
///
#[cfg(feature = "atmega48p")]
pub use avr_device::atmega48p as pac;
/// Reexport of `atmega8` from `avr-device`
///
#[cfg(feature = "atmega8")]
pub use avr_device::atmega8 as pac;
/// Reexport of `atmega88p` from `avr-device`
///
#[cfg(feature = "atmega88p")]
pub use avr_device::atmega88p as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

#[cfg(feature = "device-selected")]
pub use pac::Peripherals;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;
pub use avr_hal_generic::prelude;

#[cfg(feature = "atmega32u4")]
mod usb;

// TODO: fix bad usb-device::UsbBus link
/// This function provides a safe abstraction layer over the USB hardware, by way of the
/// [UsbBus](usb-device::UsbBus) trait.
///
/// There are a few notable limitations, however:
///
/// * This implementation requires exclusive access to the PLL, even though on a hardware
///   level it is possible for the PLL output to be used by both the USB controller and
///   the high-speed timer (TC4) simultaneously. Refer to GitHub issue #TBD for details.
///
///   TODO if Rahix agrees that this limitation isn't something we need to worry about
///   as part of PR, then create a GitHub issue so that someone else can fix it later:
///
///   > **Title**
///   >
///   > Allow the USB and TC4 hardware to both use the PLL output simultaneously
///   >
///   > **Description**
///   >
///   > Our current UsbBus implementation prevents TC4 from using PLL output, even though
///   > the hardware supports it. There are two main
///   > problems that we need to solve first:
///   >
///   > 1. The current UsbBus implementation sets the PLL output to 48MHz. This could
///   >    cause problems if the user has already configured TC4 to expect a different
///   >    clock speed from the PLL.
///   >
///   > 2. We need to make the USB suspend state configurable. Currently when the USB
///   >    bus is idle for 3ms or longer, it will disable the PLL to reduce power usage.
///   >    However, this may not be desirable if TC4 is also using the PLL.
///   >
///   > **Comment**
///   >
///   > I think we *might* be able to solve this by splitting the constructor's
///   > argument into two separate parts. Instead of passing ownership of the entire PLL
///   > configuration (`pll: avr_device::atmega32u4::PLL`), we'd have one argument for
///   > the registers that config the PLL clock speed (e.g. `pll_config: &PLLFRQ`) and one
///   > optional argument for the registers that we use to turn the PLL on and off
///   > (e.g. `pll_suspend: Option<&mut pllcsr>`). A value of `None` would indicate that
///   > the user wants us to keep the PLL running while USB is idle.
///   >
///   > A few disclaimers:
///   >
///   > * This is a simplification. Instead of `pll_suspend: Option<&mut pllcsr>` we'd
///   >   probably want to define a new trait,
///   >   similar to what is done [in the `agausmann/atmega-usbd` repo](https://github.com/agausmann/atmega-usbd/blob/5fc68ca813ce0a37dab65dd4d66efe1ec125f2a8/src/lib.rs#L590-L618).
///   >
///   > * This is just one possible solution; there are others.
///   >
///   > * I've not spent much time investigating this, so this proposed solution might not work.
///
/// * The current implementation does not attempt to minimize power usage. For details,
///   see GitHub issue #TBD.
///
///   TODO if Rahix agrees that this limitation isn't something we need to worry about
///   as part of PR, then create a GitHub issue so that someone else can fix it later:
///
///   * Add support for using interrupts, in addition to polling.
///     Similar to `agausmann/atmega-usbd`.
///
///   * Shutdown the PLL when the USB module is suspended (TODO: do in this PR?)
///
///   * and more?
///
/// * The underlying struct that implements `UsbBus` is private. This is done intentionally
///   in order to make it easier to address the other issues without breaking backwards
///   compatibility.
#[cfg(feature = "atmega32u4")]
pub fn default_usb_bus_with_pll<CLOCKUSB: ClockUSB>(
    usb: avr_device::atmega32u4::USB_DEVICE,
    pll: avr_device::atmega32u4::PLL,
) -> impl usb_device::class_prelude::UsbBus
where
    avr_hal_generic::delay::Delay<CLOCKUSB>: embedded_hal::delay::DelayNs,
{
    return usb::UsbdBus::<CLOCKUSB>::new(usb, pll);
}

#[cfg(feature = "device-selected")]
pub mod adc;
#[cfg(feature = "device-selected")]
pub use adc::Adc;

#[cfg(feature = "device-selected")]
pub mod i2c;
#[cfg(feature = "device-selected")]
pub use i2c::I2c;

#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(feature = "device-selected")]
pub use spi::Spi;

#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub use port::Pins;

#[cfg(feature = "device-selected")]
pub mod simple_pwm;

#[cfg(feature = "device-selected")]
pub mod usart;
#[cfg(feature = "device-selected")]
pub use usart::Usart;

#[cfg(feature = "device-selected")]
pub mod wdt;
#[cfg(feature = "atmega32u4")]
use usb::ClockUSB;
#[cfg(feature = "device-selected")]
pub use wdt::Wdt;

#[cfg(feature = "device-selected")]
pub mod eeprom;
#[cfg(feature = "device-selected")]
pub use eeprom::Eeprom;

pub struct Atmega;

#[cfg(any(
    feature = "atmega48p",
    feature = "atmega88p",
    feature = "atmega168",
    feature = "atmega328p"
))]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTB, $p.PORTC, $p.PORTD)
    };
}
#[cfg(any(feature = "atmega16", feature = "atmega164pa"))]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
    };
}
#[cfg(feature = "atmega328pb")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE)
    };
}
#[cfg(feature = "atmega32u4")]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF)
    };
}

#[cfg(any(feature = "atmega128a"))]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new(
            $p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF, $p.PORTG,
        )
    };
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new(
            $p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF, $p.PORTG, $p.PORTH,
            $p.PORTJ, $p.PORTK, $p.PORTL,
        )
    };
}

#[cfg(any(feature = "atmega1284p", feature = "atmega32a"))]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
    };
}

#[cfg(any(feature = "atmega8"))]
#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTB, $p.PORTC, $p.PORTD)
    };
}
