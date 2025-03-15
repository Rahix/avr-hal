use std::ffi::{c_char, CStr};
use std::marker::PhantomData;
use std::mem::offset_of;
use std::path::Path;

use anyhow::Result;

use goblin::elf::Elf;

#[repr(C, packed)]
#[derive(Debug)]
struct AvrDeviceInfoDesc<'a> {
    flash_start: u32,
    flash_size: u32,
    sram_start: u32,
    sram_size: u32,
    eeprom_start: u32,
    eeprom_size: u32,
    offset_table_size: u32,
    offset_table: [u32; 1],
    strtab: PhantomData<&'a [u8]>,
}

// https://avrdudes.github.io/avr-libc/avr-libc-user-manual/mem_sections.html#sec_dot_note
pub fn target_name_from_binary(binary: impl AsRef<Path>) -> Result<String> {
    let file_data = std::fs::read(binary)?;
    let note = Elf::parse(&file_data)?
        .iter_note_sections(&file_data, Some(".note.gnu.avr.deviceinfo"))
        .map(|mut it| it.nth(0))
        .flatten()
        .transpose()?
        .ok_or_else(|| anyhow::anyhow!("AVR device info section not found"))?;

    let device_info_p = note.desc.as_ptr() as *const AvrDeviceInfoDesc;
    let device_info = unsafe { &*device_info_p };
    let device_name_offset =
        offset_of!(AvrDeviceInfoDesc, strtab) as isize + device_info.offset_table[0] as isize;
    let device_name_p = unsafe { note.desc.as_ptr().offset(device_name_offset) } as *const c_char;
    let device_name = unsafe { CStr::from_ptr(device_name_p) }.to_str()?;

    Ok(device_name.to_string())
}
