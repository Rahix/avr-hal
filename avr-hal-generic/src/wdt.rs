//! WDT Implementation

/// Implement traits for the watchdog interface
#[macro_export]
macro_rules! impl_wdt {
    // Build the match for translating time out period into wdtcsr bits
    ($w:ident, $period:ident, $($variant:ident => {$($bits:ident()).+}),+) =>
    {
        (match $period.into() {
            $(WatchdogTimeOutPeriod::$variant => $w.$($bits()).+),+
        })
        .wde()
        .set_bit()
        .wdce()
        .clear_bit()
    };
    // Create time out periods and watchdog
    (
        pub enum WatchdogTimeOutPeriod {
            $($(#[$doc:meta])*$variant:ident $prescale:tt),+$(,)*
        }

        pub struct $Wdt:ident {
            mcu_status_register: $MCUSR:ty,
            peripheral: $WDT:ty,
        }
    ) => {
        use $crate::hal::watchdog::*;

        /// Approximate length of the time-out period before the watchdog provides a system reset.
        ///
        /// After enabling the watchdog timer, call [`Watchdog::feed`] before the period ends to
        /// prevent a reset.
        ///
        /// [`Watchdog::feed`]: watchdog/trait.Watchdog.html#tymethod.feed
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum WatchdogTimeOutPeriod {
            $($(#[$doc])*$variant),+
        }

        /// Provides a system reset when a counter reaches a given time-out value.
        ///
        /// # Note
        /// Changing the watchdog configuration requires two separate writes to WDTCSR where the
        /// second write must occur within 4 cycles of the first or the configuration will not
        /// change. You may need to adjust optimization settings to prevent other operations from
        /// being emitted between these two writes.
        ///
        /// # Example
        /// ```
        /// let mut watchdog = board::wdt::Wdt::new(&dp.CPU.mcusr, dp.WDT);
        /// watchdog.start(board::wdt::WatchdogTimeOutPeriod::Ms8000);
        ///
        /// loop {
        ///     watchdog.feed();
        /// }
        /// ```
        pub struct $Wdt {
            peripheral: $WDT,
        }

        impl $Wdt {
            /// Initializes a Wdt.
            ///
            /// If a prior reset was provided by the watchdog, the WDRF in MCUSR would be set, so
            /// WDRF is also cleared to allow for re-enabling the watchdog.
            pub fn new(mcu_status_register: &$MCUSR, peripheral: $WDT) -> Self {
                mcu_status_register.modify(|_, w| w.wdrf().clear_bit());
                $Wdt { peripheral }
            }
        }

        impl $crate::hal::watchdog::WatchdogEnable for $Wdt {
            type Time = WatchdogTimeOutPeriod;

            fn start<T>(&mut self, period: T)
            where
                T: Into<Self::Time>,
            {
                // The sequence for changing time-out configuration is as follows:
                //
                //     1. In the same operation, write a logic one to the Watchdog change enable bit
                //        (WDCE) and WDE. A logic one must be written to WDE regardless of the
                //        previous value of the WDE bit.
                //     2. Within the next four clock cycles, write the WDE and Watchdog prescaler
                //        bits (WDP) as desired, but with the WDCE bit cleared. This must be done in
                //        one operation.
                avr_hal::avr_device::interrupt::free(|_| {
                    // Reset the watchdog timer
                    self.feed();
                    // Enable watchdog configuration mode
                    self.peripheral
                        .wdtcsr
                        .modify(|_, w| w.wdce().set_bit().wde().set_bit());
                    // Enable watchdog and set interval
                    self.peripheral.wdtcsr.write(|w| {
                        $crate::impl_wdt!(w, period, $($variant => $prescale),+)
                    });
                });
            }
        }

        impl $crate::hal::watchdog::Watchdog for $Wdt {
            #[inline]
            fn feed(&mut self) {
                avr_device::asm::wdr();
            }
        }

        impl $crate::hal::watchdog::WatchdogDisable for $Wdt {
            fn disable(&mut self) {
                // The sequence for clearing WDE is as follows:
                //
                //     1. In the same operation, write a logic one to the Watchdog change enable bit
                //        (WDCE) and WDE. A logic one must be written to WDE regardless of the
                //        previous value of the WDE bit.
                //     2. Within the next four clock cycles, clear the WDE and WDCE bits.
                //        This must be done in one operation.
                avr_hal::avr_device::interrupt::free(|_| {
                    // Reset the watchdog timer
                    self.feed();
                    // Enable watchdog configuration mode
                    self.peripheral
                        .wdtcsr
                        .modify(|_, w| w.wdce().set_bit().wde().set_bit());
                    // Disable watchdog
                    self.peripheral.wdtcsr.reset();
                });
            }
        }
    };
}
