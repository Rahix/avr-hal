use resolve_path::PathResolveExt;
use std::thread;
use std::time::Duration;

use anyhow::Context as _;
use std::path;
use std::process;

use std::io::Write;

use crate::config::BoardAvrdudeOptions;

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
        options: &BoardAvrdudeOptions,
        port: Option<impl AsRef<path::Path>>,
        bin: &path::Path,
        debug: bool,
    ) -> anyhow::Result<Self> {
        let avrdude_version = Self::get_avrdude_version()?;

        println!("{:?}", options);

        let mut command = match options.avrdude_path.as_ref() {
            Some(avrdude_path) => {
                let avrdude_path = avrdude_path.resolve();
                // let avrdude_path = path::absolute(avrdude_path).unwrap();
                &mut process::Command::new(avrdude_path.as_os_str())
            }
            None => &mut process::Command::new("avrdude"), //     let avrdude_path = path::absolute(options.avrdude_path);
                                                           //     &mut process::Command::new(avrdude_path)
                                                           // } else {
        };

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
            .arg("-v")
            .arg("-V")
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

        if let Some(fuse2) = options.fuse2 {
            command = command.arg("-U").arg(format!("fuse2:w:{:#02X}:m", fuse2));
        }

        if let Some(fuse5) = options.fuse5 {
            command = command.arg("-U").arg(format!("fuse2:w:{:#02X}:m", fuse5));
        }

        if let Some(fuse8) = options.fuse8 {
            command = command.arg("-U").arg(format!("fuse2:w:{:#02X}:m", fuse8));
        }

        if let Some(port) = port.as_ref() {
            command = command.arg("-P").arg(port.as_ref());
        }

        if let Some(avrdude_conf) = options.avrdude_conf.as_ref() {
            let avrdude_conf = avrdude_conf.resolve();
            command = command.arg("-C").arg(avrdude_conf.as_os_str());
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

        match options.curclock {
            Some(curclock) => {
                if curclock == true {
                    command = command.arg("-curclock");
                }
            }
            None => {}
        }

        command = command.arg("-D");
        command = command.arg("-U").arg(flash_instruction);

        // Add x nometadata if no_metadata is true
        match options.no_metadata {
            Some(no_metadata) => {
                if no_metadata == true {
                    command = command.arg("-x").arg("nometadata");
                }
            }
            None => {}
        }

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

        // Reset if this is nanao-every
        match options.delay_serial {
            Some(delay_serial) => {
                if delay_serial == true {
                    if let Some(port) = port.as_ref() {
                        // Open the port
                        let serial_port = serialport::new(port.as_ref().to_str().unwrap(), 1200);
                        serial_port.open().expect("Failed to open port");

                        // Wait for half a second
                        let half_second = Duration::from_millis(500);
                        thread::sleep(half_second);
                    }
                }
            }
            None => {}
        }

        println!("{:?}", command);

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
