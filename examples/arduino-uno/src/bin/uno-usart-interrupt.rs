/*!
 * A more robust serial example modeled after what the Arduino C++ library does.
 * Enables sending and recieving via buffers and interrupts.
 */
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

// TODO: This should really use heapless::spsc::Queue or heapless::Deque.
// The update requires updating atomic_polyfill to support AVR.
// Which requires asm! to work correctly with avr.
use arduino_uno_examples::spsc::{Consumer, Producer, Queue};
use avr_device::interrupt::Mutex;
use core::cell::RefCell;
use core::ops::DerefMut;
use embedded_hal::serial::{Read, Write};

static SERIAL: Mutex<
    RefCell<
        Option<
            arduino_hal::Usart<
                arduino_hal::pac::USART0,
                arduino_hal::port::Pin<arduino_hal::port::mode::Input, arduino_hal::hal::port::PD0>,
                arduino_hal::port::Pin<
                    arduino_hal::port::mode::Output,
                    arduino_hal::hal::port::PD1,
                >,
            >,
        >,
    >,
> = Mutex::new(RefCell::new(None));

static mut RX_QUEUE: Queue<u8, 64> = Queue::new();
static mut TX_QUEUE: Queue<u8, 64> = Queue::new();
static RX_PRODUCER: Mutex<RefCell<Option<Producer<u8, 64>>>> = Mutex::new(RefCell::new(None));
static RX_CONSUMER: Mutex<RefCell<Option<Consumer<u8, 64>>>> = Mutex::new(RefCell::new(None));
static TX_PRODUCER: Mutex<RefCell<Option<Producer<u8, 64>>>> = Mutex::new(RefCell::new(None));
static TX_CONSUMER: Mutex<RefCell<Option<Consumer<u8, 64>>>> = Mutex::new(RefCell::new(None));

// Read into buffer via interupt.
#[avr_device::interrupt(atmega328p)]
unsafe fn USART_RX() {
    avr_device::interrupt::free(|cs| {
        if let Some(ref mut serial) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
            if let Ok(b) = serial.read() {
                if let Some(ref mut rx_producer) = RX_PRODUCER.borrow(cs).borrow_mut().deref_mut() {
                    // Ignore data if the buffer is full.
                    let _ = rx_producer.enqueue(b);
                }
            }
        };
    });
}

// Write to serial from buffer.
#[avr_device::interrupt(atmega328p)]
unsafe fn USART_UDRE() {
    avr_device::interrupt::free(|cs| {
        if let Some(ref mut serial) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
            if let Some(ref mut tx_consumer) = TX_CONSUMER.borrow(cs).borrow_mut().deref_mut() {
                if let Some(b) = tx_consumer.dequeue() {
                    let _ = serial.write(b);
                } else {
                    // Clear interrupt if we are out of data.
                    serial.unlisten(arduino_hal::hal::usart::Event::DataRegisterEmpty);
                }
            }
        }
    });
}

// This is a safe wrapper around the serial that avoids locking up the interrupts for too long.
// It also enables use with ufmt.
struct Serial {}

impl Serial {
    fn read(&mut self) -> Option<u8> {
        avr_device::interrupt::free(|cs| {
            if let Some(ref mut rx_consumer) = RX_CONSUMER.borrow(cs).borrow_mut().deref_mut() {
                rx_consumer.dequeue()
            } else {
                None
            }
        })
    }
}

impl ufmt::uWrite for Serial {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for b in s.as_bytes() {
            // Poll until the queue has free space.
            // Enable interrupts between each loop to ensure the CPU isn't hogged.
            loop {
                let ready = avr_device::interrupt::free(|cs| {
                    if let Some(ref mut tx_producer) =
                        TX_PRODUCER.borrow(cs).borrow_mut().deref_mut()
                    {
                        tx_producer.ready()
                    } else {
                        false
                    }
                });
                if ready {
                    break;
                }
            }
            // Write byte to the queue.
            // Also, enable the serial data empty interrupt to start the send.
            avr_device::interrupt::free(|cs| {
                if let Some(ref mut tx_producer) = TX_PRODUCER.borrow(cs).borrow_mut().deref_mut() {
                    let _ = tx_producer.enqueue(*b);
                }
                if let Some(ref mut serial) = SERIAL.borrow(cs).borrow_mut().deref_mut() {
                    serial.listen(arduino_hal::hal::usart::Event::DataRegisterEmpty);
                }
            });
        }
        Ok(())
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    avr_device::interrupt::free(|cs| {
        serial.listen(arduino_hal::hal::usart::Event::RxComplete);
        SERIAL.borrow(cs).replace(Some(serial));
        // This is the only place the queues are ever directly used.
        // They must be mutable statics for lifetime reasons.
        let (rx_producer, rx_consumer) = unsafe { RX_QUEUE.split() };
        let (tx_producer, tx_consumer) = unsafe { TX_QUEUE.split() };
        RX_PRODUCER.borrow(cs).replace(Some(rx_producer));
        RX_CONSUMER.borrow(cs).replace(Some(rx_consumer));
        TX_CONSUMER.borrow(cs).replace(Some(tx_consumer));
        TX_PRODUCER.borrow(cs).replace(Some(tx_producer));
    });
    unsafe { avr_device::interrupt::enable() };

    let mut serial = Serial {};
    loop {
        // Read a byte from the serial connection
        if let Some(b) = serial.read() {
            // Answer
            ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap();
        }
    }
}
