use crate::config::BoardAvrdudeOptions;
use anyhow::Context as _;
use colored::Colorize;
use std::io::Write;
use std::path;
use std::process;

#[derive(Debug)]
pub struct Avrdude {
    #[allow(dead_code)]
    config: Option<tempfile::NamedTempFile>,
    process: process::Child,
}

impl Avrdude {
    pub fn avrdude_installed() -> anyhow::Result<()> {
        let dude_output = process::Command::new("avrdude")
            .arg("-?")
            .stderr(std::process::Stdio::null())
            .status();
        let gcc_output = process::Command::new("avr-gcc")
            .arg("-v")
            .stderr(std::process::Stdio::null())
            .status();
        if dude_output.is_err() || gcc_output.is_err() {
            // Response should specify wrong version,
            // but then platform specific advice on
            // what to use instead.
            let default_error = format!(
                "{} and/or {} is not installed.\n\
                Please install the latest version, in accordance with the below instructions.\n\
                For more help, visit https://github.com/Rahix/avr-hal/wiki/Setting-up-environment. \n",
                "Avrdude".bright_black(),
                "avr-gcc".bright_black()
            );
            #[cfg(target_os = "windows")]
            {
                let winget = terminal_link::Link::new(
                    "winget",
                    "https://learn.microsoft.com/en-us/windows/package-manager/winget/",
                );
                let scoop = terminal_link::Link::new("scoop", "https://scoop.sh/");
                anyhow::bail!(
                    "{} \n\
                    Use {} on Windows 10 and 11: {}\n\
                    On older systems, install and use {}: {}",
                    default_error,
                    winget,
                    "winget install AVRDudes.AVRDUDE ZakKemble.avr-gcc".bright_black(),
                    scoop,
                    "scoop install avrdude avr-gcc".bright_black(),
                );
            }
            #[cfg(target_os = "macos")]
            {
                anyhow::bail!(
                    "{} \n\
                    Use the following commands:\n\
                    {}",
                    default_error,
                    " xcode-select --install\n \
                    brew tap osx-cross/avr\n \
                    brew install avr-gcc avrdude"
                        .bright_black()
                );
            }
            #[cfg(target_os = "linux")]
            {
                anyhow::bail!(
                    "{} \n\
                    Use the following command: {}", 
                    default_error,
                    "sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential".bright_black()
            );
            }
            #[allow(unreachable_code)]
            {
                anyhow::bail!(
                    "{} \n\
                    You don't seem to be on a (directly) supported platform.\n\
                    Please confirm you are using one of the following platforms: Windows, Linux, or macOS.\n\
                    For more help, visit https://github.com/Rahix/avr-hal/wiki/Setting-up-environment.",
                    default_error,
                )
            }
        }

        Ok(())
    }

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
        options: &BoardAvrdudeOptions,
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
            .arg(
                options
                    .programmer
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("board has no programmer"))?,
            )
            .arg("-p")
            .arg(
                options
                    .partno
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("board has no part number"))?,
            );

        if let Some(port) = port {
            command = command.arg("-P").arg(port.as_ref());
        }

        if let Some(baudrate) = options.baudrate.flatten() {
            command = command.arg("-b").arg(baudrate.to_string());
        }

        // TODO: Check that `bin` does not contain :
        let mut flash_instruction: std::ffi::OsString = "flash:w:".into();
        flash_instruction.push(bin);
        flash_instruction.push(":e");

        if options
            .do_chip_erase
            .ok_or_else(|| anyhow::anyhow!("board doesn't specify whether to erase the chip"))?
        {
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
