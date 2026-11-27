#![no_std]
#![no_main]
use arduino_hal::prelude::*;
use arduino_hal::Peripherals;
use panic_halt as _;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::StringDescriptors;
use usb_device::device::UsbDeviceBuilder;
use usb_device::device::UsbVidPid;
use usb_device::LangID;
use usbd_serial::SerialPort;

#[arduino_hal::entry]
fn main() -> ! {
    let dp: Peripherals = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial_hw = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial_hw, "Hello from Arduino!").unwrap_infallible();

    let usb_bus = arduino_hal::default_usb_bus_with_pll_macro!(dp);
    let usb_bus_allocator = UsbBusAllocator::new(usb_bus);

    let mut serial_usb = SerialPort::new(&usb_bus_allocator);

    let string_descriptors = StringDescriptors::new(LangID::EN_US)
        .manufacturer("test manufacturer")
        .product("test product")
        .serial_number("test serial number");

    let rand_ids = UsbVidPid(0x1ea7, 0x4a09);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus_allocator, rand_ids)
        .strings(&[string_descriptors])
        .unwrap()
        .max_packet_size_0(64)
        .unwrap()
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    // TODO: only in dev builds
    usb_dev.force_reset().unwrap();

    loop {
        // Wait until we have data
        if !usb_dev.poll(&mut [&mut serial_usb]) {
            continue;
        }

        // Read the data into this buffer
        let mut read_buf = [0u8; 10];
        let Ok(read_count) = serial_usb.read(&mut read_buf) else {
            continue;
        };
        if read_count == 0 {
            continue;
        }

        // Ideally we want to do something like this:
        //
        // ```
        // let mut write_buf = [0u8; 20];
        // let write_count = ufmt::uwriteln!(&mut write_buf, "Got: {}", &write_buf);
        // ```
        //
        // TODO: Figure out how to get the above code to compile. It seems like
        // I might need to manually implement the uDebug trait? That doesn't seem
        // right... In the meantime, simply echo the string back

        // TODO: is this `.expect()` safe?
        let len = serial_usb
            .write(&read_buf[0..read_count])
            .expect("The host should be reading data faster than the arduino can write it");
        assert_eq!(len, read_count);
    }
}
