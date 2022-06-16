use crate::clock::Clock as CpuClock;
use avr_device::interrupt::CriticalSection;
use avr_hal_generic::time::Prescaler;
use avr_hal_generic::time::TimingCircuitOps;
use core::cell::Cell;
use core::ops;

/// Represents a timer that has a proper timer interrupt handler assigned.
///
/// Please use `TimerClock` wrapper instead of this trait.
/// Also, do not implement this trait yourself, instead use the `todo` macro.
///
/// # Safety
///
/// If value of this type exists, an interrupt handler must be installed,
/// that handles the OCR interrupts of indicated `Timer` peripheral.
///
/// Also, at most a single value of the underling type must exist at any given
/// point in time.
pub unsafe trait Timepiece<H> {
    /// The use timer peripheral
    type Circuit: TimingCircuitOps<H>;
    /// The CPU clock speed to which this timer is tuned for
    type CpuClock: CpuClock;
    /// The type of the software milliseconds counter
    type Millis: Copy + ops::Add<Output = Self::Millis> + From<u8>;
    /// The typo of microsecond output
    type Micros: Copy
        + From<u32>
        + From<Self::Millis>
        + From<<Self::Circuit as TimingCircuitOps<H>>::Counter>
        + ops::Add<Output = Self::Micros>
        + ops::Mul<Output = Self::Micros>
        + ops::Div<Output = Self::Micros>;
    /// The resolution of the software milliseconds counter
    const RESOLUTION: Self::Millis;
    /// The timer parameters to achieve the state resolution
    const TIMER_PARAMS: (Prescaler, <Self::Circuit as TimingCircuitOps<H>>::Counter);

    /// Reads the current value of the software milliseconds counter
    fn access_millis(cs: &CriticalSection) -> &Cell<Self::Millis>;

    /// Gives access to the underling timer peripheral
    fn access_peripheral(&self) -> &Self::Circuit;
}

/// Increment the timer value of `Pt`
///
/// This function should only be used by a timer interrupt.
#[doc(hidden)]
pub fn update_timer<H, Tp: Timepiece<H>>(cs: &CriticalSection) {
    let counter_cell = Tp::access_millis(cs);
    let counter = counter_cell.get();

    let interrupt_duration = Tp::RESOLUTION;

    // TODO: use wrapping arithmetics
    counter_cell.set(counter + interrupt_duration);
}
