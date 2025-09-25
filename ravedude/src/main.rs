//! # ravedude
//! `ravedude` is a CLI utility to make Rust development for AVR microcontrollers
//! super smooth.  It's a wrapper around `avrdude` and provides easy access to the
//! target's serial console, similar to the Arduino IDE.
//!
//! `ravedude` is meant to be used as a cargo "runner".  This allows you to just use
//! `cargo run` for building, deploying, and running your AVR code!
//!
//! # Installation
//! On Linux systems, you'll need pkg-config and libudev development files
//! installed:
//!
//! - *Archlinux*: `pacman -S systemd pkgconf`
//! - *Ubuntu/Debian*: `apt install libudev-dev pkg-config`
//! - *Fedora*: `dnf install systemd-devel pkgconf-pkg-config`
//!
//! Next, install the latest version from crates.io with the following command:
//!
//! ```text
//! cargo +stable install --locked ravedude
//! ```
//!
//! # Usage
//! To use ravedude in your project, add the following to your project's `.cargo/config.toml`:
//!
//! ```text
//! [target.'cfg(target_arch = "avr")']
//! runner = "ravedude"
//! ```
//!
//! Then, create a `Ravedude.toml` file next to your `Cargo.toml`.  This is the configuration file
//! for ravedude.  For a simple Arduino Uno, you can use the following config:
//!
//! ```text
//! [general]
//! board = "uno"
//! serial-baudrate = 57600
//! open-console = true
//! ```
//!
//! # Configuration
//! For off-the-shelf AVR boards that are already supported by ravedude, configuration is very
//! simple.  Just two lines in `Ravedude.toml` are all that is necessary:
//!
//! ```text
//! [general]
//! board = "<board-name-here>"
//! ```
//!
//! Depending on your project, you may want to add any of the following additional options:
//!
//! ```text
//! [general]
//! # if auto-detection is not working, you can hard-code a specific port here
//! # (the port can also be passed via the RAVEDUDE_PORT environment variable)
//! port = "/dev/ttyACM0"
//!
//! # ravedude should open a serial console after flashing
//! open-console = true
//!
//! # baudrate for the serial console (this is **not** the avrdude flashing baudrate)
//! serial-baudrate = 57600
//!
//! # time to wait for the board to be reset (in milliseconds).  this skips the manual prompt for resetting the board.
//! reset-delay = 2000
//! ```
//!
//! # Custom Boards
//! For boards that are not yet part of _ravedude_, you can specify all relevant options yourself
//! in `Ravedude.toml`.  It works like this:
//!
//! ```text
//! [general]
//! # port = ...
//! # open-console = true
//! # serial-baudrate = 57600
//!
//! [board]
//! name = "Custom Arduino Uno"
//!
//! [board.reset]
//! # The board automatically resets when attempting to flash
//! automatic = true
//!
//! [board.avrdude]
//! # avrdude configuration
//! programmer = "arduino"
//! partno = "atmega328p"
//! baudrate = -1
//! do-chip-erase = true
//! ```
//!
//! For reference, take a look at [`boards.toml`](https://github.com/Rahix/avr-hal/blob/main/ravedude/src/boards.toml).
use anyhow::Context as _;
use colored::Colorize as _;
use config::OutputMode;

use std::path::Path;
use std::thread;
use std::time::Duration;

mod avrdude;
mod board;
mod config;
mod console;
mod ui;

/// This represents the minimum (Major, Minor) version raverdude requires avrdude to meet.
const MIN_VERSION_AVRDUDE: (u8, u8) = (6, 3);

/// ravedude is a rust wrapper around avrdude for providing the smoothest possible development
/// experience with rust on AVR microcontrollers.
///
/// ravedude is primarily intended to be used as a "runner" in the cargo configuration.
#[derive(clap::Parser, Debug)]
#[clap(name = "ravedude",
    version = git_version::git_version!(
        args = ["--always", "--dirty", "--abbrev=12"],
        cargo_prefix = "v",
        cargo_suffix = " (no git)",
        fallback = "unknown"
    ))]
struct Args {
    /// Utility flag for dumping a config of a named board to TOML.
    #[clap(long = "dump-config")]
    dump_config: bool,

    /// After successfully flashing the program, open a serial console to see output sent by the
    /// board and possibly interact with it.
    #[clap(short = 'c', long = "open-console")]
    open_console: bool,

    #[clap(
        short = 'C',
        long = "console-port",
        value_parser,
        env = "RAVEDUDE_CONSOLE_PORT"
    )]
    console_port: Option<std::path::PathBuf>,

    /// Baudrate which should be used for the serial console.
    #[clap(short = 'b', long = "baudrate")]
    baudrate: Option<u32>,

    /// Overwrite which port to use. By default ravedude will try to find a connected board by
    /// itself.
    #[clap(short = 'P', long = "port", value_parser, env = "RAVEDUDE_PORT")]
    port: Option<std::path::PathBuf>,

    /// This assumes the board is already resetting.
    /// Instead of giving the reset instructions and waiting for user confirmation, we wait the amount in milliseconds before proceeding.
    /// Set this value to 0 to skip the board reset question instantly.
    #[clap(short = 'd', long = "reset-delay")]
    reset_delay: Option<u64>,

    /// Print the avrdude command that is executed for flashing the binary.
    #[clap(long = "debug-avrdude")]
    debug_avrdude: bool,

    #[clap(name = "BINARY", value_parser)]
    /// The binary to be flashed.
    ///
    /// If no binary is given, flashing will be skipped.
    // (Note: this is where the board is stored in legacy configurations.)
    bin: Option<std::path::PathBuf>,

    /// Deprecated binary for old configurations of ravedude without `Ravedude.toml`.
    /// Should not be used in newer configurations.
    #[clap(name = "LEGACY BINARY", value_parser)]
    bin_legacy: Option<std::path::PathBuf>,

    /// Output mode.
    /// Can be ascii, hex, dec or bin
    #[clap(short = 'o', long = "output-mode")]
    output_mode: Option<OutputMode>,
}

impl Args {
    /// Get the board name for legacy configurations.
    /// `None` if the configuration isn't a legacy configuration or the board name doesn't exist.
    fn legacy_board_name(&self) -> Option<String> {
        if self.bin_legacy.is_none() {
            None
        } else {
            self.bin
                .as_deref()
                .and_then(|board| board.to_str().map(String::from))
        }
    }

    /// Get the binary argument with fallback for the legacy menchanism.
    ///
    /// Returns `None` if no binary argument was passed.
    fn bin_or_legacy_bin(&self) -> Option<&std::path::Path> {
        self.bin_legacy
            .as_ref()
            .map(|p| p.as_path())
            .or(self.bin.as_ref().map(|p| p.as_path()))
    }
}

fn main() {
    match ravedude() {
        Ok(()) => (),
        Err(e) => {
            ui::print_error(e);
            std::process::exit(1);
        }
    }
}

/// Finds the location of a `Ravedude.toml` or `None` if not found.
fn find_manifest() -> anyhow::Result<Option<std::path::PathBuf>> {
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let path = Path::new(&manifest_dir).join("Ravedude.toml");
        return Ok(path.exists().then_some(path));
    }

    // If `CARGO_MANIFEST_DIR` isn't set, Cargo scans the current dir and all of its parents for Cargo.toml
    // so we mirror its behavior here.
    let current_dir = std::env::current_dir()?;

    for dir_to_test in current_dir.ancestors() {
        let path_to_test = dir_to_test.join("Ravedude.toml");
        if path_to_test.exists() {
            return Ok(Some(path_to_test));
        }
    }

    Ok(None)
}

fn ravedude() -> anyhow::Result<()> {
    let args: Args = clap::Parser::parse();

    let manifest_path = find_manifest()?;

    let mut ravedude_config = match (manifest_path.as_deref(), args.legacy_board_name()) {
        (Some(_), Some(board_name)) => {
            anyhow::bail!("can't pass board as command-line argument when Ravedude.toml is present; set `board = {:?}` under [general] in Ravedude.toml", board_name);
        }
        (Some(path), None) => board::get_board_from_manifest(path)?,
        (None, Some(board_name)) => {
            warning!(
                "Passing the board as command-line argument is deprecated; create a Ravedude.toml in the project root instead:"
            );
            eprintln!(
                "\n# Ravedude.toml\n{}",
                toml::to_string(&config::RavedudeConfig::from_args(&args)?)?
            );

            board::get_board_from_name(&board_name)?
        }
        (None, None) => {
            anyhow::bail!("couldn't find Ravedude.toml in project");
        }
    };

    ravedude_config
        .general_options
        .apply_overrides_from(&args)?;

    if args.dump_config {
        println!("{}", toml::to_string(&ravedude_config)?);
        return Ok(());
    }

    avrdude::Avrdude::avrdude_installed()?;
    avrdude::Avrdude::require_min_ver(MIN_VERSION_AVRDUDE)?;

    let Some(mut board) = ravedude_config.board_config else {
        anyhow::bail!("no named board given and no board options provided");
    };

    let board_avrdude_options = board
        .avrdude
        .take()
        .ok_or_else(|| anyhow::anyhow!("board has no avrdude options"))?;

    task_message!(
        "Board",
        "{}",
        &board.name.as_deref().unwrap_or("Unnamed Board")
    );

    let port = match ravedude_config.general_options.port {
        Some(ref port) => Ok(Some(port.clone())),
        None => match board.guess_port() {
            Some(Ok(port)) => Ok(Some(port)),
            p @ Some(Err(_)) => p.transpose().context(
                "no matching serial port found, use -P, add a serial-port entry under [general] in Ravedude.toml, or set RAVEDUDE_PORT in your environment",
            ),
            None => Ok(None),
        },
    }?;

    let console_port = ravedude_config.general_options.console_port.clone();

    if let Some(bin) = args.bin_or_legacy_bin() {
        if let Some(wait_time) = args.reset_delay {
            if wait_time > 0 {
                println!("Waiting {} ms before proceeding", wait_time);
                let wait_time = Duration::from_millis(wait_time);
                thread::sleep(wait_time);
            } else {
                println!("Assuming board has been reset");
            }
        } else if matches!(board.reset, Some(config::ResetOptions { automatic: false })) {
            warning!("this board cannot reset itself.");
            eprintln!();
            eprint!("Once reset, press ENTER here: ");
            std::io::stdin().read_line(&mut String::new())?;
        }

        if let Some(port) = port.as_ref() {
            task_message!(
                "Programming",
                "{} {} {}",
                bin.display(),
                "=>".blue().bold(),
                port.display()
            );
        } else {
            task_message!("Programming", "{}", bin.display(),);
        }

        let mut avrdude = avrdude::Avrdude::run(
            &board_avrdude_options,
            port.as_ref(),
            bin,
            args.debug_avrdude,
        )?;
        avrdude.wait()?;

        task_message!("Programmed", "{}", bin.display());
    } else {
        task_message!(
            "",
            "{}",
            "(Skip flashing because no binary was given)".dimmed()
        );
    }

    if ravedude_config.general_options.open_console {
        let baudrate = ravedude_config
            .general_options
            .serial_baudrate
            .context(if manifest_path.is_some() {
                "`serial-baudrate` under [general] in Ravedude.toml is needed for the serial console"
            }else{
                "-b/--baudrate is needed for the serial console"
            })?;

        let port = console_port
            .or_else(|| port)
            .context("console can only be opened for devices with USB-to-Serial")?;
        let newline_mode = ravedude_config.general_options.newline_mode()?;

        task_message!("Console", "{} at {} baud", port.display(), baudrate);
        task_message!("", "{}", "CTRL+C to exit.".dimmed());
        // Empty line for visual consistency
        eprintln!();
        console::open(
            &port,
            baudrate.get(),
            ravedude_config.general_options.output_mode,
            newline_mode,
            newline_mode.space_after(),
        )?;
    } else if args.bin.is_none() && port.is_some() {
        warning!("you probably meant to add -c/--open-console?");
    }

    Ok(())
}
