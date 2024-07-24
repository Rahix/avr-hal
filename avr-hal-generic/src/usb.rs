use core::cell::Cell;

use usb_device::{bus::{UsbBus, PollResult}, UsbDirection, UsbError, class_prelude::UsbBusAllocator, endpoint::{EndpointAddress, EndpointType}, Result as UsbResult};
use avr_device::interrupt::{CriticalSection, Mutex as AvrDMutex};

const EP_SIZE_8: u8 = 0b000;
const EP_SIZE_16: u8 = 0b001;
const EP_SIZE_32: u8 = 0b010;
const EP_SIZE_64: u8 = 0b011;
const EP_SIZE_128: u8 = 0b100;
const EP_SIZE_256: u8 = 0b101;
const EP_SIZE_512: u8 = 0b110;

#[derive(Default)]
pub struct EndpointTableEntry { // REVIEW: what should the scoping be here?
    is_allocated: bool,
    eptype_bits: u8,
    epdir_bit: bool,
    epsize_bits: u8,
}

impl EndpointTableEntry {
    fn buffer_size(&self) -> usize {
        match self.epsize_bits {
            EP_SIZE_8 => 8,
            EP_SIZE_16 => 16,
            EP_SIZE_32 => 32,
            EP_SIZE_64 => 64,
            EP_SIZE_128 => 128,
            EP_SIZE_256 => 256,
            EP_SIZE_512 => 512,
            _ => unreachable!(),
        }
    }
}

// Using Macro 2.0 here while not stable yet makes this code a lot more readable and easier to write
pub macro create_usb_bus (
    $USB_DEVICE:ty,
    $MAX_ENDPOINTS:literal,
) {
    pub struct AvrUsbBus<S: SuspendNotifier> {
        usb: AvrDMutex<$USB_DEVICE>,
        suspend_notifier: AvrDMutex<S>,
        pending_ins: AvrDMutex<Cell<u8>>,
        endpoints: [EndpointTableEntry; $MAX_ENDPOINTS],
        dpram_usage: u16, // TODO: This need to be extracted
    }

    impl AvrUsbBus<()> {
        /// Create a new UsbBus without power-saving functionality.
        ///
        /// If you would like to disable the PLL when the USB peripheral is
        /// suspended, then construct the bus with [`UsbBus::with_suspend_notifier`].
        pub fn new(usb: $USB_DEVICE) -> UsbBusAllocator<Self> {
            Self::with_suspend_notifier(usb, ())
        }
    }
    
    impl<S: SuspendNotifier> AvrUsbBus<S> {
        /// Create a UsbBus with a suspend and resume handler.
        ///
        /// If you want the PLL to be automatically disabled when the USB peripheral
        /// is suspended, then you can pass the PLL resource here; for example:
        ///
        /// ```
        /// use avr_device::atmega32u4::Peripherals;
        /// use atmega_usbd::UsbBus;
        ///
        /// let dp = Peripherals.take().unwrap();
        /// // ... (other initialization stuff)
        /// let bus = UsbBus::with_suspend_notifier(dp.USB_DEVICE, dp.PLL);
        /// ```
        ///
        /// **Note: If you are using the PLL output for other peripherals like the
        /// high-speed timer, then disabling the PLL may affect the behavior of
        /// those peripherals.** In such cases, you can either use [`UsbBus::new`]
        /// to leave the PLL running, or implement [`SuspendNotifier`] yourself,
        /// with some custom logic to gracefully shut down the PLL in cooperation
        /// with your other peripherals.
        pub fn with_suspend_notifier(usb: $USB_DEVICE, suspend_notifier: S) -> UsbBusAllocator<Self> {
            UsbBusAllocator::new(Self {
                usb: AvrDMutex::new(usb),
                suspend_notifier: AvrDMutex::new(suspend_notifier),
                pending_ins: AvrDMutex::new(Cell::new(0)),
                endpoints: Default::default(),
                dpram_usage: 0,
            })
        }

        fn active_endpoints(&self) -> impl Iterator<Item = (usize, &EndpointTableEntry)> {
            self.endpoints
                .iter()
                .enumerate() // why enumerate then immediately drop?
                .filter(|&(_, ep)| ep.is_allocated)
        }

        fn set_current_endpoint(&self, cs: CriticalSection, index: usize) -> Result<(), UsbError> {
            if index >= $MAX_ENDPOINTS {
                return Err(UsbError::InvalidEndpoint);
            }
            let usb = self.usb.borrow(cs);
            // TODO: the rest of this needs to be abstracted
            if usb.usbcon.read().frzclk().bit_is_set() {
                return Err(UsbError::InvalidState);
            }
            usb.uenum.write(|w| w.bits(index as u8));
            if usb.uenum.read().bits() & 0b111 != (index as u8) {
                return Err(UsbError::InvalidState);
            }
            Ok(())
        }

        fn endpoint_byte_count(&self, cs: CriticalSection) -> u16 {
            let usb = self.usb.borrow(cs);
            // FIXME: Potential for desync here? LUFA doesn't seem to care.
            ((usb.uebchx.read().bits() as u16) << 8) | (usb.uebclx.read().bits() as u16)
        }

        fn configure_endpoint(&self, cs: CriticalSection, index: usize) -> Result<(), UsbError> {
            let usb = self.usb.borrow(cs);
            self.set_current_endpoint(cs, index)?;
            let endpoint = &self.endpoints[index];
    
            usb.ueconx.modify(|_, w| w.epen().set_bit());
            usb.uecfg1x.modify(|_, w| w.alloc().clear_bit());
    
            usb.uecfg0x.write(|w| {
                w.epdir()
                    .bit(endpoint.epdir_bit)
                    .eptype()
                    .bits(endpoint.eptype_bits)
            });
            usb.uecfg1x
                .write(|w| w.epbk().bits(0).epsize().bits(endpoint.epsize_bits));
            usb.uecfg1x.modify(|_, w| w.alloc().set_bit());
    
            assert!(
                usb.uesta0x.read().cfgok().bit_is_set(),
                "could not configure endpoint {}",
                index
            );
    
            usb.ueienx
                .modify(|_, w| w.rxoute().set_bit().rxstpe().set_bit());
            Ok(())
        }
    }

    impl<S: SuspendNotifier> UsbBus for AvrUsbBus<S> {
        fn alloc_ep(
            &mut self,
            ep_dir: UsbDirection,
            ep_addr: Option<EndpointAddress>,
            ep_type: EndpointType,
            max_packet_size: u16,
            _interval: u8,
        ) -> Result<EndpointAddress, UsbError> {
            unimplemented!()
        }

        fn enable(&mut self) {
            unimplemented!()
        }

        fn reset(&self) {
            unimplemented!()
        }

        fn set_device_address(&self, addr: u8) {
            unimplemented!()
        }

        fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
            unimplemented!()
        }

        fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
            unimplemented!()
        }

        fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
            unimplemented!()
        }
        
        fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
            unimplemented!()
        }

        fn suspend(&self) {
            unimplemented!()
        }

        fn resume(&self) {
            unimplemented!()
        }

        fn poll(&self) -> PollResult {
            unimplemented!()
        }
    }
}


/// Receiver for handling suspend and resume events from the USB device.
///
/// See [`UsbBus::with_suspend_notifier`] for more details.
pub trait SuspendNotifier: Send + Sized + 'static {
    /// Called by `UsbBus` when the USB peripheral has been suspended and the
    /// PLL is safe to shut down.
    fn suspend(&self) {}

    /// Called by `UsbBus` when the USB peripheral is about to resume and is
    /// waiting for PLL to be enabled.
    ///
    /// This function should block until PLL lock has been established.
    fn resume(&self) {}
}

impl SuspendNotifier for () {}