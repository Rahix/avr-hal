/// Emit a message in the cargo style with a green "verb" up front and some text afterwards.
#[macro_export]
macro_rules! task_message {
    ($verb:expr, $($fmt:tt)+) => {
        eprint!("{:>12} ", colored::Colorize::bold(colored::Colorize::green($verb)));
        eprintln!($($fmt)+);
    };
}

#[macro_export]
macro_rules! warning {
    ($($fmt:tt)+) => {
        eprint!("{}", colored::Colorize::bold(colored::Colorize::yellow("Warning")));
        eprint!("{}", colored::Colorize::bold(": "));
        eprintln!("{}", colored::Colorize::bold(&*format!($($fmt)+)));
    };
}

pub fn print_error(e: anyhow::Error) {
    use colored::Colorize as _;

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
}
