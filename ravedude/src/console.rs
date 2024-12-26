use anyhow::Context as _;
use std::io::Read as _;
use std::io::Write as _;

pub fn open(port: &std::path::Path, baudrate: u32) -> anyhow::Result<()> {
    let mut rx = serialport::new(port.to_string_lossy(), baudrate)
        .timeout(std::time::Duration::from_secs(2))
        .open_native()
        .with_context(|| format!("failed to open serial port `{}`", port.display()))?;
    let mut tx = rx.try_clone_native()?;

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // Set a CTRL+C handler to terminate cleanly instead of with an error.
    ctrlc::set_handler(move || {
        eprintln!("");
        eprintln!("Exiting.");
        std::process::exit(0);
    })
    .context("failed setting a CTRL+C handler")?;

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
                stdout.write(&buf[..count]).unwrap();
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
