//!HAL abstraction for EEPROM
//!
use core::marker;

#[derive(ufmt::derive::uDebug, Debug)]
pub struct OutOfBoundsError;

/// Internal trait for low-level EEPROM peripherals.
///
/// This trait defines the common interface for all EEPROM peripheral variants.
pub trait EepromOps<H> {
    const CAPACITY: u16;

    /// Read a single byte from offset `address`.  Does not do a bounds check.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_read_byte(&self, address: u16) -> u8;
    /// Erase and write a single byte at offset `address`.  Does not do a bounds check.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_write_byte(&mut self, address: u16, data: u8);
    /// Erase a single byte at offset `address`.  Does not do a bounds check.
    ///
    /// **Warning**: This is a low-level method and should not be called directly from user code.
    fn raw_erase_byte(&mut self, address: u16);
}

pub struct Eeprom<H, EEPROM> {
    p: EEPROM,
    _h: marker::PhantomData<H>,
}

impl<H, EEPROM> Eeprom<H, EEPROM>
where
    EEPROM: EepromOps<H>,
{
    pub const CAPACITY: u16 = EEPROM::CAPACITY;

    #[inline]
    pub fn new(p: EEPROM) -> Self {
        Self {
            p,
            _h: marker::PhantomData,
        }
    }
    #[inline]
    pub fn capacity(&self) -> u16 {
        Self::CAPACITY
    }

    #[inline]
    pub fn read_byte(&self, offset: u16) -> u8 {
        assert!(offset < Self::CAPACITY);
        self.p.raw_read_byte(offset)
    }

    #[inline]
    pub fn write_byte(&mut self, offset: u16, data: u8) {
        assert!(offset < Self::CAPACITY);
        self.p.raw_write_byte(offset, data)
    }

    #[inline]
    pub fn erase_byte(&mut self, offset: u16) {
        assert!(offset < Self::CAPACITY);
        self.p.raw_erase_byte(offset)
    }

    pub fn read(&self, offset: u16, buf: &mut [u8]) -> Result<(), OutOfBoundsError> {
        if buf.len() as u16 + offset > Self::CAPACITY {
            return Err(OutOfBoundsError);
        }
        for (i, byte) in buf.iter_mut().enumerate() {
            *byte = self.p.raw_read_byte(offset + i as u16);
        }
        Ok(())
    }

    pub fn write(&mut self, offset: u16, buf: &[u8]) -> Result<(), OutOfBoundsError> {
        if buf.len() as u16 + offset > Self::CAPACITY {
            return Err(OutOfBoundsError);
        }

        for (i, byte) in buf.iter().enumerate() {
            self.p.raw_write_byte(offset + i as u16, *byte)
        }
        Ok(())
    }

    pub fn erase(&mut self, from: u16, to: u16) -> Result<(), OutOfBoundsError> {
        if to > Self::CAPACITY || from > to {
            return Err(OutOfBoundsError);
        }

        for i in from..to {
            self.p.raw_erase_byte(i)
        }

        Ok(())
    }
}

impl<H, EEPROM> embedded_storage::nor_flash::ReadNorFlash for Eeprom<H, EEPROM>
where
    EEPROM: EepromOps<H>,
{
    type Error = OutOfBoundsError;
    const READ_SIZE: usize = 1;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        Eeprom::<H, EEPROM>::read(self, offset as u16, bytes)
    }

    fn capacity(&self) -> usize {
        Eeprom::<H, EEPROM>::capacity(self) as usize
    }
}

impl<H, EEPROM> embedded_storage::nor_flash::NorFlash for Eeprom<H, EEPROM>
where
    EEPROM: EepromOps<H>,
{
    const WRITE_SIZE: usize = 1;
    const ERASE_SIZE: usize = 1;
    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        Eeprom::<H, EEPROM>::erase(self, from as u16, to as u16)
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        Eeprom::<H, EEPROM>::write(self, offset as u16, bytes)
    }
}
// AVR supports multiple writes
impl<H, EEPROM> embedded_storage::nor_flash::MultiwriteNorFlash for Eeprom<H, EEPROM> where
    EEPROM: EepromOps<H>
{
}

#[macro_export]
macro_rules! impl_eeprom_common {
    (
        hal: $HAL:ty,
        peripheral: $EEPROM:ty,
        capacity: $capacity:literal,
        addr_width: $addrwidth:ty,
        set_address: |$periph_var:ident, $address:ident| $set_address:block,
        set_erasewrite_mode: |$periph_ewmode_var:ident| $set_erasewrite_mode:block,
        set_erase_mode: |$periph_emode_var:ident| $set_erase_mode:block,
        set_write_mode: |$periph_wmode_var:ident| $set_write_mode:block,
    ) => {
        impl $crate::eeprom::EepromOps<$HAL> for $EEPROM {
            const CAPACITY: u16 = $capacity;

            fn raw_read_byte(&self, address: u16) -> u8 {
                unsafe {
                    {
                        let $periph_var = &self;
                        let $address = address as $addrwidth;
                        $set_address
                    }

                    self.eecr.write(|w| w.eere().set_bit());
                    self.eedr.read().bits()
                }
            }

            fn raw_write_byte(&mut self, address: u16, data: u8) {
                unsafe {
                    {
                        let $periph_var = &self;
                        let $address = address as $addrwidth;
                        $set_address
                    }

                    //Start EEPROM read operation
                    self.eecr.write(|w| w.eere().set_bit());
                    let old_value = self.eedr.read().bits();
                    let diff_mask = old_value ^ data;

                    // Check if any bits are changed to '1' in the new value.
                    if (diff_mask & data) != 0 {
                        // Now we know that _some_ bits need to be erased to '1'.

                        // Check if any bits in the new value are '0'.
                        if data != 0xff {
                            // Now we know that some bits need to be programmed to '0' also.
                            self.eedr.write(|w| w.bits(data)); // Set EEPROM data register.

                            {
                                let $periph_ewmode_var = &self;
                                $set_erasewrite_mode
                            }
                            self.eecr.modify(|_, w| w.eepe().set_bit()); // Start Erase+Write operation.
                        } else {
                            // Now we know that all bits should be erased.
                            {
                                let $periph_emode_var = &self;
                                $set_erase_mode
                            }
                            self.eecr.modify(|_, w| w.eepe().set_bit()); // Start Erase-only operation.
                        }
                    }
                    //Now we know that _no_ bits need to be erased to '1'.
                    else {
                        // Check if any bits are changed from '1' in the old value.
                        if diff_mask != 0 {
                            // Now we know that _some_ bits need to the programmed to '0'.
                            self.eedr.write(|w| w.bits(data)); // Set EEPROM data register.
                            {
                                let $periph_wmode_var = &self;
                                $set_write_mode
                            }
                            self.eecr.modify(|_, w| w.eepe().set_bit()); // Start Write-only operation.
                        }
                    }
                }
            }

            fn raw_erase_byte(&mut self, address: u16) {
                unsafe {
                    {
                        let $periph_var = &self;
                        let $address = address as $addrwidth;
                        $set_address
                    }
                    // Now we know that all bits should be erased.
                    {
                        let $periph_emode_var = &self;
                        $set_erase_mode
                    }
                    // Start Erase-only operation.
                    self.eecr.modify(|_, w| w.eepe().set_bit());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_eeprom_atmega_old {
    (
        hal: $HAL:ty,
        peripheral: $EEPROM:ty,
        capacity: $capacity:literal,
        addr_width: $addrwidth:ty,
        set_address: |$periph_var:ident, $address:ident| $set_address:block,
    ) => {
        mod atmega_helper {
            #[inline]
            pub unsafe fn wait_read(regs: &$EEPROM) {
                //Wait for completion of previous write.
                while regs.eecr.read().eewe().bit_is_set() {}
            }

            #[inline]
            pub unsafe fn set_address(regs: &$EEPROM, address: u16) {
                wait_read(regs);
                let $periph_var = regs;
                let $address = address;
                $set_address
            }
        }

        impl $crate::eeprom::EepromOps<$HAL> for $EEPROM {
            const CAPACITY: u16 = $capacity;

            fn raw_read_byte(&self, address: u16) -> u8 {
                unsafe {
                    atmega_helper::set_address(&self, address);
                }
                self.eecr.write(|w| w.eere().set_bit());
                self.eedr.read().bits()
            }

            fn raw_write_byte(&mut self, address: u16, data: u8) {
                unsafe {
                    atmega_helper::set_address(&self, address);
                }

                //Start EEPROM read operation
                self.eedr.write(|w| unsafe { w.bits(data) });

                self.eecr.write(|w| w.eemwe().set_bit().eewe().clear_bit());

                self.eecr.write(|w| w.eewe().set_bit());
            }

            fn raw_erase_byte(&mut self, address: u16) {
                self.raw_write_byte(address, 0);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_eeprom_atmega {
    (
        hal: $HAL:ty,
        peripheral: $EEPROM:ty,
        capacity: $capacity:literal,
        addr_width: $addrwidth:ty,
        set_address: |$periph_var:ident, $address:ident| $set_address:block,
    ) => {
        mod atmega_helper {
            #[inline]
            pub unsafe fn wait_read(regs: &$EEPROM) {
                //Wait for completion of previous write.
                while regs.eecr.read().eepe().bit_is_set() {}
            }
            #[inline]
            pub unsafe fn set_address(regs: &$EEPROM, address: $addrwidth) {
                wait_read(regs);
                let $periph_var = regs;
                let $address = address;
                $set_address
            }
            #[inline]
            pub unsafe fn set_erasewrite_mode(regs: &$EEPROM) {
                regs.eecr.write(|w| {
                    // Set Master Write Enable bit, and and Erase+Write mode mode..
                    w.eempe().set_bit().eepm().val_0x00()
                })
            }
            #[inline]
            pub unsafe fn set_erase_mode(regs: &$EEPROM) {
                regs.eecr.write(|w| {
                    // Set Master Write Enable bit, and Erase-only mode..
                    w.eempe().set_bit().eepm().val_0x01()
                });
            }
            #[inline]
            pub unsafe fn set_write_mode(regs: &$EEPROM) {
                regs.eecr.write(|w| {
                    // Set Master Write Enable bit, and Write-only mode..
                    w.eempe().set_bit().eepm().val_0x02()
                });
            }
        }

        $crate::impl_eeprom_common! {
            hal: $HAL,
            peripheral: $EEPROM,
            capacity: $capacity,
            addr_width: $addrwidth,
            set_address: |peripheral, address| {atmega_helper::set_address(peripheral, address)},
            set_erasewrite_mode: |peripheral| {atmega_helper::set_erasewrite_mode(peripheral)},
            set_erase_mode: |peripheral| {atmega_helper::set_erase_mode(peripheral)},
            set_write_mode: |peripheral| {atmega_helper::set_write_mode(peripheral)},
        }
    };
}

#[macro_export]
macro_rules! impl_eeprom_attiny {
    (
        hal: $HAL:ty,
        peripheral: $EEPROM:ty,
        capacity: $capacity:literal,
        addr_width: $addrwidth:ty,
        set_address: |$periph_var:ident, $address:ident| $set_address:block,
    ) => {
        mod attiny_helper {
            #[inline]
            pub unsafe fn wait_read(regs: &$EEPROM) {
                while regs.eecr.read().eepe().bit_is_set() {}
            }
            #[inline]
            pub unsafe fn set_address(regs: &$EEPROM, address: $addrwidth) {
                wait_read(regs);
                let $periph_var = regs;
                let $address = address;
                $set_address
            }
            #[inline]
            pub unsafe fn set_erasewrite_mode(regs: &$EEPROM) {
                regs.eecr.write(|w| {
                    // Set Master Write Enable bit...and and Erase+Write mode mode..
                    w.eempe().set_bit().eepm().atomic()
                });
            }
            #[inline]
            pub unsafe fn set_erase_mode(regs: &$EEPROM) {
                regs.eecr.write(|w| {
                    // Set Master Write Enable bit, and Erase-only mode..
                    w.eempe().set_bit().eepm().erase()
                });
            }
            #[inline]
            pub unsafe fn set_write_mode(regs: &$EEPROM) {
                regs.eecr.write(|w| {
                    // Set Master Write Enable bit, and Write-only mode..
                    w.eempe().set_bit().eepm().write()
                });
            }
        }

        $crate::impl_eeprom_common! {
            hal: $HAL,
            peripheral: $EEPROM,
            capacity: $capacity,
            addr_width: $addrwidth,
            set_address: |peripheral, address| {attiny_helper::set_address(peripheral, address)},
            set_erasewrite_mode: |peripheral| {attiny_helper::set_erasewrite_mode(peripheral)},
            set_erase_mode: |peripheral| {attiny_helper::set_erase_mode(peripheral)},
            set_write_mode: |peripheral| {attiny_helper::set_write_mode(peripheral)},
        }
    };
}
