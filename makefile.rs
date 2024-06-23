//! ```cargo
//! [dependencies]
//! owo-colors  = "*"
//! ```

use owo_colors::colors::*;
use owo_colors::OwoColorize;
use std::env;
use std::process::{Command, ExitCode};

#[derive(Debug)]
enum CompilationType {
    // Name, examples
    Board(&'static str, bool),
    // Name, spec, crate
    Mcu(&'static str, &'static str, &'static str),
}

fn main() -> ExitCode {
    // Matrix elements
    let matrix = vec![
        CompilationType::Board("arduino-uno", true),
        CompilationType::Board("arduino-diecimila", true),
        CompilationType::Board("arduino-leonardo", true),
        CompilationType::Board("arduino-mega2560", true),
        CompilationType::Board("arduino-mega1280", true),
        CompilationType::Board("sparkfun-promicro", true),
        CompilationType::Board("sparkfun-promini-3v3", true),
        CompilationType::Board("sparkfun-promini-5v", true),
        CompilationType::Board("trinket-pro", true),
        CompilationType::Board("trinket", true),
        CompilationType::Board("arduino-nano", true),
        CompilationType::Board("nano168", true),
        CompilationType::Board("atmega2560", true),
        // MCU Time
        CompilationType::Mcu("atmega1280", "atmega1280", "atmega-hal"),
        CompilationType::Mcu("atmega32a", "atmega32a", "atmega-hal"),
        CompilationType::Mcu("atmega128a", "atmega128a", "atmega-hal"),
        CompilationType::Mcu("atmega328pb", "atmega328p", "atmega-hal"),
        CompilationType::Mcu("atmega48p", "atmega48p", "atmega-hal"),
        CompilationType::Mcu("atmega1284p", "atmega1284p", "atmega-hal"),
        CompilationType::Mcu("atmega8", "atmega8", "atmega-hal"),
        CompilationType::Mcu("attiny85", "attiny85", "attiny-hal"),
        CompilationType::Mcu("attiny88", "attiny88", "attiny-hal"),
        CompilationType::Mcu("attiny167", "attiny167", "attiny-hal"),
        CompilationType::Mcu("attiny2313", "attiny2313", "attiny-hal"),
    ];

    for comp in matrix {
        let mut directory = String::new();
        let mut args = Vec::new();
        let spec_str;
        match comp {
            CompilationType::Board(name, examples) => {
                if examples {
                    directory = format!("examples/{}", name);
                    args = vec!["build", "--bins"];
                } else {
                    directory = "arduino-hal/".to_string();
                    args = vec!["build", "--features", name]
                }
            }
            CompilationType::Mcu(name, spec, run_crate) => {
                directory = format!("mcu/{}", run_crate);
                spec_str = format!("../../avr-specs/avr-{}.json", spec);
                args = vec![
                    "build",
                    "--features",
                    name,
                    "-Z",
                    "build-std=core",
                    "--target",
                    &spec_str,
                ]
            }
        }

        println!("=== {} ===", "Building".magenta().bold());
        println!("{} {:?}", "Type:".yellow(), comp);
        println!("{} {}", "Directory:".yellow(), directory);
        println!("{} {:?}", "Args:".yellow(), args);

        // Run cargo with the args, and directory attached
        let mut child = Command::new("cargo")
            .args(args)
            .current_dir(directory)
            .spawn()
            .expect("Something fucked up.");
        let res = child.wait().unwrap();

        println!("Exited with code: {}", res.bold());
        if !res.success() {
            return ExitCode::from(res.code().unwrap() as u8);
        }
        println!("--- {} ---", "BUILT".green().bold());
    }

    println!(
        "{} All tests succesfully passed!",
        "Success!".green().bold()
    );

    ExitCode::SUCCESS
}
