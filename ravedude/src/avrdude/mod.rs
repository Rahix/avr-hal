use anyhow::Context as _;
use std::path;
use std::process;

use std::io::Write;

#[derive(Debug)]
pub struct AvrdudeOptions<'a> {
    pub programmer: &'a str,
    pub partno: &'a str,
    pub baudrate: Option<u32>,
    pub do_chip_erase: bool,
}

#[derive(Debug)]
pub struct Avrdude {
    #[allow(dead_code)]
    config: Option<tempfile::NamedTempFile>,
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
        let mut version_splits = version.split('.');
        let major = version_splits.next().ok_or_else(err)?.parse::<u8>()?;
        let minor = version_splits.next().ok_or_else(err)?.parse::<u8>()?;
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
        debug: bool,
    ) -> anyhow::Result<Self> {
        let avrdude_version = Self::get_avrdude_version()?;

        let mut command = &mut process::Command::new("avrdude");

        let config = if avrdude_version <= (7, 0) {
            let config = tempfile::Builder::new()
                .prefix(".avrdude-")
                .suffix(".conf")
                .tempfile()
                .unwrap();
            let mut f = std::fs::File::create(&config).context("could not create avrdude.conf")?;
            f.write_all(include_bytes!("avrdude-6.conf"))
                .context("could not write avrdude.conf for avrdude <=7.0")?;
            f.flush().unwrap();

            command = command.arg("-C").arg(&config.as_ref());
            Some(config)
        } else {
            // For avrdude versions >=7.1, we don't supply a custom configuration file for now.
            None
        };

        let mut command = command
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

        if debug {
            crate::task_message!(
                "Dbg.Command",
                "{} {}",
                command.get_program().to_string_lossy(),
                command
                    .get_args()
                    .map(|s| s.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }

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
