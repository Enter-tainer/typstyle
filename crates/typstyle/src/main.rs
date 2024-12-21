mod cli;
mod fmt;
mod logging;

use std::process::{ExitCode, Termination};

use anyhow::Result;
use clap::Parser;
use fmt::{format_all, format_many, format_one, FormatStatus};
use log::error;

use crate::cli::CliArguments;

enum CliResults {
    Good,
    Bad,
}

impl Termination for CliResults {
    fn report(self) -> ExitCode {
        match self {
            CliResults::Good => ExitCode::SUCCESS,
            _ => ExitCode::FAILURE,
        }
    }
}

fn main() -> CliResults {
    logging::init();

    let args = CliArguments::parse();
    args.validate_input();

    // Should put the formatter into check mode
    let check = args.check;
    match execute(args) {
        Ok(FormatStatus::Changed) if check => CliResults::Bad,
        Ok(_) => CliResults::Good,
        Err(e) => {
            error!("{e}");
            CliResults::Bad
        }
    }
}

fn execute(args: CliArguments) -> Result<FormatStatus> {
    if let Some(command) = &args.command {
        match command {
            cli::Command::FormatAll { directory } => {
                return format_all(directory, &args);
            }
            #[cfg(feature = "completion")]
            cli::Command::Completions { shell } => {
                use clap::CommandFactory;

                clap_complete::generate(
                    *shell,
                    &mut cli::CliArguments::command(),
                    "typstyle",
                    &mut std::io::stdout(),
                );
            }
        }
        return Ok(FormatStatus::Unchanged);
    }

    if args.input.is_empty() {
        format_one(None, &args)
    } else {
        format_many(&args.input, &args)
    }
}
