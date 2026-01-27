use crate::pac::pll::pllcsr::W as pllcsr_writer;
use avr_device::atmega32u4::usb_device::udint;
use avr_device::atmega32u4::usb_device::ueintx;
use avr_device::atmega32u4::usb_device::usbint;
use avr_device::atmega32u4::usb_device::UDINT;
use avr_device::atmega32u4::usb_device::UEINTX;
use avr_device::atmega32u4::usb_device::USBINT;
use avr_device::atmega32u4::PLL;
use avr_device::atmega32u4::USB_DEVICE;
use avr_device::interrupt;
use avr_device::interrupt::CriticalSection;
use avr_device::interrupt::Mutex;
use avr_hal_generic::clock::Clock;
use avr_hal_generic::clock::MHz16;
use avr_hal_generic::clock::MHz8;
use avr_hal_generic::delay::Delay;
use core::cell::Cell;
use core::cmp::max;
use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;
use usb_device::bus::PollResult;
use usb_device::bus::UsbBus;
use usb_device::endpoint::EndpointAddress;
use usb_device::endpoint::EndpointType;
use usb_device::UsbDirection;
use usb_device::UsbError;

const MAX_ENDPOINTS: usize = 7;

// From datasheet section 22.1
const ENDPOINT_MAX_BUFSIZE: [u16; MAX_ENDPOINTS] = [64, 256, 64, 64, 64, 64, 64];

// TODO: do the links work?
/// The USB controller can only be used when the MCU is running at certain
/// specific clock speends. This trait represents those clock speeds.
///
/// There are only two such clock speeds (ref [PLLCSR](PLLCSR)): 16MHz and 8MHz.
/// This trait is already implemented for those speeds, so there is no need for
/// users to ever implement this trait.
pub trait ClockUSB: Clock + Sync {
    /// Configure the PLLCSR.pindiv for this clock speed
    fn setup_pllcsr_pindiv(writer: &mut pllcsr_writer) -> &mut pllcsr_writer;
}

impl ClockUSB for MHz16 {
    fn setup_pllcsr_pindiv(writer: &mut pllcsr_writer) -> &mut pllcsr_writer {
        writer.pindiv().set_bit()
    }
}

impl ClockUSB for MHz8 {
    fn setup_pllcsr_pindiv(writer: &mut pllcsr_writer) -> &mut pllcsr_writer {
        writer.pindiv().clear_bit()
    }
}

// TODO: cleanup these "footnotes". Make sure that they show up properly in the docs, and that I'm able to link to them as expected
// * https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html#footnotes

// FOOTNOTE-EP0: TODO verify
//
// As I understand it, a USB endpoint is *ALWAYS* either IN or OUT. In particular, there are actually
// two endpoint zeros: one for IN, and one for OUT. However, the atmega registers treat these two
// EP0s as if they were a single endpoint. As a result, this introduces a bunch of edge cases where ep0
// needs to be treated specially.

/// Convert the EndpointType enum to the bits used by the eptype field in UECFG0X.
///
/// Refer to section 22.18.2 of the datasheet.
fn eptype_bits_from_ep_type(ep_type: EndpointType) -> u8 {
    match ep_type {
        EndpointType::Control => 0b00,
        EndpointType::Isochronous { .. } => 0b01,
        EndpointType::Bulk => 0b10,
        EndpointType::Interrupt => 0b11,
    }
}

fn epdir_bit_from_direction(direction: UsbDirection) -> bool {
    match direction {
        UsbDirection::In => true,
        UsbDirection::Out => false,
    }
}

fn epsize_bits_from_max_packet_size(max_packet_size: u16) -> u8 {
    let value = max(8, max_packet_size.next_power_of_two());
    match value {
        8 => 0b000,
        16 => 0b001,
        32 => 0b010,
        64 => 0b011,
        128 => 0b100,
        256 => 0b101,
        512 => 0b110,
        _ => unreachable!(),
    }
}

struct EndpointTableEntry {
    ep_type: EndpointType,
    ep_dir: UsbDirection,
    max_packet_size: u16,
}

pub struct UsbdBus<CLOCKUSB: ClockUSB>
where
    Delay<CLOCKUSB>: DelayNs,
{
    usb: Mutex<USB_DEVICE>,
    pll: Mutex<PLL>,
    pending_ins: Mutex<Cell<u8>>,
    endpoints: [Option<EndpointTableEntry>; MAX_ENDPOINTS],
    phantom: PhantomData<CLOCKUSB>,
}

impl<CLOCKUSB: ClockUSB> UsbdBus<CLOCKUSB>
where
    Delay<CLOCKUSB>: DelayNs,
{
    pub fn new(usb: USB_DEVICE, pll: PLL) -> Self {
        Self {
            usb: Mutex::new(usb),
            pll: Mutex::new(pll),
            pending_ins: Mutex::new(Cell::new(0)),
            endpoints: Default::default(),
            phantom: PhantomData,
        }
    }

    fn active_endpoints(&self) -> impl Iterator<Item = (usize, &EndpointTableEntry)> {
        self.endpoints
            .iter()
            .enumerate()
            .filter(|&(_, ep)| ep.is_some())
            .map(|(index, ep)| (index, ep.as_ref().unwrap()))
    }

    /// Docs from the data sheet:
    ///
    /// > Prior to any operation performed by the CPU, the endpoint must first be selected. This
    /// > is done by setting the EPNUM2:0 bits (UENUM register) with the endpoint number which
    /// > will be managed by the CPU.
    /// >
    /// > The CPU can then access to the various endpoint registers and data
    fn set_current_endpoint(&self, cs: CriticalSection, index: usize) -> Result<(), UsbError> {
        if index >= MAX_ENDPOINTS {
            return Err(UsbError::InvalidEndpoint);
        }
        let index: u8 = index.try_into().unwrap();
        let usb = self.usb.borrow(cs);
        if usb.usbcon().read().frzclk().bit_is_set() {
            return Err(UsbError::InvalidState);
        }

        usb.uenum().write(|w| w.set(index));
        let read_back = usb.uenum().read().bits();

        // The `atmeta-usbd` crate uses this bitmask [1]. According to the datasheet the other bits should always read as zero, but I'm leaving this check in just in case.
        //
        // [1] https://github.com/agausmann/atmega-usbd/blob/5fc68ca813ce0a37dab65dd4d66efe1ec125f2a8/src/lib.rs#L126
        assert_eq!(read_back & 0b111, read_back);

        if read_back != index {
            return Err(UsbError::InvalidState);
        }

        Ok(())
    }

    ///  This function returns the full eleven-bit value of the BYCT register.
    ///
    ///  The datasheet's docs for UEBCLX says:
    ///
    ///  > Set by the hardware. BYCT10:0 is:
    ///  > * (for IN endpoint) increased after each writing into the endpoint and decremented after each byte sent,
    ///  > * (for OUT endpoint) increased after each byte sent by the host, and decremented after each byte read by the software.
    fn endpoint_byte_count(&self, cs: CriticalSection) -> u16 {
        let usb = self.usb.borrow(cs);
        // The BYCT register is split across two registers:
        // uebclx (low eight bits) and uebchx (high three bits).
        ((usb.uebchx().read().bits() as u16) << 8) | (usb.uebclx().read().bits() as u16)
    }

    fn get_endpoint_table_entry(
        &self,
        _cs: CriticalSection,
        index: usize,
    ) -> Result<&EndpointTableEntry, UsbError> {
        if let Some(Some(ref endpoint)) = self.endpoints.get(index) {
            Ok(endpoint)
        } else {
            Err(UsbError::InvalidEndpoint)
        }
    }
}

impl<CLOCKUSB: ClockUSB> UsbBus for UsbdBus<CLOCKUSB>
where
    Delay<CLOCKUSB>: DelayNs,
{
    /// This function initializes a single element in `self.endpoints`
    ///
    /// Upstream docs:
    ///
    /// > Allocates an endpoint and specified endpoint parameters. This method is called by the device
    /// > and class implementations to allocate endpoints, and can only be called before
    /// > [`enable`](UsbBus::enable) is called.
    /// >
    /// > # Arguments
    /// >
    /// > * `ep_dir` - The endpoint direction.
    /// > * `ep_addr` - A static endpoint address to allocate. If Some, the implementation should
    /// >   attempt to return an endpoint with the specified address. If None, the implementation
    /// >   should return the next available one.
    /// > * `max_packet_size` - Maximum packet size in bytes.
    /// > * `interval` - Polling interval parameter for interrupt endpoints.
    /// >
    /// > # Errors
    /// >
    /// > * [`EndpointOverflow`](crate::UsbError::EndpointOverflow) - Available total number of
    /// >   endpoints, endpoints of the specified type, or endpoind packet memory has been exhausted.
    /// >   This is generally caused when a user tries to add too many classes to a composite device.
    /// > * [`InvalidEndpoint`](crate::UsbError::InvalidEndpoint) - A specific `ep_addr` was specified
    /// >   but the endpoint in question has already been allocated.
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        // We intentionally don't use a critical section here. This is because, unlike all the other
        // functions in this trait, this function only modifies `self`'s internal state.

        // WLOG: ep_addr is not None
        let Some(addr) = ep_addr else {
            // If it is None, then find an index and do a recursive call.
            let index = self
                .endpoints
                .iter()
                .enumerate()
                .skip(1) // Skip the control endpoint, which is always index zero
                .find_map(|(index, ep)| {
                    if ep.is_none() && max_packet_size <= ENDPOINT_MAX_BUFSIZE[index] {
                        Some(index)
                    } else {
                        None
                    }
                })
                .ok_or(UsbError::EndpointMemoryOverflow)?;
            let ep_addr = Some(EndpointAddress::from_parts(index, ep_dir));
            return self.alloc_ep(ep_dir, ep_addr, ep_type, max_packet_size, interval);
        };

        let index = addr.index();
        if addr.direction() != ep_dir {
            unreachable!("Requested endpoint address has mismatched direction. This suggests a bug in usb-device?");
        }
        if index >= self.endpoints.len() {
            return Err(UsbError::InvalidEndpoint);
        }
        // TODO: is this really necessary?
        //
        // > Ignore duplicate ep0 allocation by usb_device.
        // > Endpoints can only be configured once, and
        // > control endpoint must be configured as "OUT".
        //
        // ref @FOOTNOTE-EP0
        //
        // (FWIW, section 22.18.2's docs for UECFG0X.EPDIR confirm that ep0
        // must be configured as "OUT")
        if index == 0 && addr.direction() == UsbDirection::In {
            return Ok(ep_addr.unwrap());
        }
        if self.endpoints[index].is_some() || max_packet_size > ENDPOINT_MAX_BUFSIZE[index] {
            return Err(UsbError::InvalidEndpoint);
        }

        self.endpoints[addr.index()] = Some(EndpointTableEntry {
            ep_type,
            ep_dir,
            max_packet_size,
        });
        Ok(addr)
    }

    /// Enables and initializes the USB peripheral. Soon after enabling the device will be reset, so
    /// there is no need to perform a USB reset in this method.
    fn enable(&mut self) {
        interrupt::free(|cs| {
            let usb = self.usb.borrow(cs);
            let pll = self.pll.borrow(cs);

            // Explicitly reset pllfrq back to the value that it would have after
            // power cycling the device.
            pll.pllfrq().write(|w| unsafe { w.bits(0b0000_0100) });

            // The Datasheet is quite vague about how to enable the USB controller (section 21.12).
            // So instead, we reference the `USB_ClockEnable` function from Arduino's C++ library:
            // https://github.com/arduino/ArduinoCore-avr/blob/7c38f34da561266e1e5cf7769f0e61b0aa5dda39/cores/arduino/USBCore.cpp#L680-L754
            //
            // > UHWCON |= (1<<UVREGE);                  // power internal reg
            usb.uhwcon().modify(|_, w| w.uvrege().set_bit());

            // > USBCON = (1<<USBE) | (1<<FRZCLK);       // clock frozen, usb enabled
            usb.usbcon()
                .modify(|_, w| w.usbe().set_bit().frzclk().set_bit());

            // > // ATmega32U4
            // > #if F_CPU == 16000000UL
            // >         PLLCSR |= (1<<PINDIV);                   // Need 16 MHz xtal
            // > #elif F_CPU == 8000000UL
            // >         PLLCSR &= ~(1<<PINDIV);                  // Need  8 MHz xtal
            // > #else
            // >         #error "Clock rate of F_CPU not supported"
            // > #endif
            //
            // Using an `if` would arguably be the cleaner solution. However if we wanted to
            // do that while still enforcing the 8MHz/16MHz requirement at compile time, then
            // we would have to move `usb.rs` from `mcu/atmega-hal` to `arduino-hal` (which
            // contains the definition of `DefaultClock`).
            pll.pllcsr().modify(|_, w| CLOCKUSB::setup_pllcsr_pindiv(w));

            // > PLLCSR |= (1<<PLLE);
            // > while (!(PLLCSR & (1<<PLOCK)))          // wait for lock pll
            // > {
            // > }
            pll.pllcsr().modify(|_, w| w.plle().set_bit());
            while pll.pllcsr().read().plock().bit_is_clear() {}

            // > // Some tests on specific versions of macosx (10.7.3), reported some
            // > // strange behaviors when the board is reset using the serial
            // > // port touch at 1200 bps. This delay fixes this behavior.
            // > delay(1);
            let mut delay = Delay::<CLOCKUSB>::new();
            delay.delay_ms(1);

            // > USBCON = (USBCON & ~(1<<FRZCLK)) | (1<<OTGPADE);        // start USB clock, enable VBUS Pad
            usb.usbcon().modify(|_, w| {
                w.frzclk()
                    .clear_bit()
                    .otgpade()
                    .set_bit()
                    // Note: We need vbuste in order to be able to detect changes to `usbsta.vbus`
                    // TODO: Why does the C++ code not do this?
                    .vbuste()
                    .set_bit()
            });

            // > UDCON &= ~((1<<RSTCPU) | (1<<LSM) | (1<<RMWKUP) | (1<<DETACH)); // enable attach resistor, set full speed mode
            usb.udcon().modify(|_, w| {
                w.rstcpu()
                    .clear_bit()
                    .lsm()
                    .clear_bit()
                    .rmwkup()
                    .clear_bit()
                    .detach()
                    .clear_bit()
            });
        });
    }

    /// Called when the host resets the device. This will be soon called after
    /// [`poll`](crate::device::UsbDevice::poll) returns [`PollResult::Reset`]. This method should
    /// reset the state of all endpoints and peripheral flags back to a state suitable for
    /// enumeration, as well as ensure that all endpoints previously allocated with alloc_ep are
    /// initialized as specified.
    fn reset(&self) {
        interrupt::free(|cs| {
            let usb = self.usb.borrow(cs);

            // This is NOT ported from C++.
            //
            // TODO: is this actually necessary? If so, explain why.
            usb.udint().modify(|_, w| w.eorsti().clear_bit());

            // Refer to the `InitEndpoints` function from:
            // https://github.com/arduino/ArduinoCore-avr/blob/7c38f34da561266e1e5cf7769f0e61b0aa5dda39/cores/arduino/USBCore.cpp#L364-L382

            // > /* Copyright (c) 2010, Peter Barrett
            // > ** Sleep/Wakeup support added by Michael Dreher
            // > **
            // > ** Permission to use, copy, modify, and/or distribute this software for
            // > ** any purpose with or without fee is hereby granted, provided that the
            // > ** above copyright notice and this permission notice appear in all copies.
            // > **
            // > ** THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
            // > ** WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
            // > ** WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR
            // > ** BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES
            // > ** OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
            // > ** WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
            // > ** ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
            // > ** SOFTWARE.
            // > */
            // TODO: loop over all endpoints, not just the active ones? e.g. so we can free unused memory
            // > for (u8 i = 1; i < sizeof(_initEndpoints) && _initEndpoints[i] != 0; i++)
            // > {
            for (index, _ep) in self.active_endpoints() {
                let endpoint = self.get_endpoint_table_entry(cs, index).unwrap();

                // > UENUM = i;
                self.set_current_endpoint(cs, index).unwrap();

                // > UECONX = (1<<EPEN);
                usb.ueconx().modify(|_, w| w.epen().set_bit());

                // Nobody else does this (TODO!?), but if the `alloc` bit is already
                // set then it's a good idea to toggle it off first.
                //
                // This ensures that there isn't any wasted memory in between `index`'s buffer and
                // `index - 1`'s buffer (refer to section 21.9: Memory Management).
                usb.uecfg1x().modify(|_, w| w.alloc().clear_bit());

                // > UECFG0X = _initEndpoints[i];
                //
                // `_initEndpoints` is initialized with these values: https://github.com/arduino/ArduinoCore-avr/blob/7c38f34da561266e1e5cf7769f0e61b0aa5dda39/cores/arduino/USBAPI.h#L51-L53
                //
                //     > #define EP_TYPE_BULK_IN      ((1<<EPTYPE1) | (1<<EPDIR))
                //     > #define EP_TYPE_BULK_OUT      (1<<EPTYPE1)
                //     > #define EP_TYPE_INTERRUPT_IN ((1<<EPTYPE1) | (1<<EPTYPE0) | (1<<EPDIR))
                usb.uecfg0x().write(|w| unsafe {
                    w.epdir()
                        .bit(epdir_bit_from_direction(endpoint.ep_dir))
                        .eptype()
                        .bits(eptype_bits_from_ep_type(endpoint.ep_type))
                });

                // > #if USB_EP_SIZE == 16
                // > ...
                // > #elif USB_EP_SIZE == 64
                // > UECFG1X = EP_DOUBLE_64;
                // > #else
                // > ...
                // > #endif
                //
                // EP_DOUBLE_64 is defined as 0x36 == 0b0011_0110. This corresponds to:
                // * epsize: 0b011 (64 bits)
                // * epbk: 0b00 (single bank mode)
                // * alloc: 0b1 (allocate)
                //
                // TODO: Why are we using single bank mode, even
                // though the C++ code uses double bank mode?
                usb.uecfg1x().write(|w| unsafe {
                    w.epbk()
                        .bits(0b00)
                        .epsize()
                        .bits(epsize_bits_from_max_packet_size(endpoint.max_packet_size))
                        .alloc()
                        .set_bit()
                });

                // Check CFGOK (config okay) to make sure that everything works
                //
                // The C++ code doesn't bother with this for some reason.
                assert!(
                    usb.uesta0x().read().cfgok().bit_is_set(),
                    "could not configure endpoint {}",
                    index
                );
            }
            // > }

            // > UERST = 0x7E;        // And reset them
            // > UERST = 0;
            //
            // For additional context, UERST contains seven one-bit fields.
            // We set all of them, then clear all of them. The docs for
            // those fields says:
            //
            //     > Set to reset the selected endpoint FIFO prior to any other operation, upon hardware reset
            //     > or when an USB bus reset has been received. See “Endpoint Reset” on page 270 for more
            //     > information
            //     >
            //     > Then, clear by software to complete the reset operation and start using the endpoint.
            //
            // TODO: once again, patch PAC to avoid this unnecessary unsafe.
            usb.uerst().write(|w| unsafe { w.bits(0x7E) });
            usb.uerst().write(|w| unsafe { w.bits(0) });

            // This code is NOT ported from C++. TODO: cleanup this explanation of why.
            //
            // Tests from the `usb2` branch suggest that this is
            // necessary.  Why? It makes no sense. The docs seems to
            // suggest that I'm allowed to enable both at the same
            // time.
            //
            // If I *am* allowed to enable them both at the same time, then
            // this is absolutely pointless??
            usb.udint()
                .clear_interrupts(|w| w.wakeupi().clear_bit().suspi().clear_bit());
            usb.udien()
                .modify(|_, w| w.wakeupe().clear_bit().suspe().set_bit());
        })
    }

    /// Sets the device USB address to `addr`.
    fn set_device_address(&self, addr: u8) {
        interrupt::free(|cs| {
            let usb = self.usb.borrow(cs);

            // Quoting from section 22.7 of the datasheet:
            //
            // > The USB device address is set up according to the USB protocol:
            // > 1. the USB device, after power-up, responds at address 0
            // > 2. the host sends a SETUP command (SET_ADDRESS(addr))
            // > 3. the firmware handles this request, and records that address in UADD, but keep ADDEN cleared
            // > 4. the USB device firmware sends an IN command of 0 bytes (IN 0 Zero Length Packet)
            // > 5. then, the firmware can enable the USB device address by setting ADDEN. The only accepted address by the controller is the one stored in UADD.
            // >
            // > ADDEN and UADD shall not be written at the same time.
            //
            // This here is step three: "records that address in UADD, *but keep ADDEN cleared*" (emphasis added).
            usb.udaddr().modify(|_, w| unsafe { w.uadd().bits(addr) });

            // We skip ahead to step five: "enable the USB device address by setting ADDEN"
            usb.udaddr().modify(|_, w| w.adden().set_bit());

            // TODO: I'm guessing that step four is handled by `usb-device` after we return? Is this correct?
        });
    }

    /// Writes a single packet of data to the specified endpoint and returns number of bytes
    /// actually written.
    ///
    /// The only reason for a short write is if the caller passes a slice larger than the amount of
    /// memory allocated earlier, and this is generally an error in the class implementation.
    ///
    /// # Errors
    ///
    /// * [`InvalidEndpoint`](crate::UsbError::InvalidEndpoint) - The `ep_addr` does not point to a
    ///   valid endpoint that was previously allocated with [`UsbBus::alloc_ep`].
    /// * [`WouldBlock`](crate::UsbError::WouldBlock) - A previously written packet is still pending
    ///   to be sent.
    /// * [`BufferOverflow`](crate::UsbError::BufferOverflow) - The packet is too long to fit in the
    ///   transmission buffer. This is generally an error in the class implementation, because the
    ///   class shouldn't provide more data than the `max_packet_size` it specified when allocating
    ///   the endpoint.
    ///
    /// Implementations may also return other errors if applicable.
    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize, UsbError> {
        interrupt::free(|cs| {
            let index = ep_addr.index();
            let endpoint = self.get_endpoint_table_entry(cs, index)?;

            // We should only be writing to endpoints that are "IN" towards the host.
            assert_eq!(UsbDirection::In, ep_addr.direction());
            if index != 0 {
                // endpoint 0 is a special case; ref @FOOTNOTE-EP0
                assert_eq!(UsbDirection::In, endpoint.ep_dir);
            }

            let usb = self.usb.borrow(cs);
            self.set_current_endpoint(cs, ep_addr.index())?;

            if endpoint.ep_type == EndpointType::Control {
                if usb.ueintx().read().txini().bit_is_clear() {
                    return Err(UsbError::WouldBlock);
                }

                // Note: This check sometimes returns a buffer overflow error even when the
                // buffer is large enough to fit the data. This is intentional.
                //
                // During setup the user requested that we allocate `max_packet_size` bytes, but
                // the number of bytes that we actually allocated may be larger than that. The
                // `UsbBus` trait doesn't provide a way for the user to query how much memory
                // was actually allocated. As such, if they try to write more than `max_packet_size` bytes,
                // then there's probably a bug in the users code, and we should return an error even if the
                // buffer would not overflow.
                if buf.len() > endpoint.max_packet_size.into() {
                    return Err(UsbError::BufferOverflow);
                }

                for &byte in buf {
                    usb.uedatx().write(|w| unsafe { w.bits(byte) });
                }

                usb.ueintx().clear_interrupts(|w| w.txini().clear_bit());
            } else {
                if usb.ueintx().read().txini().bit_is_clear() {
                    return Err(UsbError::WouldBlock);
                }
                //NB: RXOUTI serves as KILLBK for IN endpoints and needs to stay zero:
                usb.ueintx()
                    .clear_interrupts(|w| w.txini().clear_bit().rxouti().clear_bit());

                for &byte in buf {
                    if usb.ueintx().read().rwal().bit_is_clear() {
                        return Err(UsbError::BufferOverflow);
                    }
                    usb.uedatx().write(|w| unsafe { w.bits(byte) });
                }

                //NB: RXOUTI serves as KILLBK for IN endpoints and needs to stay zero:
                usb.ueintx()
                    .clear_interrupts(|w| w.fifocon().clear_bit().rxouti().clear_bit());
            }

            let pending_ins = self.pending_ins.borrow(cs);
            pending_ins.set(pending_ins.get() | 1 << ep_addr.index());

            Ok(buf.len())
        })
    }

    /// Reads a single packet of data from the specified endpoint and returns the actual length of
    /// the packet.
    ///
    /// This should also clear any NAK flags and prepare the endpoint to receive the next packet.
    ///
    /// # Errors
    ///
    /// * [`InvalidEndpoint`](crate::UsbError::InvalidEndpoint) - The `ep_addr` does not point to a
    ///   valid endpoint that was previously allocated with [`UsbBus::alloc_ep`].
    /// * [`WouldBlock`](crate::UsbError::WouldBlock) - There is no packet to be read. Note that
    ///   this is different from a received zero-length packet, which is valid in USB. A zero-length
    ///   packet will return `Ok(0)`.
    /// * [`BufferOverflow`](crate::UsbError::BufferOverflow) - The received packet is too long to
    ///   fit in `buf`. This is generally an error in the class implementation, because the class
    ///   should use a buffer that is large enough for the `max_packet_size` it specified when
    ///   allocating the endpoint.
    ///
    /// Implementations may also return other errors if applicable.
    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        interrupt::free(|cs| {
            let index = ep_addr.index();
            let endpoint = self.get_endpoint_table_entry(cs, index)?;
            let usb = self.usb.borrow(cs);

            // We should only be reading if the data is flowing "OUT" from the host.
            assert_eq!(UsbDirection::Out, ep_addr.direction());
            if index != 0 {
                // endpoint 0 is a special case; ref @FOOTNOTE-EP0
                assert_eq!(UsbDirection::Out, endpoint.ep_dir);
            }

            self.set_current_endpoint(cs, index)?;

            if endpoint.ep_type == EndpointType::Control {
                let ueintx = usb.ueintx().read();
                if ueintx.rxouti().bit_is_clear() && ueintx.rxstpi().bit_is_clear() {
                    return Err(UsbError::WouldBlock);
                }

                let bytes_to_read = self.endpoint_byte_count(cs) as usize;
                if bytes_to_read > buf.len() {
                    return Err(UsbError::BufferOverflow);
                }

                for slot in &mut buf[..bytes_to_read] {
                    *slot = usb.uedatx().read().bits();
                }
                usb.ueintx()
                    .clear_interrupts(|w| w.rxouti().clear_bit().rxstpi().clear_bit());

                Ok(bytes_to_read)
            } else {
                if usb.ueintx().read().rxouti().bit_is_clear() {
                    return Err(UsbError::WouldBlock);
                }
                usb.ueintx().clear_interrupts(|w| w.rxouti().clear_bit());

                let mut bytes_read = 0;
                for slot in buf {
                    if usb.ueintx().read().rwal().bit_is_clear() {
                        break;
                    }
                    *slot = usb.uedatx().read().bits();
                    bytes_read += 1;
                }
                if usb.ueintx().read().rwal().bit_is_set() {
                    return Err(UsbError::BufferOverflow);
                }

                usb.ueintx().clear_interrupts(|w| w.fifocon().clear_bit());
                Ok(bytes_read)
            }
        })
    }

    /// Sets or clears the STALL condition for an endpoint. If the endpoint is an OUT endpoint, it
    /// should be prepared to receive data again.
    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        interrupt::free(|cs| {
            let usb = self.usb.borrow(cs);
            if self.set_current_endpoint(cs, ep_addr.index()).is_ok() {
                usb.ueconx()
                    .modify(|_, w| w.stallrq().bit(stalled).stallrqc().bit(!stalled));
            }
        });
    }

    /// Gets whether the STALL condition is set for an endpoint.
    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        interrupt::free(|cs| {
            let usb = self.usb.borrow(cs);
            if self.set_current_endpoint(cs, ep_addr.index()).is_ok() {
                usb.ueconx().read().stallrq().bit_is_set()
            } else {
                false
            }
        })
    }

    /// Causes the USB peripheral to enter USB suspend mode, lowering power consumption and
    /// preparing to detect a USB wakeup event. This will be called after
    /// [`poll`](crate::device::UsbDevice::poll) returns [`PollResult::Suspend`]. The device will
    /// continue be polled, and it shall return a value other than `Suspend` from `poll` when it no
    /// longer detects the suspend condition.
    fn suspend(&self) {
        interrupt::free(|cs| {
            let usb = self.usb.borrow(cs);
            usb.udint()
                .clear_interrupts(|w| w.suspi().clear_bit().wakeupi().clear_bit());
            usb.udien()
                .modify(|_, w| w.wakeupe().set_bit().suspe().clear_bit());
            usb.usbcon().modify(|_, w| w.frzclk().set_bit());
        });
    }

    /// Resumes from suspend mode. This may only be called after the peripheral has been previously
    /// suspended.
    fn resume(&self) {
        interrupt::free(|cs| {
            let usb = self.usb.borrow(cs);
            usb.usbcon().modify(|_, w| w.frzclk().clear_bit());
            usb.udint()
                .clear_interrupts(|w| w.wakeupi().clear_bit().suspi().clear_bit());
            usb.udien()
                .modify(|_, w| w.wakeupe().clear_bit().suspe().set_bit());
        });
    }

    /// Gets information about events and incoming data. Usually called in a loop or from an
    /// interrupt handler. See the [`PollResult`] struct for more information.
    fn poll(&self) -> PollResult {
        interrupt::free(|cs| {
            let usb = self.usb.borrow(cs);

            let usbint = usb.usbint().read();
            let udint = usb.udint().read();

            // TODO: This is sketchy?
            //
            // IMO it would be cleaner to always have both bits set
            // in `udien`. Then instead of storing state in `udien`,
            // just make an enum in `self`.
            let udien = usb.udien().read();

            if usbint.vbusti().bit_is_set() {
                // TODO: is this a race condtion?
                //
                // Suppose we start with everything in a good state, and VBUS is set.
                //
                // Then VBUS is cleared. However, before we can
                // return `Suspend`, VBUS is toggled again. We
                // then return `Resume` even though usb-device
                // is already in a `Resume` state.
                let vbus_is_set = usb.usbsta().read().vbus().bit_is_set();

                usb.usbint().clear_interrupts(|w| w.vbusti().clear_bit());

                // TODO: I don't understand; what does vbus have to do with Suspend?
                if vbus_is_set {
                    return PollResult::Resume;
                } else {
                    return PollResult::Suspend;
                }
            }
            if udint.suspi().bit_is_set() && udien.suspe().bit_is_set() {
                return PollResult::Suspend;
            }
            if udint.wakeupi().bit_is_set() && udien.wakeupe().bit_is_set() {
                return PollResult::Resume;
            }
            if udint.eorsti().bit_is_set() {
                return PollResult::Reset;
            }
            if udint.sofi().bit_is_set() {
                usb.udint().clear_interrupts(|w| w.sofi().clear_bit());
            }

            // Can only query endpoints while clock is running
            // (e.g. not in suspend state)
            if usb.usbcon().read().frzclk().bit_is_clear() {
                let mut ep_out = 0u8;
                let mut ep_setup = 0u8;
                let mut ep_in_complete = 0u8;
                let pending_ins = self.pending_ins.borrow(cs);

                for (index, _ep) in self.active_endpoints() {
                    if self.set_current_endpoint(cs, index).is_err() {
                        // Endpoint selection has stopped working...
                        break;
                    }

                    let ueintx = usb.ueintx().read();
                    if ueintx.rxouti().bit_is_set() {
                        ep_out |= 1 << index;
                    }
                    if ueintx.rxstpi().bit_is_set() {
                        ep_setup |= 1 << index;
                    }
                    if pending_ins.get() & (1 << index) != 0 && ueintx.txini().bit_is_set() {
                        ep_in_complete |= 1 << index;
                        pending_ins.set(pending_ins.get() & !(1 << index));
                    }
                }
                if ep_out | ep_setup | ep_in_complete != 0 {
                    return PollResult::Data {
                        ep_out: ep_out as u16,
                        ep_in_complete: ep_in_complete as u16,
                        ep_setup: ep_setup as u16,
                    };
                }
            }

            PollResult::None
        })
    }

    /// Simulates a disconnect from the USB bus, causing the host to reset and re-enumerate the
    /// device.
    ///
    /// The default implementation just returns `Unsupported`.
    ///
    /// # Errors
    ///
    /// * [`Unsupported`](crate::UsbError::Unsupported) - This UsbBus implementation doesn't support
    ///   simulating a disconnect or it has not been enabled at creation time.
    fn force_reset(&self) -> Result<(), UsbError> {
        interrupt::free(|cs| {
            self.usb
                .borrow(cs)
                .udcon()
                .modify(|_, w| w.detach().set_bit());
        });

        let mut delay = Delay::<CLOCKUSB>::new();
        delay.delay_ms(10);

        interrupt::free(|cs| {
            self.usb
                .borrow(cs)
                .udcon()
                .modify(|_, w| w.detach().clear_bit());
        });

        Ok(())
    }
}

/// Extension trait for conveniently clearing AVR interrupt flag registers.
trait ClearInterrupts {
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
        self.write(|w| f(unsafe { w.bits(0x7d) }));
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
        self.write(|w| f(unsafe { w.bits(0xdf) }));
    }
}

impl ClearInterrupts for USBINT {
    type Writer = usbint::W;

    fn clear_interrupts<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&mut Self::Writer) -> &mut Self::Writer,
    {
        // Bits 7:1 are reserved as do not set.
        self.write(|w| f(unsafe { w.bits(0x01) }));
    }
}

// TODO: add a drop implementation.
// Either have it panic, or actually test it.
