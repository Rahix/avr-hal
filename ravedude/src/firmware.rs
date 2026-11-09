use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::Context as _;

/// Refuses to flash an image that would not fit below the bootloader.
pub fn check_application_size(path: &Path, limit: u32) -> anyhow::Result<()> {
    let mut file =
        File::open(path).with_context(|| format!("failed opening {}", path.display()))?;
    let size = read_elf_flash_size(&mut file)
        .with_context(|| format!("failed reading flash size of {} binary", path.display()))?;

    if size > limit {
        return Err(anyhow::anyhow!(
            "firmware is too big (0x{size:x} bytes)!\n\
             board max-application-size=0x{limit:x} to not overwrite the bootloader"
        ));
    }

    Ok(())
}

fn read_elf_flash_size(file: &mut File) -> anyhow::Result<u32> {
    let mut header = [0u8; ELF_HEADER_LEN];
    file.read_exact(&mut header)
        .context("binary is too small to be an AVR ELF file")?;

    if !header.starts_with(&ELF_MAGIC) {
        anyhow::bail!("binary is not an ELF file");
    }
    if header[EI_CLASS] != ELFCLASS32
        || header[EI_DATA] != ELFDATA2LSB
        || header[EI_VERSION] != EV_CURRENT
        || read_u16(&header, E_MACHINE) != EM_AVR
    {
        anyhow::bail!("binary is not a supported AVR ELF file");
    }

    let phoff = read_u32(&header, E_PHOFF);
    let phentsize = usize::from(read_u16(&header, E_PHENTSIZE));
    let phnum = usize::from(read_u16(&header, E_PHNUM));

    if phnum == 0 {
        anyhow::bail!("binary has no program headers");
    }
    if phentsize < ELF_PROGRAM_HEADER_LEN {
        anyhow::bail!("binary has an invalid program header size ({phentsize})");
    }

    let table_len = phnum * phentsize;
    if table_len > MAX_PROGRAM_HEADER_TABLE_LEN {
        anyhow::bail!("binary has an unreasonably large program header table ({table_len} bytes)");
    }

    file.seek(SeekFrom::Start(u64::from(phoff)))
        .context("failed to seek to the program header table")?;
    let mut table = vec![0u8; table_len];
    file.read_exact(&mut table)
        .context("binary has a truncated program header table")?;

    let size = (0..phnum)
        .filter_map(|index| {
            let offset = index * phentsize;
            let entry = &table[offset..offset + ELF_PROGRAM_HEADER_LEN];
            if read_u32(entry, P_TYPE) != PT_LOAD {
                return None;
            }

            let paddr = read_u32(entry, P_PADDR);
            let filesz = read_u32(entry, P_FILESZ);
            // Data (SRAM) segments are loaded at 0x800000 and above and never
            // occupy flash.
            if filesz == 0 || paddr >= AVR_DATA_SPACE_START {
                return None;
            }

            Some(u64::from(paddr) + u64::from(filesz))
        })
        .max();

    let Some(size) = size else {
        anyhow::bail!("binary has no loadable flash segments");
    };
    u32::try_from(size).context("firmware size does not fit in 32 bits")
}

fn read_u16(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(buf[offset..offset + 2].try_into().unwrap())
}

fn read_u32(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
}

/// Highest address that still belongs to the AVR flash address space.  Data
/// (SRAM) segments live at `0x800000` and above.
const AVR_DATA_SPACE_START: u32 = 0x80_0000;

const ELF_MAGIC: [u8; 4] = *b"\x7fELF";
const ELFCLASS32: u8 = 1;
const ELFDATA2LSB: u8 = 1;
const EV_CURRENT: u8 = 1;
const EM_AVR: u16 = 83;
const PT_LOAD: u32 = 1;

// ELF header field offsets (see `elf.h`).
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const E_MACHINE: usize = 18;
const E_PHOFF: usize = 28;
const E_PHENTSIZE: usize = 42;
const E_PHNUM: usize = 44;

// Program header field offsets (ELF32).
const P_TYPE: usize = 0;
const P_PADDR: usize = 12;
const P_FILESZ: usize = 16;

const ELF_HEADER_LEN: usize = 52;
const ELF_PROGRAM_HEADER_LEN: usize = 32;
const MAX_PROGRAM_HEADER_TABLE_LEN: usize = 1024;

#[cfg(test)]
mod tests {
    use super::{E_MACHINE, EI_DATA, check_application_size, read_elf_flash_size};
    use std::io::Write;
    use std::path::Path;

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

        fn path(&self) -> &Path {
            self.file.path()
        }

        fn open(&self) -> std::fs::File {
            std::fs::File::open(self.path()).expect("open temporary file")
        }
    }

    /// Builds a minimal 32-bit little-endian AVR ELF with one `PT_LOAD`
    /// segment per `(physical address, file size)` pair.
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

    fn flash_size(contents: &[u8]) -> anyhow::Result<u32> {
        let file = TestFile::new(".elf", contents);
        read_elf_flash_size(&mut file.open())
    }

    #[test]
    fn measures_the_highest_flash_segment() {
        assert_eq!(
            flash_size(&elf(&[(0, 20_000), (20_000, 400)])).unwrap(),
            20_400
        );
    }

    #[test]
    fn ignores_segments_in_the_data_address_space() {
        assert_eq!(
            flash_size(&elf(&[(0, 1_000), (0x80_0100, 500)])).unwrap(),
            1_000
        );
    }

    #[test]
    fn ignores_empty_segments() {
        assert_eq!(flash_size(&elf(&[(0, 1_000), (0, 0)])).unwrap(), 1_000);
    }

    #[test]
    fn rejects_a_binary_without_flash_segments() {
        let error = flash_size(&elf(&[(0x80_0100, 500)]))
            .unwrap_err()
            .to_string();
        assert!(error.contains("no loadable flash segments"), "{error}");
    }

    #[test]
    fn rejects_a_non_elf_file() {
        let error = flash_size(&[0u8; 52]).unwrap_err().to_string();
        assert!(error.contains("not an ELF file"), "{error}");
    }

    #[test]
    fn rejects_a_non_avr_elf() {
        let mut image = elf(&[(0, 100)]);
        image[E_MACHINE..E_MACHINE + 2].copy_from_slice(&62u16.to_le_bytes()); // EM_X86_64
        let error = flash_size(&image).unwrap_err().to_string();
        assert!(error.contains("not a supported AVR ELF"), "{error}");
    }

    #[test]
    fn rejects_a_big_endian_elf() {
        let mut image = elf(&[(0, 100)]);
        image[EI_DATA] = 2; // ELFDATA2MSB
        let error = flash_size(&image).unwrap_err().to_string();
        assert!(error.contains("not a supported AVR ELF"), "{error}");
    }

    #[test]
    fn rejects_an_elf_without_program_headers() {
        let error = flash_size(&elf(&[])).unwrap_err().to_string();
        assert!(error.contains("no program headers"), "{error}");
    }

    #[test]
    fn rejects_a_truncated_header() {
        let error = flash_size(b"\x7fELF").unwrap_err().to_string();
        assert!(error.contains("too small"), "{error}");
    }

    #[test]
    fn rejects_a_truncated_program_header_table() {
        let mut image = elf(&[(0, 100), (0, 100)]);
        image.truncate(52 + 32);
        let error = flash_size(&image).unwrap_err().to_string();
        assert!(error.contains("truncated program header table"), "{error}");
    }

    #[test]
    fn rejects_an_unreasonably_large_program_header_table() {
        let mut image = elf(&[]);
        image[0x2c..0x2e].copy_from_slice(&1_000u16.to_le_bytes());
        let error = flash_size(&image).unwrap_err().to_string();
        assert!(error.contains("unreasonably large"), "{error}");
    }

    #[test]
    fn refuses_an_image_that_overlaps_the_bootloader() {
        let file = TestFile::new(".elf", &elf(&[(0, 31_008)]));
        let error = check_application_size(file.path(), 30_720)
            .unwrap_err()
            .to_string();
        assert!(error.contains("0x7920"), "{error}");
        assert!(error.contains("0x7800"), "{error}");
        assert!(error.contains("max-application-size"), "{error}");
    }

    #[test]
    fn accepts_an_image_that_fits_exactly() {
        let file = TestFile::new(".elf", &elf(&[(0, 30_720)]));
        check_application_size(file.path(), 30_720).unwrap();
    }

    #[test]
    fn accepts_a_smaller_image() {
        let file = TestFile::new(".elf", &elf(&[(0, 28_000)]));
        check_application_size(file.path(), 30_720).unwrap();
    }
}
