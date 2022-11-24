
#[macro_export]
macro_rules! impl_eeprom_traditional {
    (
        name: $Name:ident,
        peripheral: $ep:ty,
        capacity: $capacity:literal,
        set_address: |$periph_var:ident, $address:ident| $set_address:block,
        set_erasewrie_mode: |$periph_ewmode_var:ident| $set_erasewrie_mode:block,
        set_erase_mode: |$periph_emode_var:ident| $set_erase_mode:block,
        set_write_mode: |$periph_wmode_var:ident| $set_write_mode:block,
    ) => {
        /// Flash erase/program error
        #[derive(Debug, Clone, Copy)]
        pub enum Error {
            Bounds,
            Other,
        }        
        pub struct $Name {}
        impl $Name {
            #[inline]
            unsafe fn wait_ready(&self) {
                //Wait for completion of previous write.
                while (*<$ep>::ptr()).eecr.read().eepe().bit_is_set() {}
            }

            #[inline]
            unsafe fn eeprom_set_address(&self, address: u16) {
                // wait until EEPE become to zero by hardware
                self.wait_ready();

                let $periph_var = &(*<$ep>::ptr());
                let $address = address;
                $set_address
            }
            #[inline]
            unsafe fn eeprom_set_erasewrite_mode(&self) {
                let $periph_ewmode_var = &(*<$ep>::ptr());
                $set_erasewrie_mode
            }
            #[inline]
            unsafe fn eeprom_set_erase_mode(&self) {
                let $periph_emode_var = &(*<$ep>::ptr());
                $set_erase_mode
            }
            #[inline]
            unsafe fn eeprom_set_write_mode(&self) {
                let $periph_wmode_var = &(*<$ep>::ptr());
                $set_write_mode
            }

            unsafe fn eeprom_get_char(&mut self, address: u16) -> u8 {
                self.eeprom_set_address(address);
                //Start EEPROM read operation
                (*<$ep>::ptr()).eecr.write(|w| w.eere().set_bit());

                // Return the byte read from EEPROM
                (*<$ep>::ptr()).eedr.read().bits()
            }
            /// attention: if call it, should better call between disab/enable interrupt
            unsafe fn eeprom_put_char(&mut self, address: u16, data: u8) -> () {
                self.eeprom_set_address(address);

                let periph = &(*<$ep>::ptr());

                //Start EEPROM read operation
                periph.eecr.write(|w| w.eere().set_bit());
                let old_value = periph.eedr.read().bits();
                let diff_mask = old_value ^ data;

                // Check if any bits are changed to '1' in the new value.
                if (diff_mask & data) != 0 {
                    // Now we know that _some_ bits need to be erased to '1'.

                    // Check if any bits in the new value are '0'.
                    if data != 0xff {
                        // Now we know that some bits need to be programmed to '0' also.
                        periph.eedr.write(|w| w.bits(data)); // Set EEPROM data register.

                        self.eeprom_set_erasewrite_mode();

                        periph.eecr.write(|w| w.eepe().set_bit()); // Start Erase+Write operation.
                    } else {
                        // Now we know that all bits should be erased.

                        self.eeprom_set_erase_mode();

                        periph.eecr.write(|w| w.eepe().set_bit()); // Start Erase-only operation.
                    }
                }
                //Now we know that _no_ bits need to be erased to '1'.
                else {
                    // Check if any bits are changed from '1' in the old value.
                    if diff_mask != 0 {
                        // Now we know that _some_ bits need to the programmed to '0'.
                        periph.eedr.write(|w| w.bits(data)); // Set EEPROM data register.

                        self.eeprom_set_write_mode();

                        periph.eecr.write(|w| w.eepe().set_bit()); // Start Write-only operation.
                    }
                }
            }
        }

        impl $crate::embedded_storage::nor_flash::ReadNorFlash for $Name {
            type Error = Error;
            const READ_SIZE: usize = 1;

            fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
                if bytes.len() + offset as usize > $capacity {
                    return Err(Self::Error::Bounds);
                }
                let len = bytes.len();
                let mut offset = offset as u16;
                for i in 0..len {
                    bytes[i] = unsafe { self.eeprom_get_char(offset) };
                    offset += 1;
                }
                Ok(())
            }
            fn capacity(&self) -> usize {
                $capacity
            }
        }

        impl $crate::embedded_storage::nor_flash::NorFlash for $Name {
            const WRITE_SIZE: usize = 1;
            const ERASE_SIZE: usize = 1;
            fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
                if to > $capacity {
                    return Err(Self::Error::Bounds);
                }

                $crate::avr_device::interrupt::free(|_cs| {
                    unsafe {
                        for i in from..to {
                            self.eeprom_set_address(i as u16);

                            // Now we know that all bits should be erased.
                            self.eeprom_set_erase_mode();

                            (*<$ep>::ptr()).eecr.write(|w| w.eepe().set_bit()); // Start Erase-only operation.
                        }
                    }
                });

                Ok(())
            }

            fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
                
                if bytes.len() + offset as usize > $capacity {
                    return Err(Self::Error::Bounds);
                }

                let mut offset = offset as u16;
                for i in bytes {
                    $crate::avr_device::interrupt::free(|_cs| {
                        unsafe { self.eeprom_put_char(offset as u16, *i) };
                        offset += 1;
                    });
                }
                Ok(())
            }
        }
        // AVR supports multiple writes
        impl $crate::embedded_storage::nor_flash::MultiwriteNorFlash for $Name {}
    }; // () => {};
}

#[macro_export]
macro_rules! impl_atmega_eeprom {
    (
        name: $Name:ident,
        peripheral: $ep:ty,
        capacity: $capacity:literal,
        set_address: |$periph_var:ident, $address:ident| $set_address:block,

    ) => {
        $crate::impl_eeprom_traditional! {
            name: $Name,
            peripheral: $ep,
            capacity: $capacity,
            set_address: |$periph_var, $address|$set_address,
            set_erasewrie_mode: |peripheral| {
                peripheral.eecr.write(|w| {
                    w.eempe().set_bit(); // Set Master Write Enable bit
                    w.eepm().val_0x00() // ...and and Erase+Write mode mode..
                });

            },
            set_erase_mode: |peripheral| {
                peripheral.eecr.write(|w| {
                    w.eempe().set_bit(); // Set Master Write Enable bit
                    w.eepm().val_0x01() // ...and Erase-only mode..
                });

            },
            set_write_mode: |peripheral| {
                peripheral.eecr.write(|w| {
                    w.eempe().set_bit(); // Set Master Write Enable bit
                    w.eepm().val_0x02() // ...and Write-only mode..
                });

            },
        }
    };
}

#[macro_export]
macro_rules! impl_attiny_eeprom {
    (
        name: $Name:ident,
        peripheral: $ep:ty,
        capacity: $capacity:literal,
        set_address: |$periph_var:ident, $address:ident| $set_address:block,
    ) => {
        $crate::impl_eeprom_traditional! {
            name: $Name,
            peripheral: $ep,
            capacity: $capacity,
            set_address: |$periph_var, $address|$set_address,
            set_erasewrie_mode: |peripheral| {
                peripheral.eecr.write(|w| {
                    w.eempe().set_bit(); // Set Master Write Enable bit
                    w.eepm().atomic() // ...and and Erase+Write mode mode..
                });

            },
            set_erase_mode: |peripheral| {
                peripheral.eecr.write(|w| {
                    w.eempe().set_bit(); // Set Master Write Enable bit
                    w.eepm().erase() // ...and Erase-only mode..
                });

            },
            set_write_mode: |peripheral| {
                peripheral.eecr.write(|w| {
                    w.eempe().set_bit(); // Set Master Write Enable bit
                    w.eepm().write() // ...and Write-only mode..
                });

            },
        }
    };
}
