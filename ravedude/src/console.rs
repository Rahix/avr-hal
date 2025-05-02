use std::io::Read as _;
use std::io::Write as _;

use anyhow::Context as _;

use crate::config::NewlineMode;
use crate::config::OutputMode;
use crate::config::OutputMode::*;

pub fn open(
    port: &std::path::PathBuf,
    baudrate: u32,
    output_mode: OutputMode,
    newline_mode: NewlineMode,
    space_after: Option<u8>,
) -> anyhow::Result<()> {
    let mut rx = serialport::new(port.to_string_lossy(), baudrate)
        .timeout(std::time::Duration::from_secs(2))
        .open_native()
        .with_context(|| format!("failed to open serial port `{}`", port.display()))?;
    let mut tx = rx.try_clone_native()?;

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // Set a CTRL+C handler to terminate cleanly instead of with an error.
    ctrlc::set_handler(move || {
        eprintln!();
        eprintln!("Exiting.");
        std::process::exit(0);
    })
    .context("failed setting a CTRL+C handler")?;

    let mut byte_count = 0;

    // Spawn a thread for the receiving end because stdio is not portably non-blocking...
    std::thread::spawn(move || loop {
        #[cfg(not(target_os = "windows"))]
        let mut buf = [0u8; 4098];

        // Use buffer size 1 for windows because it blocks on rx.read until the buffer is full
        #[cfg(target_os = "windows")]
        let mut buf = [0u8; 1];

        match rx.read(&mut buf) {
            Ok(count) => {
                #[cfg(target_os = "windows")]
                {
                    // On windows, we must ensure that we are not sending anything outside of the
                    // ASCII range.
                    for byte in &mut buf[..count] {
                        if *byte & 0x80 != 0 {
                            *byte = '?'.try_into().unwrap();
                        }
                    }
                }
                if output_mode == Ascii {
                    stdout.write(&buf[..count]).unwrap();
                } else {
                    for byte in &buf[..count] {
                        byte_count += 1;
                        match output_mode {
                            Ascii => unreachable!(),
                            Hex => write!(stdout, "{:02x} ", byte).unwrap(),
                            Dec => write!(stdout, "{:03} ", byte).unwrap(),
                            Bin => write!(stdout, "{:08b} ", byte).unwrap(),
                        }

                        if let Some(space_after) = space_after {
                            if byte_count % space_after == 0 {
                                write!(stdout, " ").unwrap();
                            }
                        }
                        match newline_mode {
                            NewlineMode::On(newline_on) => {
                                if *byte == newline_on {
                                    writeln!(stdout).unwrap()
                                }
                            }
                            NewlineMode::After(newline_after) => {
                                if byte_count % newline_after == 0 {
                                    writeln!(stdout).unwrap();
                                }
                            }
                            NewlineMode::Off => {}
                        }
                    }
                }
                stdout.flush().unwrap();
            }
            Err(e) => {
                assert!(e.kind() == std::io::ErrorKind::TimedOut);
            }
        }
    });

    loop {
        let mut buf = [0u8; 4098];
        let count = stdin.read(&mut buf)?;
        tx.write(&buf[..count])?;
        tx.flush()?;
    }
}
