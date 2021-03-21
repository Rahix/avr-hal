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
    pub fn run(
        options: &AvrdudeOptions,
        port: &path::Path,
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
                .context("coult not write avrdude.conf")?;
            f.flush().unwrap();
        }

        let mut command = process::Command::new("avrdude");
        let mut command = command
            .arg("-C")
            .arg(&config.as_ref())
            .arg("-c")
            .arg(options.programmer)
            .arg("-p")
            .arg(options.partno)
            .arg("-P")
            .arg(port);

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
