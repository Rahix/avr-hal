//! WDT Implementation
use core::marker::PhantomData;

/// Watchdog Timeout
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Timeout {
    /// 16 milliseconds
    Ms16,
    /// 32 milliseconds
    Ms32,
    /// 64 milliseconds
    Ms64,
    /// 125 milliseconds
    Ms125,
    /// 250 milliseconds
    Ms250,
    /// 500 milliseconds
    Ms500,
    /// 1 second
    Ms1000,
    /// 2 seconds
    Ms2000,
    /// 4 seconds
    Ms4000,
    /// 8 seconds
    Ms8000,
}

/// Internal trait for low-level watchdog operations.
///
/// **HAL users should use the [`Wdt`] type instead.**
pub trait WdtOps<H> {
    type MCUSR;

    /// Initialize the watchdog timer.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_init(&mut self, m: &Self::MCUSR);

    /// Start the watchdog timer with the specified timeout.
    ///
    /// If the timeout value is not supported, `Err(())` should be returned.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_start(&mut self, timeout: Timeout) -> Result<(), ()>;

    /// Feed this watchdog, to reset its period.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_feed(&mut self);

    /// Disable/stop this watchdog again.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_stop(&mut self);
}

pub struct Wdt<H, WDT> {
    p: WDT,
    _h: PhantomData<H>,
}

impl<H, WDT: WdtOps<H>> Wdt<H, WDT> {
    pub fn new(mut p: WDT, m: &WDT::MCUSR) -> Self {
        p.raw_init(m);
        Self { p, _h: PhantomData }
    }

    pub fn start(&mut self, timeout: Timeout) -> Result<(), ()> {
        self.p.raw_start(timeout)
    }

    pub fn feed(&mut self) {
        self.p.raw_feed()
    }

    pub fn stop(&mut self) {
        self.p.raw_stop()
    }
}

#[macro_export]
macro_rules! impl_wdt {
    (
        hal: $HAL:ty,
        peripheral: $WDT:ty,
        mcusr: $MCUSR:ty,
        wdtcsr_name: $wdtcsr:ident,
        timeout: |$to:ident, $w:ident| $to_match:expr,
    ) => {
        impl $crate::wdt::WdtOps<$HAL> for $WDT {
            type MCUSR = $MCUSR;

            #[inline]
            fn raw_init(&mut self, m: &Self::MCUSR) {
                /// If a prior reset was provided by the watchdog, the WDRF in MCUSR would be set,
                /// so WDRF is also cleared to allow for re-enabling the watchdog.
                m.modify(|_, w| w.wdrf().clear_bit());
            }

            #[inline]
            fn raw_start(&mut self, timeout: Timeout) -> Result<(), ()> {
                // The sequence for changing time-out configuration is as follows:
                //
                //     1. In the same operation, write a logic one to the Watchdog change enable bit
                //        (WDCE) and WDE. A logic one must be written to WDE regardless of the
                //        previous value of the WDE bit.
                //     2. Within the next four clock cycles, write the WDE and Watchdog prescaler
                //        bits (WDP) as desired, but with the WDCE bit cleared. This must be done in
                //        one operation.
                $crate::avr_device::interrupt::free(|_| {
                    // Reset the watchdog timer.
                    self.raw_feed();
                    // Enable watchdog configuration mode.
                    self.$wdtcsr
                        .modify(|_, w| w.wdce().set_bit().wde().set_bit());
                    // Enable watchdog and set interval.
                    self.$wdtcsr.write(|w| {
                        let $to = timeout;
                        let $w = w;
                        ($to_match).wde().set_bit().wdce().clear_bit()
                    });

                    Ok(())
                })
            }

            #[inline]
            fn raw_feed(&mut self) {
                avr_device::asm::wdr();
            }

            #[inline]
            fn raw_stop(&mut self) {
                // The sequence for clearing WDE is as follows:
                //
                //     1. In the same operation, write a logic one to the Watchdog change enable bit
                //        (WDCE) and WDE. A logic one must be written to WDE regardless of the
                //        previous value of the WDE bit.
                //     2. Within the next four clock cycles, clear the WDE and WDCE bits.
                //        This must be done in one operation.
                $crate::avr_device::interrupt::free(|_| {
                    // Reset the watchdog timer.
                    self.raw_feed();
                    // Enable watchdog configuration mode.
                    self.$wdtcsr
                        .modify(|_, w| w.wdce().set_bit().wde().set_bit());
                    // Disable watchdog.
                    self.$wdtcsr.reset();
                })
            }
        }
    };
}
