use anyhow::Context as _;
use std::path;
use std::path::PathBuf;
use std::process;

#[derive(Debug)]
pub struct AvrdudeOptions<'a> {
    pub programmer: &'a str,
    pub partno: &'a str,
    pub baudrate: Option<u32>,
    pub do_chip_erase: bool,
    pub fuse2: Option<u8>,
    pub fuse5: Option<u8>,
    pub fuse8: Option<u8>,
}

#[derive(Debug)]
pub struct Avrdude {
    #[allow(dead_code)]
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
        avrdude_root: Option<std::path::PathBuf>,
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

        // Set the binary to be the default or <avrdude_root>/bin/avrdude
        println!("We are here 2 {:?}", options);

        let mut bin_command = PathBuf::from("avrdude");
        if let Some(avrdude_root) = &avrdude_root {
            let mut new_path_buf = avrdude_root.clone();
            new_path_buf.extend(&["bin", "avrdude"]);
            bin_command = new_path_buf.to_path_buf();
        }

        let mut command = process::Command::new(bin_command.into_os_string());
        let mut command = command
            .arg("-v")
            .arg("-V")
            .arg("-c")
            .arg(options.programmer)
            .arg("-p")
            .arg(options.partno);

        if let Some(avrdude_root) = &avrdude_root {
    
            if let Some(fuse2) = options.fuse2 {
                command = command.arg("-U").arg( format!("fuse2:w:{:#02X}:m",fuse2));
            }

            if let Some(fuse5) = options.fuse5 {
                command = command.arg("-U").arg( format!("fuse2:w:{:#02X}:m",fuse5));
            }

            if let Some(fuse8) = options.fuse8 {
                command = command.arg("-U").arg( format!("fuse2:w:{:#02X}:m",fuse8));
            }

            if let Some(port) = port {
                command = command.arg("-P").arg(port.as_ref());
            }

            let mut new_path_buf = avrdude_root.clone();
            new_path_buf.extend(&["etc", "avrdude.conf"]);
            let conf_command = new_path_buf.to_path_buf();
            
            command = command.arg("-C").arg(conf_command)

        }
        else {
            command = command.arg("-C").arg(&config.as_ref())
        }

        if let Some(baudrate) = options.baudrate {
            command = command.arg("-b").arg(baudrate.to_string());
        }

        println!("{:?}", command);

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
