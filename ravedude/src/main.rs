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
use clap::Parser;
use colored::Colorize as _;
use std::num::NonZero;

use std::path::Path;
use std::thread;
use std::time::Duration;

mod avrdude;
mod board;
mod config;
mod console;
mod target_detect;
mod ui;

/// This represents the minimum (Major, Minor) version raverdude requires avrdude to meet.
const MIN_VERSION_AVRDUDE: (u8, u8) = (6, 3);

// ravedude has two subcommands: the `board` subcommand, which flashes a binary to a board, and a
// `chip` command, which flashes a binary to a chip. The differ in that
// * The `chip` command determines the target chip from the elf binary metadata, where as the
//   `board` command requires that you specify the board by using a TOML config file
// * The `chip` command allows more of its options to be specified through environment variables,
//   whereas the board command reads its options from the TOML config and command-line arguments

// A unification between the two is desirable, but I am keeping them separate initially in order to
// maintain backwards compatibility in `board` while doing new development in `chip`.

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
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Parser, Debug)]
enum Subcommand {
    Board(BoardArgs),
    Chip(ChipArgs),
}

/// High-level flashing command. Use this if you are working on a standalone binary that will be
/// flashed to a specific off-the shelf board, such as Arduino Uno, Adafruit Trinket, or Arduino Pro
/// Mini.
#[derive(clap::Parser, Debug)]
struct BoardArgs {
    /// Utility flag for dumping a config of a named board to TOML.
    #[clap(long = "dump-config")]
    dump_config: bool,

    /// After successfully flashing the program, open a serial console to see output sent by the
    /// board and possibly interact with it.
    #[clap(short = 'c', long = "open-console")]
    open_console: bool,

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
}

#[derive(clap::Parser, clap::ValueEnum, Debug, Clone)]
enum ConsoleMode {
    /// Do not connect to the chip after flashing the binary
    None,
    /// Connect to the chip using a plain serial console
    Plain,
}

impl std::fmt::Display for ConsoleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConsoleMode::None => write!(f, "none"),
            ConsoleMode::Plain => write!(f, "plain"),
        }
    }
}

/// Low-level flashing command. Use this if you are flashing to a bare chip or a chip on a custom board.
#[derive(clap::Parser, Debug)]
struct ChipArgs {
    /// The name of the AVR programmer you are using. Run `avrdude "-c?"` for the full list.
    #[clap(long = "programmer", env = "AVR_PROGRAMMER_NAME")]
    programmer_name: Option<String>,

    /// The serial port used to connect to the programmer. Autodetected when possible if unspecified.
    #[clap(long = "programmer-port", env = "AVR_PROGRAMMER_PORT", value_parser)]
    programmer_port: Option<std::path::PathBuf>,

    /// The baudrate used to connect to the programmer. Autodetected when possible if unspecified.
    #[clap(long = "programmer-baudrate", env = "AVR_PROGRAMMER_BAUDRATE")]
    programmer_baudrate: Option<NonZero<u32>>,

    /// If set, erase the target before flashing.
    #[clap(
        long = "erase-target",
        env = "AVR_PROGRAMMER_ERASE_TARGET",
        default_value_t = true
    )]
    erase_target: bool,

    /// Serial connection mode, used to connect to the chip after flashing the binary
    #[clap(
        long = "console-mode",
        env = "AVR_CONSOLE_MODE",
        default_value_t = ConsoleMode::None
    )]
    console_mode: ConsoleMode,

    /// The serial port used to connect to the chip after flashing, if different from the programmer serial port.
    #[clap(long = "console-port", env = "AVR_CONSOLE_PORT", value_parser)]
    console_port: Option<std::path::PathBuf>,

    /// The baudrate used to connect to the chip after flashing, if different from the programmer baudrate.
    #[clap(short = 'b', long = "console-baudrate", env = "AVR_CONSOLE_BAUDRATE")]
    console_baudrate: Option<NonZero<u32>>,

    /// Print verbose information
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    #[clap(value_parser)]
    /// The binary to be flashed.
    binary: std::path::PathBuf,
}

impl BoardArgs {
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
    let mut args: Args = match Args::try_parse() {
        Ok(args) => args,
        Err(_) => {
            // If no command is specified, try parsing as just `BoardArgs`
            match BoardArgs::try_parse() {
                Ok(boardArgs) => Args {
                    subcommand: Subcommand::Board(boardArgs),
                },
                // If `BoardArgs` parsing doesn't work either, go back to parsing as `Args` to force the default help to be printed
                Err(_) => Args::parse(),
            }
        }
    };

    match &mut args.subcommand {
        Subcommand::Board(args) => ravedude_board(args),
        Subcommand::Chip(ref mut args) => ravedude_chip(args),
    }
}

fn ravedude_board(args: &BoardArgs) -> anyhow::Result<()> {
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
        Some(port) => Ok(Some(port)),
        None => match board.guess_port() {
            Some(Ok(port)) => Ok(Some(port)),
            p @ Some(Err(_)) => p.transpose().context(
                "no matching serial port found, use -P, add a serial-port entry under [general] in Ravedude.toml, or set RAVEDUDE_PORT in your environment",
            ),
            None => Ok(None),
        },
    }?;

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

        let port = port.context("console can only be opened for devices with USB-to-Serial")?;

        task_message!("Console", "{} at {} baud", port.display(), baudrate);
        task_message!("", "{}", "CTRL+C to exit.".dimmed());
        // Empty line for visual consistency
        eprintln!();
        console::open(&port, baudrate.get())?;
    } else if args.bin.is_none() && port.is_some() {
        warning!("you probably meant to add -c/--open-console?");
    }

    Ok(())
}

fn ravedude_chip(args: &mut ChipArgs) -> anyhow::Result<()> {
    avrdude::Avrdude::require_min_ver(MIN_VERSION_AVRDUDE)?;

    // Some programmers require an explicit port, and we could hardcode that here for a better error message.
    if let Some(port) = args.programmer_port.as_ref() {
        task_message!(
            "Programming",
            "{} {} {}",
            args.binary.display(),
            "=>".blue().bold(),
            port.display()
        );
    } else {
        task_message!("Programming", "{}", args.binary.display(),);
    }

    let target = target_detect::target_name_from_binary(&args.binary)?;

    let avrdude_options = config::BoardAvrdudeOptions {
        programmer: args.programmer_name.clone(),
        partno: Some(target),
        baudrate: Some(args.programmer_baudrate),
        do_chip_erase: Some(args.erase_target),
    };

    let mut avrdude = avrdude::Avrdude::run(
        &avrdude_options,
        args.programmer_port.as_ref(),
        &args.binary,
        args.verbose,
    )?;
    avrdude.wait()?;

    task_message!("Programmed", "{}", args.binary.display());

    match args.console_mode {
        ConsoleMode::None => Ok(()),
        ConsoleMode::Plain => {
            let port = args.console_port.take().or(args.programmer_port.take());
            let baudrate = args
                .console_baudrate
                .take()
                .or(args.programmer_baudrate.take());

            match (port, baudrate) {
                (Some(port), Some(baudrate)) => {
                    task_message!("Console", "{} at {} baud", port.display(), baudrate);
                    task_message!("", "{}", "CTRL+C to exit.".dimmed());
                    // Empty line for visual consistency
                    eprintln!();
                    console::open(&port, baudrate.get())
                }

                _ => Err(anyhow::anyhow!(
                    "Console port and baudrate have to be specified."
                )),
            }
        }
    }?;

    Ok(())
}
