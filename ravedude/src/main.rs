use anyhow::Context as _;
use colored::Colorize as _;
use structopt::clap::AppSettings;

mod avrdude;
mod board;
mod console;

/// ravedude is a rust wrapper around avrdude for providing the smoothest possible development
/// experience with rust on AVR microcontrollers.
///
/// ravedude is primarily intended to be used as a "runner" in the cargo configuration.
#[derive(structopt::StructOpt, Debug)]
#[structopt(name = "ravedude",
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::DeriveDisplayOrder)]
struct Args {
    /// After sucessfully flashing the program, open a serial console to see output sent by the
    /// board and possibly interact with it.
    #[structopt(short = "c", long = "open-console")]
    open_console: bool,

    /// Do not actually flash the program, just pretend to do it.
    #[structopt(short = "n", long = "no-program")]
    no_program: bool,

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
    #[structopt(name = "BINARY", parse(from_os_str))]
    bin: std::path::PathBuf,
}

fn main() {
    match ravedude() {
        Ok(()) => (),
        Err(e) => {
            eprintln!(
                "{}{}{}",
                "Error".red().bold(),
                ": ".bold(),
                e.to_string().bold()
            );
            eprintln!("");
            for cause in e.chain().skip(1) {
                eprintln!(
                    "{}{}{}",
                    "Caused by".yellow().bold(),
                    ": ".bold(),
                    cause.to_string().bold()
                );
            }
            std::process::exit(1);
        }
    }
}

fn ravedude() -> anyhow::Result<()> {
    let args: Args = structopt::StructOpt::from_args();

    let board = board::get_board(&args.board).expect("board not found");

    eprintln!("{:>12} {}", "Board".green().bold(), board.display_name());

    if board.needs_reset() {
        eprintln!(
            "{}{}",
            "Warning".yellow().bold(),
            ": this board cannot reset itself.".bold()
        );
        eprint!("Press the reset-button and then ENTER here: ");
        std::io::stdin().read_line(&mut String::new())?;
    }

    let port = args.port.map_or_else(
        || board.guess_port().context("no matching serial port found"),
        Ok,
    )?;

    eprintln!(
        "{:>12} {} {} {}",
        "Programming".green().bold(),
        args.bin.display(),
        "=>".blue().bold(),
        port.display()
    );

    if args.no_program {
        eprintln!(
            "{}{}",
            "Warning".yellow().bold(),
            ": skipped due to --no-progam".bold()
        );
    } else {
        let mut avrdude = avrdude::Avrdude::run(&board.avrdude_options(), &port, &args.bin)?;
        avrdude.wait()?;
    }

    eprintln!("{:>12} {}", "Programmed".green().bold(), args.bin.display());

    if args.open_console {
        let baudrate = args
            .baudrate
            .context("-b/--baudrate is needed for the serial console")?;

        eprintln!(
            "{:>12} {} at {} baud",
            "Console".green().bold(),
            port.display(),
            baudrate
        );
        console::open(&port, baudrate)?;
    }

    Ok(())
}
