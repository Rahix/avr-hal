use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::{Context as _, anyhow, bail};

/// Highest address that still belongs to the AVR flash address space.  Data
/// (SRAM) segments live at `0x800000` and above.
const AVR_DATA_SPACE_START: u64 = 0x80_0000;

const ELF_HEADER_LEN: usize = 52;
const ELF_PROGRAM_HEADER_LEN: usize = 32;
const PT_LOAD: u32 = 1;

/// Returns the number of bytes the firmware will occupy in flash, or `None`
/// if the file format does not allow a reliable size to be derived.
///
/// ELF images are measured from their loadable segments: avrdude programs each
/// `PT_LOAD` segment at its physical address, so the end of the highest
/// flash-resident segment is the amount of flash that gets written.
pub fn image_size(path: &Path) -> anyhow::Result<Option<u64>> {
    let mut file = File::open(path)
        .with_context(|| format!("failed opening firmware image {}", path.display()))?;
    let file_len = file.metadata()?.len();

    let mut header = [0u8; ELF_HEADER_LEN];
    let filled = read_up_to(&mut file, &mut header)?;

    if filled < 4 || &header[..4] != b"\x7fELF" {
        return Ok(if is_raw_binary(path) {
            Some(file_len)
        } else {
            None
        });
    }
    if filled < ELF_HEADER_LEN {
        bail!(
            "firmware image {} has a truncated ELF header",
            path.display()
        );
    }
    if header[4] != 1 || header[5] != 1 {
        // Not a 32-bit little-endian image, so not something an AVR can run.
        return Ok(None);
    }

    let phoff = u64::from(read_u32(&header, 0x1c));
    let phentsize = read_u16(&header, 0x2a);
    let phnum = read_u16(&header, 0x2c);

    if phnum == 0 {
        return Ok(None);
    }
    if usize::from(phentsize) < ELF_PROGRAM_HEADER_LEN {
        bail!(
            "firmware image {} has an invalid program header size",
            path.display()
        );
    }

    let table_len = u64::from(phentsize) * u64::from(phnum);
    if phoff
        .checked_add(table_len)
        .is_none_or(|end| end > file_len)
    {
        bail!(
            "firmware image {} has a program header table that extends past the end of the file",
            path.display()
        );
    }

    file.seek(SeekFrom::Start(phoff))?;
    let mut table =
        vec![0u8; usize::try_from(table_len).context("program header table too large")?];
    file.read_exact(&mut table)?;

    let mut highest_end = 0u64;
    for index in 0..usize::from(phnum) {
        let offset = index * usize::from(phentsize);
        let entry = &table[offset..offset + ELF_PROGRAM_HEADER_LEN];
        if read_u32(entry, 0) != PT_LOAD {
            continue;
        }
        let paddr = u64::from(read_u32(entry, 12));
        let filesz = u64::from(read_u32(entry, 16));
        if filesz == 0 || paddr >= AVR_DATA_SPACE_START {
            continue;
        }
        highest_end = highest_end.max(paddr + filesz);
    }

    Ok((highest_end > 0).then_some(highest_end))
}

/// Refuses to flash an image that would overlap the bootloader region.
pub fn check_application_size(path: &Path, limit: u32, board: &str) -> anyhow::Result<()> {
    let Some(size) = image_size(path)? else {
        return Ok(());
    };
    if size > u64::from(limit) {
        return Err(anyhow!(
            "firmware is {size} bytes, but board `{board}` allows at most {limit} bytes (bootloader region)"
        ));
    }
    Ok(())
}

fn read_up_to(file: &mut File, buf: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;
    while filled < buf.len() {
        match file.read(&mut buf[filled..])? {
            0 => break,
            n => filled += n,
        }
    }
    Ok(filled)
}

fn read_u16(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([buf[offset], buf[offset + 1]])
}

fn read_u32(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        buf[offset],
        buf[offset + 1],
        buf[offset + 2],
        buf[offset + 3],
    ])
}

fn is_raw_binary(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("bin"))
}

#[cfg(test)]
mod tests {
    use super::{check_application_size, image_size};
    use std::io::Write;

    struct TestFile {
        file: tempfile::NamedTempFile,
    }

    impl TestFile {
        fn new(suffix: &str, contents: &[u8]) -> Self {
            let mut file = tempfile::Builder::new()
                .suffix(suffix)
                .tempfile()
                .expect("create temporary file");
            file.write_all(contents).expect("write temporary file");
            file.flush().expect("flush temporary file");
            Self { file }
        }

        fn path(&self) -> &std::path::Path {
            self.file.path()
        }
    }

    /// Builds a minimal 32-bit little-endian ELF with one `PT_LOAD` segment
    /// per `(physical address, file size)` pair.
    fn elf(segments: &[(u32, u32)]) -> Vec<u8> {
        let mut image = vec![0u8; 52 + segments.len() * 32];
        image[..4].copy_from_slice(b"\x7fELF");
        image[4] = 1; // ELFCLASS32
        image[5] = 1; // ELFDATA2LSB
        image[6] = 1; // EV_CURRENT
        image[0x10..0x12].copy_from_slice(&2u16.to_le_bytes()); // ET_EXEC
        image[0x12..0x14].copy_from_slice(&83u16.to_le_bytes()); // EM_AVR
        image[0x14..0x18].copy_from_slice(&1u32.to_le_bytes()); // EV_CURRENT
        image[0x1c..0x20].copy_from_slice(&52u32.to_le_bytes()); // e_phoff
        image[0x28..0x2a].copy_from_slice(&52u16.to_le_bytes()); // e_ehsize
        image[0x2a..0x2c].copy_from_slice(&32u16.to_le_bytes()); // e_phentsize
        let segment_count = u16::try_from(segments.len()).expect("test segment count fits in u16");
        image[0x2c..0x2e].copy_from_slice(&segment_count.to_le_bytes());

        for (index, (paddr, filesz)) in segments.iter().enumerate() {
            let offset = 52 + index * 32;
            image[offset..offset + 4].copy_from_slice(&1u32.to_le_bytes()); // PT_LOAD
            image[offset + 12..offset + 16].copy_from_slice(&paddr.to_le_bytes());
            image[offset + 16..offset + 20].copy_from_slice(&filesz.to_le_bytes());
            image[offset + 20..offset + 24].copy_from_slice(&filesz.to_le_bytes());
        }
        image
    }

    #[test]
    fn measures_the_highest_flash_segment() {
        let file = TestFile::new(".elf", &elf(&[(0, 20_000), (20_000, 400)]));
        assert_eq!(image_size(file.path()).unwrap(), Some(20_400));
    }

    #[test]
    fn ignores_segments_in_the_data_address_space() {
        let file = TestFile::new(".elf", &elf(&[(0, 1_000), (0x80_0100, 500)]));
        assert_eq!(image_size(file.path()).unwrap(), Some(1_000));
    }

    #[test]
    fn ignores_empty_segments() {
        let file = TestFile::new(".elf", &elf(&[(0, 1_000), (0, 0)]));
        assert_eq!(image_size(file.path()).unwrap(), Some(1_000));
    }

    #[test]
    fn reports_unknown_for_an_elf_without_load_segments() {
        let file = TestFile::new(".elf", &elf(&[]));
        assert_eq!(image_size(file.path()).unwrap(), None);
    }

    #[test]
    fn measures_raw_binaries_by_file_length() {
        let file = TestFile::new(".bin", &[0u8; 31_008]);
        assert_eq!(image_size(file.path()).unwrap(), Some(31_008));
    }

    #[test]
    fn does_not_guess_the_size_of_intel_hex_files() {
        let file = TestFile::new(".hex", b":00000001FF\n");
        assert_eq!(image_size(file.path()).unwrap(), None);
    }

    #[test]
    fn rejects_a_truncated_elf_header() {
        let file = TestFile::new(".elf", b"\x7fELF\x01");
        assert!(image_size(file.path()).is_err());
    }

    #[test]
    fn rejects_a_program_header_table_past_the_end() {
        let mut image = elf(&[(0, 100)]);
        image[0x1c..0x20].copy_from_slice(&10_000u32.to_le_bytes());
        let file = TestFile::new(".elf", &image);
        assert!(image_size(file.path()).is_err());
    }

    #[test]
    fn refuses_an_image_that_overlaps_the_bootloader() {
        let file = TestFile::new(".elf", &elf(&[(0, 31_008)]));
        let error = check_application_size(file.path(), 30_720, "Arduino Nano")
            .unwrap_err()
            .to_string();
        assert!(error.contains("31008"), "{error}");
        assert!(error.contains("30720"), "{error}");
        assert!(error.contains("Arduino Nano"), "{error}");
    }

    #[test]
    fn accepts_an_image_that_fits_exactly() {
        let file = TestFile::new(".elf", &elf(&[(0, 30_720)]));
        check_application_size(file.path(), 30_720, "Arduino Nano").unwrap();
    }

    #[test]
    fn accepts_a_smaller_image() {
        let file = TestFile::new(".elf", &elf(&[(0, 28_000)]));
        check_application_size(file.path(), 30_720, "Arduino Nano").unwrap();
    }
}
