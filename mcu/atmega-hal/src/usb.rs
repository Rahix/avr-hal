
pub use avr_hal_generic::usb::*;

#[cfg(feature = "atmega32u4")]
use avr_device::atmega32u4::USB_DEVICE;


avr_hal_generic::usb::create_usb_bus! {
	USB_DEVICE,
	8,
}