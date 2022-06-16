use crate::clock::Clock as CpuClock;
use crate::time::Timepiece;
use avr_hal_generic::time::TimingCircuitOps;
use embedded_time::clock::Error as ClockError;
use embedded_time::rate::Fraction;
use embedded_time::Instant;

/// A statically accessible clock
///
/// Notice, if the clock is stopped or not yet initialized, it will just
/// return the last value, and the clock will appear frozen.
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""), Copy(bound = ""), Clone(bound = ""))]
pub struct StaticChronometer<H, Tp> {
    _pre_timer: core::marker::PhantomData<*const Tp>,
    _h: core::marker::PhantomData<*const H>,
}
unsafe impl<H, Tp> Send for StaticChronometer<H, Tp> {
    // SAFETY: We neither store a `Tp` nor `H`
}
unsafe impl<H, Tp> Sync for StaticChronometer<H, Tp> {
    // SAFETY: We neither store a `Tp` nor `H`
}
impl<H, Tp> StaticChronometer<H, Tp> {
    pub const fn new() -> Self {
        Self {
            _pre_timer: core::marker::PhantomData,
            _h: core::marker::PhantomData,
        }
    }
}
impl<H, Tp: Timepiece<H>> StaticChronometer<H, Tp> {
    /// Returns the number of milliseconds since this clock was started
    ///
    /// Notice, if the clock is stopped or not yet initialized, it will just
    /// return the last value, and the clock will appear frozen.
    pub fn millis(&self) -> Tp::Millis {
        // Get the current number of milliseconds
        avr_device::interrupt::free(|cs| Tp::access_millis(cs).get())
    }
}
impl<H, Tp: Timepiece<H>> embedded_time::clock::Clock for StaticChronometer<H, Tp>
where
    <Tp as Timepiece<H>>::Millis: ufmt::uDebug + core::hash::Hash, // + embedded_time::TimeInt // Not accessible in 0.10
    Tp: Timepiece<H, Millis = u32>, // Just workaround until above gets accessible
{
    type T = Tp::Millis;

    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);

    #[inline(always)]
    fn try_now(&self) -> Result<Instant<Self>, ClockError> {
        Ok(self.now())
    }
}
impl<H, Tp: Timepiece<H>> StaticChronometer<H, Tp>
where
    <Tp as Timepiece<H>>::Millis: ufmt::uDebug + core::hash::Hash, // + embedded_time::TimeInt // Not accessible in 0.10
    Tp: Timepiece<H, Millis = u32>, // Just workaround until above gets accessible
{
    pub fn now(&self) -> Instant<Self> {
        avr_device::interrupt::free(|cs| self.now_with_cs(cs))
    }

    pub fn now_with_cs(&self, cs: &avr_device::interrupt::CriticalSection) -> Instant<Self> {
        // TODO make use of `cs`
        Instant::new(Self::new().millis())
    }
}

/// A Timer-based Clock, tells an approximated wall time.
///
/// This type is a defacto singleton that can only once instantiated,
/// see [`new`](Self::new).
///
/// You must create an instance of this clock to make the clock work.
/// However, if you leak or drop the clock without call [`stop`],
/// it will keep running.
#[derive(Debug)]
//#[derive(ufmt::derive::uDebug)]
pub struct Chronometer<H, Tp> {
    inner: Tp,
    _h: core::marker::PhantomData<H>,
}

impl<H, Tp: Timepiece<H>> Chronometer<H, Tp> {
    /// Initialize the clock.
    ///
    /// The clock start running immediately after this call.
    /// However, in order to work properly interrupts must be enabled too, e.g.:
    ///
    /// ```rust
    /// // Enable interrupts globally
    /// unsafe { avr_device::interrupt::enable() };
    /// ```
    pub fn new(timepiece: Tp) -> Self {
        let (prescaler, timer_cnt) = Tp::TIMER_PARAMS;

        // Configure the timer for the above interval
        let tc = timepiece.access_peripheral();

        // First, disable the timer, in case it was in use
        tc.disable();

        // Reset the global millisecond counter
        avr_device::interrupt::free(|cs| {
            Tp::access_millis(cs).set(0.into());
        });

        // Enable timer interrupt
        unsafe {
            // SAFETY: we have a `Tp: Timepiece`, which guarantees us that a
            // corresponding interrupt handler has been installed.
            tc.set_interrupt_enable();
        }

        // Configure timer & start it
        tc.enable(prescaler, timer_cnt);

        // We are done here
        Self {
            inner: timepiece,
            _h: core::marker::PhantomData,
        }
    }

    /// Stops the clock and returns back the used timer
    pub fn stop(self) -> Tp {
        self.inner.access_peripheral().disable();

        self.inner
    }

    /// Reset the millis counter to zero, and return the old value
    pub fn reset_time(&self) {
        avr_device::interrupt::free(|cs| Tp::access_millis(cs).replace(0.into()));
    }

    /// Returns the number of milliseconds since this clock was started
    pub fn millis(&self) -> Tp::Millis {
        // Get the current number of milliseconds
        avr_device::interrupt::free(|cs| Tp::access_millis(cs).get())
    }

    /// Returns the number of microseconds since this clock was started
    pub fn micros(&self) -> Tp::Micros {
        let (mut m, t, tifr) = avr_device::interrupt::free(|cs| {
            let m = Tp::access_millis(cs).get();

            let tc = &self.inner.access_peripheral();
            let (t, tifr) = tc.read_counter();

            (m, t, tifr)
        });

        // Caluclate the duration of timer tick in microseconds
        let interrupt_duration_us = Tp::Micros::from(1_000_000u32)
            * Tp::Micros::from(u32::from(Tp::TIMER_PARAMS.0.to_val()))
            / Tp::Micros::from(<Tp::CpuClock as CpuClock>::FREQ);

        // TODO: use wrapping arithmetics
        let counter_micros = Tp::Micros::from(t) * interrupt_duration_us;

        // Check whether a interrupt was pending when we read the counter value,
        // which typically means it wrapped around, without the millis getting
        // incremented, so we do it here manually:
        let max_counter_value = Tp::TIMER_PARAMS.1;
        if tifr && t < max_counter_value {
            // TODO: use wrapping arithmetics
            m = m + Tp::Millis::from(1);
        }

        let millis = Tp::Micros::from(m);

        // TODO: use wrapping arithmetics
        millis * Tp::Micros::from(1000) + counter_micros
    }
}

impl<H, Tp: Timepiece<H>> embedded_time::clock::Clock for Chronometer<H, Tp>
where
    <Tp as Timepiece<H>>::Micros: ufmt::uDebug + core::hash::Hash, // + embedded_time::TimeInt // Not accessible in 0.10
    Tp: Timepiece<H, Micros = u32>, // Just workaround until above gets accessible
{
    type T = Tp::Micros;

    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000);

    #[inline(always)]
    fn try_now(&self) -> Result<Instant<Self>, ClockError> {
        Ok(self.now())
    }
}
impl<H, Tp: Timepiece<H>> Chronometer<H, Tp>
where
    <Tp as Timepiece<H>>::Micros: ufmt::uDebug + core::hash::Hash, // + embedded_time::TimeInt // Not accessible in 0.10
    Tp: Timepiece<H, Micros = u32>, // Just workaround until above gets accessible
{
    pub fn now(&self) -> Instant<Self> {
        avr_device::interrupt::free(|cs| self.now_with_cs(cs))
    }

    pub fn now_with_cs(&self, cs: &avr_device::interrupt::CriticalSection) -> Instant<Self> {
        // TODO make use of `cs`
        Instant::new(self.micros())
    }
}
