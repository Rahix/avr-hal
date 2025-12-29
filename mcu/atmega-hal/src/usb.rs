use avr_device::atmega32u4::PLL;
use avr_device::atmega32u4::USB_DEVICE;
use usb_device::bus::PollResult;
use usb_device::bus::UsbBus;
use usb_device::endpoint::EndpointAddress;
use usb_device::endpoint::EndpointType;
use usb_device::UsbDirection;
use usb_device::UsbError;

// TODO: I'm not too familiar with naming conventions in avr-hal (or Rust, for
// that matter). Is `UsbdBus` acceptable?
//
// I decided to use the same name that the `musb` crate uses for their
// implementation; I recall seeing somewhere that "usbd" was an abbreviation for
// the "usb-device" crate.
pub struct UsbdBus {}

impl UsbdBus {
    // TODO: I'm not sure that the arguments to the `new` function are
    // correct; there's a chance that they'll need to change during
    // implementation.
    //
    // Considering this example code:
    // https://github.com/agausmann/atmega-usbd/blob/master/examples/arduino_keyboard.rs#L54-L78
    // https://github.com/agausmann/atmega-usbd/blob/master/src/lib.rs#L69-L77
    //
    // * USB_DEVICE is clearly required.
    //
    // * The PLL will definitely need to be modified, and I'm inclined to
    //   think that we should just take ownership and do it here.
    //
    //   At first glance, it looks like the PLL can simultanously drive both the
    //   USB controller and high speed timer(s). If so, there *might* be a
    //   usecase where the user initializes PLL, then passes a shared reference
    //   into this constructor. I don't think that's worth worring about though;
    //   we can always add a new constructor later, and I don't want every
    //   single user to have duplicate code for initializing PLL.
    //
    // * I don't *think* there's anything else we need?
    pub fn new(_usb: USB_DEVICE, _pll: PLL) -> Self {
        todo!();
    }
}

// TODO: implement this trait using code from
// https://github.com/agausmann/atmega-usbd/blob/master/src/lib.rs
impl UsbBus for UsbdBus {
    fn alloc_ep(
        &mut self,
        _: UsbDirection,
        _: Option<EndpointAddress>,
        _: EndpointType,
        _: u16,
        _: u8,
    ) -> Result<EndpointAddress, UsbError> {
        todo!()
    }
    fn enable(&mut self) {
        todo!()
    }
    fn reset(&self) {
        todo!()
    }
    fn set_device_address(&self, _: u8) {
        todo!()
    }
    fn write(&self, _: EndpointAddress, _: &[u8]) -> Result<usize, UsbError> {
        todo!()
    }
    fn read(&self, _: EndpointAddress, _: &mut [u8]) -> Result<usize, UsbError> {
        todo!()
    }
    fn set_stalled(&self, _: EndpointAddress, _: bool) {
        todo!()
    }
    fn is_stalled(&self, _: EndpointAddress) -> bool {
        todo!()
    }
    fn suspend(&self) {
        todo!()
    }
    fn resume(&self) {
        todo!()
    }
    fn poll(&self) -> PollResult {
        todo!()
    }
}
