//! Timing Circuits
//!
//! This module contains low-level hardware abstractions to be used for
//! time-keeping.

/// A hardware component that can generate regular interrupts.
///
/// A timing circuit is basically a counter that starts at `0` and counts up
/// in fixed (appox.) wall time intervals.
/// Once the counter reaches a defined value it resets to zero and triggers
/// some interrupt.
/// The counting interval can be configured via a [`Prescaler`] from the
/// MCUs CPU clock frequency.
//
// TODO: shall we really add the `Ops` suffix?
pub trait TimingCircuitOps<H> {
    /// The internal type of the counter register (typically `u8` or `u16`)
    type Counter: Copy + core::cmp::Ord;

    /// Returns the current value of the counter register and whether an
    /// interrupt is scheduled to be triggered.
    ///
    /// The reason behind the interrupt flag is, that the once the counter
    /// reaches the predetermined value, it will reset to zero, however,
    /// if the interrupt handler might be currently blocked from running,
    /// and thus executing its update function.
    /// Thus this interrupt flag can be seen as an additional counter bit,
    /// indicating that the counter value (might) have been reset to zero,
    /// without the interrupt been executed yet.
    fn read_counter(&self) -> (Self::Counter, bool);

    /// Configure the timer with the given prescaler and top value.
    ///
    /// This also starts the timer.
    fn initialize(&self, p: Prescaler, top: Self::Counter);

    /// Enable the timer interrupt
    ///
    /// # Safety
    /// A timer interrupt handler for this timer must have been set up.
    //
    // TODO: Would it make sens if this were safe? What if no handler is set?
    //
    // TODO: maybe we call it just `enable` and also kick-off
    //       the timer here (instead of doing it in `initialize`)
    unsafe fn enable_interrupt(&self);

    /// Disable this timer and disable any interrupts from it
    fn disable(&self);
}

/// Represents one of the few valid prescaler values for [TimingCircuitOps].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prescaler {
    P1,
    P8,
    P64,
    P256,
    P1024,
}
impl Prescaler {
    /// Returns the next best prescaler for the given prescaler exponent.
    ///
    /// The next best prescaler means here, the next bigger value, unless,
    /// the value goes beyond 10, which is the highest supported prescaler
    /// exponent.
    pub const fn from_exp(exp: u32) -> Option<Self> {
        let prescaler = match exp {
            0 => Self::P1,
            1..=3 => Self::P8,
            4..=6 => Self::P64,
            7..=8 => Self::P256,
            9..=10 => Self::P1024,
            _ => return None,
        };
        Some(prescaler)
    }

    /// Gives the exponent of this prescaler.
    pub const fn to_exp(self) -> u8 {
        match self {
            Self::P1 => 0,
            Self::P8 => 3,
            Self::P64 => 6,
            Self::P256 => 8,
            Self::P1024 => 10,
        }
    }

    /// Returns the numeric value of this prescaler.
    pub const fn to_val(self) -> u16 {
        1 << self.to_exp()
    }
}

/// Represents the interrupt firing interval of a [TimingCircuitOps] in
/// milliseconds.
///
/// When used as in a chronometer, this defines the precision of the
/// chronometer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Resolution {
    millis: u32,
}

impl Resolution {
    pub const MS_1: Self = Self::from_ms(1);
    pub const MS_2: Self = Self::from_ms(2);
    pub const MS_4: Self = Self::from_ms(4);
    pub const MS_8: Self = Self::from_ms(8);
    pub const MS_16: Self = Self::from_ms(16);

    /// A resolution of `ms` milliseconds
    pub const fn from_ms(ms: u32) -> Self {
        Self { millis: ms }
    }

    /// A resolution of `2^exp` milliseconds
    pub const fn from_exp(exp: u8) -> Self {
        Self { millis: 1 << exp }
    }

    /// The interval in milliseconds
    pub const fn as_ms(self) -> u32 {
        self.millis
    }

    /// Calculates the optimal prescaler and counter value for the given clock
    /// frequency in Hz, and maximum supported counter value.
    ///
    /// Returns `None`, if there there is no valid configuration for this
    /// resolution at the given frequency.
    pub const fn params_for_frq(self, freq_hz: u32, cnt_max: u32) -> Option<(Prescaler, u32)> {
        // The maximum valid counter value
        // TODO: use a generic parameter instead of `cnt_max`, however,
        //       this would need several unstable features.
        //const MAX: u32 = u8::MAX as u32; // 255

        let cycles_per_second = freq_hz;
        // Combine for better precision:
        //     let cycles_per_ms = (cycles_per_second + 499) / 1_000;
        //     let cycles_per_interrupt = cycles_per_ms * self.as_ms();
        let cycles_per_interrupt = (cycles_per_second * self.as_ms() + 499) / 1_000; // rounded

        // Calculate a perfect prescaler.
        // It is also the minimum prescaler, because it yield the highest
        // yet valid counter value.
        // So, if need to tweak the prescaler, we need to make it bigger.
        // Thus, we already calculate this rounded up
        let perfect_prescaler: u32 = (cycles_per_interrupt + cnt_max - 1) / cnt_max;

        // Calculate the log2 of `perfect_prescaler`, rounded up
        // To get the correct result for powers of two, we will subtract 1
        // if we have a power of two. Power of two have exactly one `1` in
        // binary.
        let sub_for_pot = if perfect_prescaler.count_ones() == 1 {
            1
        } else {
            0
        };
        let perfect_prescaler_exp = u32::BITS - perfect_prescaler.leading_zeros() - sub_for_pot;

        // Get the next best (i.e. exact or bigger) available prescaler, if any
        let prescaler = match Prescaler::from_exp(perfect_prescaler_exp) {
            Some(p) => p,
            None => return None,
        };

        // The scalar value of the available perscaler
        let prescaler_val: u16 = prescaler.to_val();

        // Calculate the number of prescaled cycles per interrupt
        let cnt = (cycles_per_interrupt + (prescaler_val / 2) as u32) / (prescaler_val as u32); // rounded

        // If we calculated correctly, it holds: `cnt <= MAX`
        assert!(cnt < cnt_max);

        Some((prescaler, cnt))
    }
}
