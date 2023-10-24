use anyhow::Context as _;
use colored::Colorize as _;
use structopt::clap::AppSettings;

use std::time::Duration;
use std::thread;

mod avrdude;
mod board;
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
    /// After successfully flashing the program, open a serial console to see output sent by the
    /// board and possibly interact with it.
    #[structopt(short = "c", long = "open-console")]
    open_console: bool,

    /// Baudrate which should be used for the serial console.
    #[structopt(short = "b", long = "baudrate")]
    baudrate: Option<u32>,

    /// Overwrite which port to use.  By default ravedude will try to find a connected board by
    /// itself.
    #[structopt(short = "P", long = "port", parse(from_os_str), env = "RAVEDUDE_PORT")]
    port: Option<std::path::PathBuf>,

    /// This assumes the board is already resetting.
    /// Instead of giving the reset instructions and waiting for user confirmation, we wait the amount in milliseconds before proceeding.
    /// Set this value to 0 to skip the board reset question instantly.
    #[structopt(short = "d", long = "reset-delay")]
    reset_delay: Option<u64>,

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
    /// * promini-5v
    /// * trinket-pro
    /// * trinket
    /// * nano168
    /// * duemilanove
    #[structopt(name = "BOARD", verbatim_doc_comment)]
    board: String,

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
    let args: Args = structopt::StructOpt::from_args();
    avrdude::Avrdude::require_min_ver(MIN_VERSION_AVRDUDE)?;

    let board = board::get_board(&args.board).expect("board not found");

    task_message!("Board", "{}", board.display_name());

    if let Some(wait_time) = args.reset_delay{
        if wait_time > 0 {
            println!("Waiting {} ms before proceeding", wait_time);
            let wait_time = Duration::from_millis(wait_time);
            thread::sleep(wait_time);
        }else{
            println!("Assuming board has been reset");
        }
    }else{
        if let Some(msg) = board.needs_reset() {
            warning!("this board cannot reset itself.");
            eprintln!("");
            eprintln!("    {}", msg);
            eprintln!("");
            eprint!("Once reset, press ENTER here: ");
            std::io::stdin().read_line(&mut String::new())?;
        }
    }

    let port = match args.port {
        Some(port) => Ok(Some(port)),
        None => match board.guess_port() {
            Some(Ok(port)) => Ok(Some(port)),
            p @ Some(Err(_)) => p.transpose().context(
                "no matching serial port found, use -P or set RAVEDUDE_PORT in your environment",
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

        let mut avrdude = avrdude::Avrdude::run(&board.avrdude_options(), port.as_ref(), bin)?;
        avrdude.wait()?;

        task_message!("Programmed", "{}", bin.display());
    } else {
        task_message!(
            "",
            "{}",
            "(Skip flashing because no binary was given)".dimmed()
        );
    }

    if args.open_console {
        let baudrate = args
            .baudrate
            .context("-b/--baudrate is needed for the serial console")?;

        let port = port.context("console can only be opened for devices with USB-to-Serial")?;

        task_message!("Console", "{} at {} baud", port.display(), baudrate);
        task_message!(
            "",
            "{}",
            "CTRL+C to exit.".dimmed()
        );
        // Empty line for visual consistency
        eprintln!();
        console::open(&port, baudrate)?;
    } else if args.bin.is_none() && port.is_some() {
        warning!("you probably meant to add -c/--open-console?");
    }

    Ok(())
}
