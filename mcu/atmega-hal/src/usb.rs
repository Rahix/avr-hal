use core::cell::Cell;

use avr_device::atmega32u4::PLL;
use avr_device::atmega32u4::USB_DEVICE;
use avr_device::interrupt::Mutex;
use usb_device::bus::PollResult;
use usb_device::bus::UsbBus;
use usb_device::endpoint::EndpointAddress;
use usb_device::endpoint::EndpointType;
use usb_device::UsbDirection;
use usb_device::UsbError;

const MAX_ENDPOINTS: usize = 7;

struct EndpointTableEntry {
    _ep_type: EndpointType,
    _direction: UsbDirection,
    _max_packet_size: u16,
}

pub struct UsbdBus {
    _usb: Mutex<USB_DEVICE>,
    _pll: Mutex<PLL>,
    _pending_ins: Mutex<Cell<u8>>,
    _endpoints: [Option<EndpointTableEntry>; MAX_ENDPOINTS],
}

impl UsbdBus {
    pub fn new(_usb: USB_DEVICE, _pll: PLL) -> Self {
        todo!();
    }
}

impl UsbBus for UsbdBus {
    fn alloc_ep(
        &mut self,
        _direction: UsbDirection,
        _ep_addr: Option<EndpointAddress>,
        _ep_type: EndpointType,
        _max_packet_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        todo!();
    }

    fn enable(&mut self) {
        todo!();
    }

    fn reset(&self) {
        todo!();
    }

    fn set_device_address(&self, _addr: u8) {
        todo!();
    }

    fn write(&self, _ep_addr: EndpointAddress, _buf: &[u8]) -> Result<usize, UsbError> {
        todo!();
    }

    fn read(&self, _ep_addr: EndpointAddress, _buf: &mut [u8]) -> Result<usize, UsbError> {
        todo!();
    }

    fn set_stalled(&self, _ep_addr: EndpointAddress, _stalled: bool) {
        todo!();
    }

    fn is_stalled(&self, _ep_addr: EndpointAddress) -> bool {
        todo!();
    }

    fn suspend(&self) {
        todo!();
    }

    fn resume(&self) {
        todo!();
    }

    fn poll(&self) -> PollResult {
        todo!();
    }
}

impl Drop for UsbdBus {
    fn drop(&mut self) {
        todo!()
    }
}
