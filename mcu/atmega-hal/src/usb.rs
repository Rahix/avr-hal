
pub use avr_hal_generic::usb::*;

// MARK: - Type Imports

#[cfg(any(feature = "atmega32u4", feature = "atmega8u2"))]
use crate::pac::{
	usb_device::{udint, UDINT, ueintx, UEINTX},
	USB_DEVICE
};

#[cfg(feature = "atmega32u4")]
use crate::pac::usb_device::{usbint, USBINT};

#[cfg(feature = "atmega8u2")]
type USBINT = !; // atmega8u2 does not have USBINT register
#[cfg(feature = "atmega8u2")]
const usbint: USBINT = !(); // atmega8u2 does not have USBINT register

// MARK: - Constants by device

#[cfg(feature = "atmega32u4")]
const MAX_ENDPOINTS: usize = 7;
#[cfg(feature = "atmega32u4")]
const ENDPOINT_MAX_BUFSIZE: [u16; MAX_ENDPOINTS] = [64, 256, 64, 64, 64, 64, 64];
#[cfg(feature = "atmega32u4")]
const DPRAM_SIZE: u16 = 832;

#[cfg(feature = "atmega8u2")]
const MAX_ENDPOINTS: usize = 5;
#[cfg(feature = "atmega8u2")]
const ENDPOINT_MAX_BUFSIZE: [u16; MAX_ENDPOINTS] = [64, 64, 64, 64, 64];
#[cfg(feature = "atmega8u2")]
const DPRAM_SIZE: u16 = 176;

avr_hal_generic::usb::create_usb_bus! {
	USB_DEVICE,
	UDINT, // REVIEW: how should i be passing these down? (do we want to pass the registers too or just the type?)
	UEINTX,
	USBINT,
	MAX_ENDPOINTS,
	ENDPOINT_MAX_BUFSIZE,
	DPRAM_SIZE,
}

/// Extension trait for conveniently clearing AVR interrupt flag registers.
///
/// To clear an interrupt flag, a zero bit must be written. However, there are
/// several other hazards to take into consideration:
///
/// 1. If you read-modify-write, it is possible that an interrupt flag will be
///   set by hardware in between the read and write, and writing the zero that
///   you previously read will clear that flag as well. So, use a default value
///   of all ones and specifically clear the bits you want. HOWEVER:
///
/// 2. Some bits of the interrupt flag register are reserved, and it is
///   specified that they should not be written as ones.
///
/// Implementers of this trait should provide an initial value to the callback
/// with all _known_ interrupt flags set to the value that has no effect (which
/// is 1, in most cases)
pub trait ClearInterrupts { // This trait must live here due to the orphan rule
    type Writer;

    fn clear_interrupts<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&mut Self::Writer) -> &mut Self::Writer;
}

impl ClearInterrupts for UDINT {
    type Writer = udint::W;

    fn clear_interrupts<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&mut Self::Writer) -> &mut Self::Writer,
    {
        // Bits 1,7 reserved as do not set. Setting all other bits has no effect
        self.write(|w| f(unsafe { w.bits(0x7d) }))
    }
}

impl ClearInterrupts for UEINTX {
    type Writer = ueintx::W;

    fn clear_interrupts<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&mut Self::Writer) -> &mut Self::Writer,
    {
        // Bit 5 read-only. Setting all other bits has no effect, EXCEPT:
        //  - RXOUTI/KILLBK should not be set for "IN" endpoints (XXX end-user beware)
        self.write(|w| f(unsafe { w.bits(0xdf) }))
    }
}

#[cfg(not(feature = "atmega8u2"))]
impl ClearInterrupts for USBINT {
    type Writer = usbint::W;

    fn clear_interrupts<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&mut Self::Writer) -> &mut Self::Writer,
    {
        // Bits 7:1 are reserved as do not set.
        self.write(|w| f(unsafe { w.bits(0x01) }))
    }
}
