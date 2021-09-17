use anyhow::Context as _;
use std::path;
use std::process;

#[derive(Debug)]
pub struct AvrdudeOptions<'a> {
    pub programmer: &'a str,
    pub partno: &'a str,
    pub baudrate: Option<u32>,
    pub do_chip_erase: bool,
}

#[derive(Debug)]
pub struct Avrdude {
    config: tempfile::NamedTempFile,
    process: process::Child,
}

impl Avrdude {
    fn get_avrdude_version() -> anyhow::Result<(u8, u8)> {
        let output = process::Command::new("avrdude").arg("-?").output()?;
        let stderr: &str = std::str::from_utf8(&output.stderr)?;
        let err = || anyhow::anyhow!("failed to derive version number from avrdude");
        let version: &str = stderr.split("version").last().ok_or_else(err)?.trim();
        let version: String = version
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let (major, minor) = version.split_once('.').ok_or_else(err)?;
        let major = major.parse::<u8>()?;
        let minor = minor.parse::<u8>()?;
        Ok((major, minor))
    }

    pub fn require_min_ver((req_major, req_minor): (u8, u8)) -> anyhow::Result<()> {
        let (major, minor) =
            Self::get_avrdude_version().context("Failed reading avrdude version information.")?;
        if (major, minor) < (req_major, req_minor) {
            anyhow::bail!(
                "Avrdude does not meet minimum version requirements. v{}.{} was found while v{}.{} or greater is required.\n\
                You may find a more suitable version here: https://download.savannah.gnu.org/releases/avrdude/",
                major, minor,
                req_major, req_minor,
            );
        }
        Ok(())
    }

    pub fn run(
        options: &AvrdudeOptions,
        port: Option<impl AsRef<path::Path>>,
        bin: &path::Path,
    ) -> anyhow::Result<Self> {
        let config = tempfile::Builder::new()
            .prefix(".avrdude-")
            .suffix(".conf")
            .tempfile()
            .unwrap();

        {
            use std::io::Write;

            let mut f = std::fs::File::create(&config).context("could not create avrdude.conf")?;
            f.write_all(include_bytes!("avrdude.conf"))
                .context("could not write avrdude.conf")?;
            f.flush().unwrap();
        }

        let mut command = process::Command::new("avrdude");
        let mut command = command
            .arg("-C")
            .arg(&config.as_ref())
            .arg("-c")
            .arg(options.programmer)
            .arg("-p")
            .arg(options.partno);

        if let Some(port) = port {
            command = command.arg("-P").arg(port.as_ref());
        }

        if let Some(baudrate) = options.baudrate {
            command = command.arg("-b").arg(baudrate.to_string());
        }

        // TODO: Check that `bin` does not contain :
        let mut flash_instruction: std::ffi::OsString = "flash:w:".into();
        flash_instruction.push(bin);
        flash_instruction.push(":e");

        if options.do_chip_erase {
            command = command.arg("-e");
        }

        command = command.arg("-D").arg("-U").arg(flash_instruction);

        let process = command.spawn().context("failed starting avrdude")?;

        Ok(Self { config, process })
    }

    pub fn wait(&mut self) -> anyhow::Result<()> {
        let ret = self.process.wait()?;
        if !ret.success() {
            anyhow::bail!("avrdude failed");
        }
        Ok(())
    }
}
