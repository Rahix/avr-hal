#[macro_export]
macro_rules! impl_adc {
    (
        pub struct $Adc:ident {
            type ChannelID = $ID:ty;
            peripheral: $ADC:ty,
            pins: {$($pxi:ident: ($PXi:ident, $ChannelID:expr),)+}
        }
    ) => {

        use $crate::void::Void;
        use $crate::hal::adc::{Channel, OneShot};
        use $crate::nb;
        use $crate::port::mode::Analog;

        pub struct $Adc {
            peripheral: $ADC,
        }

        impl $Adc {
            pub fn new(peripheral: $ADC) -> $Adc {
                peripheral.adcsra.write(|w| w.aden().set_bit()
                                             .adps().val_0x07());
                Self { peripheral}
            }

            pub fn adcsra(&self) -> u8 {
                self.peripheral.adcsra.read().bits()
            }

            pub fn admux(&self) -> u8 {
                self.peripheral.admux.read().bits()
            }
        }

        impl<WORD, PIN> OneShot<$Adc, WORD, PIN> for $Adc
        where
            WORD: From<u16>,
            PIN: Channel<$Adc, ID=$ID>,
        {
            type Error = ();

            fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {

                if self.peripheral.adcsra.read().adsc().bit_is_set() {
                    return Err(nb::Error::WouldBlock)
                }
                self.peripheral.admux.modify(|_, w| w
                    .refs().val_0x00()
                    .adlar().clear_bit()
                    .mux().variant(PIN::channel()));

                self.peripheral.adcsra.modify(|_, w| w.adsc().set_bit());

                if self.peripheral.adcsra.read().adsc().bit_is_set() {
                    return Err(nb::Error::WouldBlock)
                }
                Ok(self.peripheral.adc.read().bits().into())
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
                        adc.peripheral.didr0.write(|w| w.adc0d().set_bit());
                        $PXi { _mode: core::marker::PhantomData }
                    }
            }
        )+
    }
}
