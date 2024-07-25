
pub use avr_hal_generic::usb::*;

#[cfg(any(feature = "atmega32u4", feature = "atmega8u2"))]
use crate::pac::USB_DEVICE;

// use crate::pac::USB0 as USB_DEVICE;

avr_hal_generic::usb::create_usb_bus! {
	USB_DEVICE,
	8,
}