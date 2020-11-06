/// The division factor between the system clock frequency and the input clock to the AD converter.
///
/// To get 10bit precision, clock from 50kHz to 200kHz must be supplied. If you need less precision, you can supply higher clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockRateDivision {
    Factor2,
    Factor4,
    Factor8,
    Factor16,
    Factor32,
    Factor64,
    Factor128,
}

impl Default for ClockRateDivision {
    fn default() -> Self {
        Self::Factor128
    }
}

/// Select the voltage reference for the ADC peripheral
///
/// The internal voltage reference options may not be used if an external reference voltage is being applied to the AREF pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceVoltage {
    /// Voltage applied to AREF pin.
    Aref,
    /// Default reference voltage.
    AVcc,
    /// Internal reference voltage
    Internal,
}

impl Default for ReferenceVoltage {
    fn default() -> Self {
        Self::AVcc
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdcSettings {
    pub clock_divider: ClockRateDivision,
    pub ref_voltage: ReferenceVoltage,
}

#[macro_export]
macro_rules! impl_adc {
    (
        pub struct $Adc:ident {
            type ChannelID = $ID:ty;
            peripheral: $ADC:ty,
            set_mux: |$periph_var:ident, $id_var:ident| $set_mux:block,
            pins: {
                $($pxi:ident: ($PXi:ident, $ChannelID:expr, $didr:ident::$didr_method:ident),)+
            }
        }
    ) => {

        use $crate::void::Void;
        use $crate::hal::adc::{Channel, OneShot};
        use $crate::nb;
        use $crate::port::mode::Analog;
        pub use $crate::adc::*;

        pub struct $Adc {
            peripheral: $ADC,
            reading_channel: Option<$ID>,
        }

        impl $Adc {
            pub fn new(peripheral: $ADC, settings: AdcSettings) -> $Adc {
                let s = Self { peripheral, reading_channel: None } ;
                s.enable(settings);
                s
            }

            fn enable(&self, settings: AdcSettings) {
                self.peripheral.adcsra.write(|w| {
                    w.aden().set_bit();
                    match settings.clock_divider {
                        ClockRateDivision::Factor2 => w.adps().prescaler_2(),
                        ClockRateDivision::Factor4 => w.adps().prescaler_4(),
                        ClockRateDivision::Factor8 => w.adps().prescaler_8(),
                        ClockRateDivision::Factor16 => w.adps().prescaler_16(),
                        ClockRateDivision::Factor32 => w.adps().prescaler_32(),
                        ClockRateDivision::Factor64 => w.adps().prescaler_64(),
                        ClockRateDivision::Factor128 => w.adps().prescaler_128(),
                    }});
                self.peripheral.admux.write(|w| match settings.ref_voltage {
                    ReferenceVoltage::Aref => w.refs().aref(),
                    ReferenceVoltage::AVcc => w.refs().avcc(),
                    ReferenceVoltage::Internal => w.refs().internal(),
                });

            }

            fn disable(&self) {
                self.peripheral.adcsra.reset();
            }

            pub fn release(self) -> $ADC {
                self.disable();
                self.peripheral
            }
        }

        impl<WORD, PIN> OneShot<$Adc, WORD, PIN> for $Adc
        where
            WORD: From<u16>,
            PIN: Channel<$Adc, ID=$ID>,
        {
            type Error = Void;

            fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
                match (self.reading_channel, self.peripheral.adcsra.read().adsc().bit_is_set()) {
                    // Measurement on current pin is ongoing
                    (Some(channel), true) if channel == PIN::channel() => Err(nb::Error::WouldBlock),
                    // Measurement on current pin completed
                    (Some(channel), false) if channel == PIN::channel() => {
                        self.reading_channel = None;
                        Ok(self.peripheral.adc.read().bits().into())
                    },
                    // Measurement on other pin is ongoing
                    (Some(_), _) => {
                        self.reading_channel = None;
                        Err(nb::Error::WouldBlock)
                    },
                    // Start measurement
                    (None, _) => {
                        self.reading_channel = Some(PIN::channel());
                        {
                            let $periph_var = &mut self.peripheral;
                            let $id_var = PIN::channel();

                            $set_mux
                        }
                        self.peripheral.adcsra.modify(|_, w| w.adsc().set_bit());
                        Err(nb::Error::WouldBlock)
                    },
                }
            }
        }

        $(
            impl Channel<$Adc> for $PXi<Analog> {
                type ID = $ID;
                fn channel() -> Self::ID {
                    $ChannelID
                }
            }

            impl<MODE> $PXi<MODE> {
                    /// Make this pin a analog input and set the didr register
                    pub fn into_analog_input(self, adc: &mut $Adc) -> $PXi<Analog> {
                        adc.peripheral.$didr.modify(|_, w| w.$didr_method().set_bit());
                        $PXi { _mode: core::marker::PhantomData }
                    }
            }
        )+
    }
}
