use anyhow::Context as _;
use colored::Colorize as _;
use structopt::clap::AppSettings;

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
#[derive(structopt::StructOpt, Debug)]
#[structopt(name = "ravedude",
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::DeriveDisplayOrder,
    version = git_version::git_version!(
        args = ["--always", "--dirty", "--abbrev=12"],
        cargo_prefix = "v",
        cargo_suffix = " (no git)",
        fallback = "unknown"
    ))]
struct Args {
    /// Utility flag for dumping a config of a named board to TOML.
    #[structopt(long = "dump-config")]
    dump_config: bool,

    /// After successfully flashing the program, open a serial console to see output sent by the
    /// board and possibly interact with it.
    #[structopt(short = "c", long = "open-console")]
    open_console: bool,

    /// Baudrate which should be used for the serial console.
    #[structopt(short = "b", long = "baudrate")]
    baudrate: Option<u32>,

    /// Overwrite which port to use. By default ravedude will try to find a connected board by
    /// itself.
    #[structopt(short = "P", long = "port", parse(from_os_str), env = "RAVEDUDE_PORT")]
    port: Option<std::path::PathBuf>,

    /// This assumes the board is already resetting.
    /// Instead of giving the reset instructions and waiting for user confirmation, we wait the amount in milliseconds before proceeding.
    /// Set this value to 0 to skip the board reset question instantly.
    #[structopt(short = "d", long = "reset-delay")]
    reset_delay: Option<u64>,

    /// Print the avrdude command that is executed for flashing the binary.
    #[structopt(long = "debug-avrdude")]
    debug_avrdude: bool,

    /// Which board to interact with.
    ///
    /// Must be one of the known board identifiers:
    ///
    /// * uno
    /// * nano
    /// * nano-new
    /// * leonardo
    /// * micro
    /// * mega2560
    /// * mega1280
    /// * diecimila
    /// * promicro
    /// * promini-3v3
    /// * promini-5v
    /// * trinket-pro
    /// * trinket
    /// * nano168
    /// * duemilanove
    #[structopt(name = "BOARD", verbatim_doc_comment)]
    board: Option<String>,

    /// The binary to be flashed.
    ///
    /// If no binary is given, flashing will be skipped.
    #[structopt(name = "BINARY", parse(from_os_str))]
    bin: Option<std::path::PathBuf>,
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

fn ravedude() -> anyhow::Result<()> {
    let mut args: Args = structopt::StructOpt::from_args();

    let manifest_path = 'manifest_path: {
        // By default Cargo scans the current dir and all of its parents for Cargo.toml,
        // so we mirror its behavior here.
        let current_dir = std::env::current_dir()?;

        for dir_to_test in current_dir.ancestors() {
            let path_to_test = dir_to_test.join(Path::new("Ravedude.toml"));
            if path_to_test.exists() {
                break 'manifest_path Some(path_to_test);
            }
        }

        None
    };

    if manifest_path.is_some() {
        if let Some(board) = args.board.take() {
            if args.bin.is_none() {
                // The board arg is taken before the binary, so rearrange the args when Ravedude.toml exists
                args.bin = Some(std::path::PathBuf::from(board));
            } else {
                anyhow::bail!("can't pass board as command-line argument when Ravedude.toml is present; set `board = \"{}\"` under [general] in Ravedude.toml", board)
            }
        }
    } else if args.board.is_some() {
        warning!(
            "Passing the board as command-line argument is deprecated, use Ravedude.toml instead:\n\n# Ravedude.toml\n{}",
            toml::to_string(&config::RavedudeConfig::from_args(&args)?)?
        );
    }

    let mut ravedude_config = match manifest_path.as_deref() {
        Some(path) => board::get_board_from_manifest(path)?,
        None => board::get_board_from_name(args.board.as_deref().ok_or_else(||anyhow::anyhow!("no board given and couldn't find Ravedude.toml in project, either pass a board as an argument or make a Ravedude.toml."))?)?
    };

    if args.dump_config {
        println!("{}", toml::to_string(&ravedude_config)?);
        return Ok(());
    }

    avrdude::Avrdude::require_min_ver(MIN_VERSION_AVRDUDE)?;

    ravedude_config.general_options.apply_overrides(&mut args)?;

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

    if let Some(wait_time) = args.reset_delay {
        if wait_time > 0 {
            println!("Waiting {} ms before proceeding", wait_time);
            let wait_time = Duration::from_millis(wait_time);
            thread::sleep(wait_time);
        } else {
            println!("Assuming board has been reset");
        }
    } else {
        if let Some(config::ResetOptions {
            automatic: false,
            message,
        }) = board.reset.as_ref()
        {
            warning!("this board cannot reset itself.");
            if let Some(msg) = message.as_deref() {
                eprintln!();
                eprintln!("    {msg}");
            }
            eprintln!();
            eprint!("Once reset, press ENTER here: ");
            std::io::stdin().read_line(&mut String::new())?;
        }
    }

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

    if let Some(bin) = args.bin.as_ref() {
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

    if ravedude_config.general_options.open_console == Some(true) {
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
