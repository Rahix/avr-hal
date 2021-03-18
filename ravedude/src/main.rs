use anyhow::Context as _;
use colored::Colorize as _;
use structopt::clap::AppSettings;

mod avrdude;
mod board;
mod console;
mod ui;

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
    /// After sucessfully flashing the program, open a serial console to see output sent by the
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

    /// Which board to interact with.
    ///
    /// Must be one of the known board identifiers:
    ///
    /// * uno
    /// * nano
    /// * leonardo
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

    let board = board::get_board(&args.board).expect("board not found");

    task_message!("Board", "{}", board.display_name());

    if board.needs_reset() {
        warning!("this board cannot reset itself.");
        eprint!("Press the reset-button and then ENTER here: ");
        std::io::stdin().read_line(&mut String::new())?;
    }

    let port = args.port.map_or_else(
        || {
            board.guess_port().context(
                "no matching serial port found, use -P or set RAVEDUDE_PORT in your environment",
            )
        },
        Ok,
    )?;

    if let Some(bin) = args.bin.as_ref() {
        task_message!(
            "Programming",
            "{} {} {}",
            bin.display(),
            "=>".blue().bold(),
            port.display()
        );

        let mut avrdude = avrdude::Avrdude::run(&board.avrdude_options(), &port, bin)?;
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

        task_message!("Console", "{} at {} baud", port.display(), baudrate);
        console::open(&port, baudrate)?;
    } else if args.bin.is_none() {
        warning!("you probably meant to add -c/--open-console?");
    }

    Ok(())
}
