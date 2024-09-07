use std::io::Read as _;
use std::io::Write as _;

use anyhow::Context as _;
use colored::Colorize as _;

use crate::task_message;
use crate::Args;
use crate::OutputMode::*;

pub fn open(args: Args) -> anyhow::Result<()> {
    let baudrate = args
        .baudrate
        .context("-b/--baudrate is needed for the serial console")?;

    let port = args
        .port
        .context("console can only be opened for devices with USB-to-Serial")?;

    task_message!("Console", "{} at {} baud", port.display(), baudrate);
    task_message!(
        "Output",
        "{}",
        match args.output_mode {
            Some(Ascii) | None => "ascii",
            Some(Hex) => "hexadecimal",
            Some(Dec) => "decimal",
            Some(Bin) => "binary",
        }
    );
    task_message!("", "{}", "CTRL+C to exit.".dimmed());
    // Empty line for visual consistency
    eprintln!();

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

    let newline_after = match args.newline_after {
        Some(n) => n,
        None => match args.output_mode {
            Some(Hex) | Some(Dec) => 16,
            Some(Bin) => 8,
            _ => 0,
        },
    };

    let (spaces, space_after) = if args.newline_on.is_none() && newline_after % 4 == 0 {
        (true, 4)
    } else {
        (false, 0)
    };

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
                match args.output_mode {
                    Some(Ascii) | None => {
                        stdout.write_all(&buf).unwrap();
                    }
                    _ => {
                        for byte in &buf[..count] {
                            byte_count += 1;
                            match args.output_mode {
                                Some(Ascii) | None => unreachable!(),
                                Some(Hex) => {
                                    write!(stdout, "{:02x} ", byte).unwrap();
                                }
                                Some(Dec) => {
                                    write!(stdout, "{:03} ", byte).unwrap();
                                }
                                Some(Bin) => {
                                    write!(stdout, "{:08b} ", byte).unwrap();
                                }
                            }
                            // don’t execute in ascii mode, ascii is unreachable here
                            if spaces && byte_count % space_after == 0 {
                                write!(stdout, " ").unwrap();
                            }
                            if args.newline_on.is_none() && byte_count % newline_after == 0 {
                                writeln!(stdout).unwrap();
                            }
                            if args.newline_on.is_some()
                                && *byte as char == args.newline_on.unwrap()
                            {
                                writeln!(stdout).unwrap();
                            }
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
