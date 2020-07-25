

pub enum ClockRateDivision {
    Factor2,
    Factor4,
    Factor8,
    Factor16,
    Factor32,
    Factor64,
    Factor128,
}

pub enum ReferenceVoltage {
    Aref,
    Vcc,
    Internal,
}

pub struct AdcSettings{
    pub adps: ClockRateDivision,
    pub aref: ReferenceVoltage
}


impl Default for AdcSettings {
    fn default() -> Self {
        Self { adps: ClockRateDivision::Factor128, aref: ReferenceVoltage::Vcc}
    }
}

#[macro_export]
macro_rules! impl_adc {
    (
        pub struct $Adc:ident {
            type ChannelID = $ID:ty;
            peripheral: $ADC:ty,
            pins: {$($pxi:ident: ($PXi:ident, $ChannelID:expr, $name:ident),)+}
        }
    ) => {

        use $crate::void::Void;
        use $crate::hal::adc::{Channel, OneShot};
        use $crate::nb;
        use $crate::port::mode::Analog;
        pub use avr_hal::adc::*;

        pub struct $Adc {
            peripheral: $ADC,
            is_reading: bool,
        }

        impl $Adc {
            pub fn new(peripheral: $ADC, settings: AdcSettings) -> $Adc {
                let s = Self { peripheral, is_reading: false } ;
                s.enable(settings);
                s
            }

            fn enable(&self, settings: AdcSettings) {
                self.peripheral.adcsra.write(|w| {w.aden().set_bit();
                                            match settings.adps {
                                                ClockRateDivision::Factor2 => w.adps().val_0x01(),
                                                ClockRateDivision::Factor4 => w.adps().val_0x02(),
                                                ClockRateDivision::Factor8 => w.adps().val_0x03(),
                                                ClockRateDivision::Factor16 => w.adps().val_0x04(),
                                                ClockRateDivision::Factor32 => w.adps().val_0x05(),
                                                ClockRateDivision::Factor64 => w.adps().val_0x06(),
                                                ClockRateDivision::Factor128 => w.adps().val_0x07(),
                                            }});
                self.peripheral.admux.write(|w| match settings.aref {
                    ReferenceVoltage::Aref => w.refs().val_0x00(),
                    ReferenceVoltage::Vcc => w.refs().val_0x01(),
                    ReferenceVoltage::Internal => w.refs().val_0x03(),
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
            type Error = ();

            fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
                match (self.is_reading, self.peripheral.adcsra.read().adsc().bit_is_set()) {
                    (true, true) =>  Err(nb::Error::WouldBlock),
                    (true, false) => {
                        self.is_reading = false;
                        Ok(self.peripheral.adc.read().bits().into())
                    },
                    (false, _) => {
                        self.is_reading = true; self.peripheral.admux.modify(|_, w| w.adlar().clear_bit()
                                                                                     .mux().variant(PIN::channel()));
                        self.peripheral.adcsra.modify(|_, w| w.adsc().set_bit());
                        Err(nb::Error::WouldBlock)
                    }
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
                    /// Make this pin a analog input and enable the internal pull-up
                    pub fn into_analog_input(self, adc: &mut $Adc) -> $PXi<Analog> {
                        adc.peripheral.didr0.modify(|_, w| w.$name().set_bit());
                        $PXi { _mode: core::marker::PhantomData }
                    }
            }
        )+
    }
}
