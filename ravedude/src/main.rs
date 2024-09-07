use anyhow::{bail, Context as _};
use colored::Colorize as _;
use structopt::clap::AppSettings;

use std::str::FromStr;
use std::thread;
use std::time::Duration;

mod avrdude;
mod board;
mod console;
mod ui;

/// This represents the minimum (Major, Minor) version raverdude requires avrdude to meet.
const MIN_VERSION_AVRDUDE: (u8, u8) = (6, 3);

#[derive(Debug)]
enum OutputMode {
    Ascii,
    Hex,
    Dec,
    Bin,
}

impl FromStr for OutputMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascii" => Ok(Self::Ascii),
            "hex" => Ok(Self::Hex),
            "dec" => Ok(Self::Dec),
            "bin" => Ok(Self::Bin),
            _ => Err(anyhow::anyhow!("unknown output mode")),
        }
    }
}

// this could have fewer nested if’s, but this produces better error messages
fn parse_newline_on(s: &str) -> Result<char, anyhow::Error> {
    if let Ok(c) = s.parse::<char>() {
        Ok(c)

    // if it starts with 0x then parse the hex byte
    } else if &s[0..2] == "0x" {
        if s.len() == 4 {
            if let Ok(n) = u8::from_str_radix(&s[2..4], 16) {
                Ok(n as char)
            } else {
                bail!("invalid hex byte")
            }
        } else {
            bail!("hex byte must have 2 characters")
        }
    // if it starts with 0b then parse the binary byte
    } else if &s[0..2] == "0b" {
        if s.len() == 10 {
            if let Ok(n) = u8::from_str_radix(&s[2..10], 2) {
                Ok(n as char)
            } else {
                bail!("invalid binary byte")
            }
        } else {
            bail!("binary byte must have 8 characters")
        }
    } else {
        bail!("must be a single character or a byte in hex or binary notation")
    }
}

#[test]
fn test_parse_newline_on() {
    assert_eq!(parse_newline_on("a").unwrap(), 'a');
    assert_eq!(parse_newline_on("\n").unwrap(), '\n');
    assert_eq!(parse_newline_on("0x41").unwrap(), 'A');
    assert_eq!(parse_newline_on("0b01000001").unwrap(), 'A');
    assert!(parse_newline_on("not a char").is_err());
    assert!(parse_newline_on("0x").is_err());
    assert!(parse_newline_on("0xzz").is_err());
    assert!(parse_newline_on("0b").is_err());
    assert!(parse_newline_on("0b0a0a0a0a").is_err());
}

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

    /// Print the avrdude command that is executed for flashing the binary.
    #[structopt(long = "debug-avrdude")]
    debug_avrdude: bool,

    /// Output mode.
    /// Can be ascii, hex, dec or bin
    #[structopt(short = "o")]
    output_mode: Option<OutputMode>,

    /// Print a newline after this byte
    /// not used with output_mode ascii
    /// hex (0x) and bin (0b) notations are supported.
    /// matching chars/bytes are NOT removed
    /// to add newlines after \n (in non-ascii mode), use \n, 0x0a or 0b00001010
    #[structopt(long = "newline-on", parse(try_from_str = parse_newline_on), verbatim_doc_comment)]
    newline_on: Option<char>,

    /// Print a newline after n bytes
    /// not used with output_mode ascii
    /// defaults to 16 for hex and dec and 8 for bin
    /// if dividable by 4, bytes will be grouped to 4
    #[structopt(long = "newline-after", verbatim_doc_comment)]
    newline_after: Option<u8>,

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

    if args.newline_on.is_some() && args.newline_after.is_some() {
        bail!("newline_on and newline_after cannot be used at the same time");
    }

    task_message!("Board", "{}", board.display_name());

    if let Some(wait_time) = args.reset_delay {
        if wait_time > 0 {
            println!("Waiting {} ms before proceeding", wait_time);
            let wait_time = Duration::from_millis(wait_time);
            thread::sleep(wait_time);
        } else {
            println!("Assuming board has been reset");
        }
    } else {
        if let Some(msg) = board.needs_reset() {
            warning!("this board cannot reset itself.");
            eprintln!("");
            eprintln!("    {}", msg);
            eprintln!("");
            eprint!("Once reset, press ENTER here: ");
            std::io::stdin().read_line(&mut String::new())?;
        }
    }

    let port = match args.port.clone() {
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

        let mut avrdude = avrdude::Avrdude::run(
            &board.avrdude_options(),
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

    if args.open_console {
        console::open(args)?;
    } else if args.bin.is_none() && port.is_some() {
        warning!("you probably meant to add -c/--open-console?");
    }

    Ok(())
}
