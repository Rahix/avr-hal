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

    /// Configure the timer and enables it
    ///
    /// This will set the given prescaler and top value on this timer,
    /// and then start it.
    ///
    /// Should be call be called before [`enable`]
    fn enable(&self, p: Prescaler, top: Self::Counter);

    /// Unmasks the timer interrupt.
    ///
    /// Should be called between `disable` and `enable`.
    ///
    /// # Safety
    /// A timer interrupt handler for this timer must have been set up.
    //
    // TODO: Would it make sens if this were safe? What if no handler is set?
    unsafe fn set_interrupt_enable(&self);

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

/// Implements [TimingCircuitOps] for acc `avr_device::$chip::[<TC $n>]` timers
/// using `[<OCR $n a>]` comparator and interrupt
#[macro_export]
macro_rules! impl_timer_circuit_via_TCn_OCRnA {
    // NOTICE: because of the inner macro, we must not define `$body`!
    (
        hal: $HAL:ident,
        // `chip` should be `ident`, but Rust 1.51 does not accept it in
        // `#[$crate::avr_device::interrupt($chip)]`
        chip: $chip:tt,
        // List of 8-bit timer number as in `TCn`
        timers_8_bit: [ $( $n8:expr ),* $(,)? ] $(,)?
        // List of 16-bit timer number as in `TCn`
        timers_16_bit: [ $( $n16:expr ),* $(,)? ] $(,)?

    ) => {
        $crate::paste::paste! {
            /// Define the interrupt handler for the interrupt vector for the given
            /// Timer peripheral
            #[macro_export]
            macro_rules! attach_timing_circuit_interrupt {
                $( /* for each $n8 */
                    // Rule matching timer $n
                    ([<Timer $n8>]; $body:block) => {
                        // The timer interrupt service routine
                        #[$crate::avr_device::interrupt($chip)]
                        fn [<TIMER $n8 _COMPA>]() $body
                    };
                )*

                $( /* for each $n16 */
                    // Rule matching timer $n
                    ([<Timer $n16>]; $body:block) => {
                        // The timer interrupt service routine
                        #[$crate::avr_device::interrupt($chip)]
                        fn [<TIMER $n16 _COMPA>]() $body
                    };
                )*

                // Final rule reporting invalid timers
                ($name:ident; $body:block) => {
                    compile_error!(concat!(
                        "Your selected platform (",
                        stringify!($chip),
                        ") does not have a compatible timer named: ",
                        stringify!($name),
                        "\nSee arduino_hal::time::timers for a list of supported timers"
                    ));
                }
            }

            $( /* for each $n8 */

                /// Ensures that the mentioned interrupt exists for the given chip
                #[doc(hidden)]
                const _: $crate::avr_device::$chip::Interrupt = $crate::avr_device::$chip::Interrupt::[<TIMER $n8 _COMPA>];

                // public reexport,
                // will be re-reexported in `arduino_hal::time::timers`
                pub use $crate::avr_device::$chip::[<TC $n8>] as [<Timer $n8>];

                #[doc = "Make [`Timer" $n8 "`] eligible for time-keeping."]
                #[doc = ""]
                #[doc = "This implementation uses the `OCR" $n8 "A` comparator in CTC mode"]
                #[doc = "and thus attaches to `TIMER" $n8 "_COMPA` interrupt."]
                impl TimingCircuitOps<$HAL> for [<Timer $n8>] {

                    type Counter =
                        <
                            $crate::avr_device::$chip::[<tc $n8>]::[<tcnt $n8>]::[<TCNT $n8 _SPEC>]
                            as $crate::avr_device::generic::RegisterSpec
                        >::Ux;

                    fn read_counter(&self) -> (Self::Counter, bool) {
                        (
                            self
                                .[<tcnt $n8 >]
                                .read()
                                .bits(),
                            self
                                .[<tifr $n8>]
                                .read()
                                .[<ocf $n8 a>]()
                                .bit(),
                        )
                    }

                    unsafe fn set_interrupt_enable(&self) {
                        // Enable the interrupt by setting
                        // the Output Compare match A Interrupt Enable bit.
                        self.[<timsk $n8>].modify(|_,w| w.[<ocie $n8 a>]().set_bit());
                    }

                    fn enable(&self, p: $crate::time::Prescaler, top: Self::Counter) {
                        // Set 'clear timer on compare' (CTC) mode using comparator A
                        // Notice the CTC mode on a 8-bit timer is `0b010`.
                        // Those three bits are split between
                        // `TCCRxB::WGMx2` (top bit) and `TCCRxA::WGMx` (lower two bits)
                        // Also see: https://github.com/Rahix/avr-device/issues/96

                        // Clearing the top bit
                        self.[<tccr $n8 b>].modify(|_,w| w.[<wgm $n8 2>]().clear_bit());

                        // Setting the lower two bits
                        self.[<tccr $n8 a>].modify(|_,w| w.[<wgm $n8>]().ctc());

                        // Set top value for comparator A
                        self.[<ocr $n8 a>]
                            .modify(|_,w| unsafe {
                                // TODO: Why is this unsafe???
                                // TODO: Safety, is this sound?
                                w.bits(top)
                            });

                        // Set prescaler and thereby start the timer
                        self.[<tccr $n8 b>]
                            .modify(|_,w| match p {
                                Prescaler::P1 => w.[<cs $n8>]().direct(),
                                Prescaler::P8 => w.[<cs $n8>]().prescale_8(),
                                Prescaler::P64 => w.[<cs $n8>]().prescale_64(),
                                Prescaler::P256 => w.[<cs $n8>]().prescale_256(),
                                Prescaler::P1024 => w.[<cs $n8>]().prescale_1024(),
                            });
                    }

                    /// Disable this clock and disable any interrupts from it
                    fn disable(&self) {
                        // Stop clock
                        self.[<tccr $n8 b>].modify(|_,w| w.[<cs $n8>]().no_clock());
                        // Disable all interrupts
                        self.[<timsk $n8 >].modify(|_,w| {
                            w.[<ocie $n8 a>]()
                                .clear_bit() //
                                .[<ocie $n8 b>]()
                                .clear_bit() //
                                .[<toie $n8>]()
                                .clear_bit() //
                        });
                    }
                }
            )* /* repeats $n8 */

            $( /* for each $n16 */

                /// Ensures that the mentioned interrupt exists for the given chip
                #[doc(hidden)]
                const _: $crate::avr_device::$chip::Interrupt = $crate::avr_device::$chip::Interrupt::[<TIMER $n16 _COMPA>];

                // public reexport,
                // will be re-reexported in `arduino_hal::time::timers`
                pub use $crate::avr_device::$chip::[<TC $n16>] as [<Timer $n16>];

                #[doc = "Make [`Timer" $n16 "`] eligible for time-keeping."]
                #[doc = ""]
                #[doc = "This implementation uses the `OCR" $n16 "A` comparator in CTC mode"]
                #[doc = "and thus attaches to `TIMER" $n16 "_COMPA` interrupt."]
                impl TimingCircuitOps<$HAL> for [<Timer $n16>] {

                    type Counter =
                        <
                            $crate::avr_device::$chip::[<tc $n16>]::[<tcnt $n16>]::[<TCNT $n16 _SPEC>]
                            as $crate::avr_device::generic::RegisterSpec
                        >::Ux;

                    fn read_counter(&self) -> (Self::Counter, bool) {
                        (
                            self
                                .[<tcnt $n16 >]
                                .read()
                                .bits(),
                            self
                                .[<tifr $n16>]
                                .read()
                                .[<ocf $n16 a>]()
                                .bit(),
                        )
                    }

                    unsafe fn set_interrupt_enable(&self) {
                        // Enable the interrupt by setting
                        // the Output Compare match A Interrupt Enable bit.
                        self.[<timsk $n16>].modify(|_,w| w.[<ocie $n16 a>]().set_bit());
                    }

                    fn enable(&self, p: $crate::time::Prescaler, top: Self::Counter) {
                        // Set 'clear timer on compare' (CTC) mode using comparator A
                        // Notice the CTC mode on a 16-bit timer is `0b0100`.
                        // Those four bits are split between
                        // `TCCRxB::WGMx` (top two bits) and `TCCRxA::WGMx` (lower two bits)
                        // Also see: https://github.com/Rahix/avr-device/issues/96

                        // Setting the top two bits
                        self.[<tccr $n16 b>].modify(|_,w| w.[<wgm $n16>]().bits(0b01));

                        // Setting the lower two bits
                        self.[<tccr $n16 a>].modify(|_,w| w.[<wgm $n16>]().bits(0b00));

                        // Set top value for comparator A
                        self.[<ocr $n16 a>]
                            .modify(|_,w| unsafe {
                                // TODO: Why is this unsafe???
                                // TODO: Safety, is this sound?
                                w.bits(top)
                            });

                        // Set prescaler and thereby start the timer
                        self.[<tccr $n16 b>]
                            .modify(|_,w| match p {
                                Prescaler::P1 => w.[<cs $n16>]().direct(),
                                Prescaler::P8 => w.[<cs $n16>]().prescale_8(),
                                Prescaler::P64 => w.[<cs $n16>]().prescale_64(),
                                Prescaler::P256 => w.[<cs $n16>]().prescale_256(),
                                Prescaler::P1024 => w.[<cs $n16>]().prescale_1024(),
                            });
                    }

                    /// Disable this clock and disable any interrupts from it
                    fn disable(&self) {
                        // Stop clock
                        self.[<tccr $n16 b>].modify(|_,w| w.[<cs $n16>]().no_clock());
                        // Disable all interrupts
                        self.[<timsk $n16 >].modify(|_,w| {
                            w.[<ocie $n16 a>]()
                                .clear_bit() //
                                .[<ocie $n16 b>]()
                                .clear_bit() //
                                .[<toie $n16>]()
                                .clear_bit() //
                        });
                    }
                }
            )* /* repeats $n16 */
        }
    };
}
