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


/// Represents one of the few valid prescaler values for [TimingCircuitOps]s.
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
