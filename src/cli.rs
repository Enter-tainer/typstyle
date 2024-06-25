use std::{
    path::PathBuf,
    process::{ExitCode, Termination},
};

use clap::{Parser, Subcommand};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Parser)]
#[clap(name = "typstyle", author, version, about, long_version(LONG_VERSION.as_str()))]
pub struct CliArguments {
    /// Path to the input files, if not provided, read from stdin. If multiple files are provided, they will be processed in order
    pub input: Vec<PathBuf>,
    /// The column width of the output
    #[clap(short, long, default_value = "80")]
    pub column: usize,
    /// Print the AST of the input file
    #[clap(short, long, default_value = "false")]
    pub ast: bool,
    /// Print the pretty document
    #[clap(short, long, default_value = "false")]
    pub pretty_doc: bool,
    /// Format the file in place
    #[clap(short, long, default_value = "false")]
    pub inplace: bool,
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Format all files in-place in the given directory
    FormatAll {
        /// The directory to format. If not provided, the current directory is used
        directory: Option<PathBuf>,
    },
}

pub enum CliResults {
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

static NONE: &str = "None";
static LONG_VERSION: Lazy<String> = Lazy::new(|| {
    format!(
        "
Version:             {}
Build Timestamp:     {}
Build Git Describe:  {}
Commit SHA:          {}
Commit Date:         {}
Commit Branch:       {}
Cargo Target Triple: {}
",
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_BUILD_TIMESTAMP"),
        option_env!("VERGEN_GIT_DESCRIBE").unwrap_or(NONE),
        option_env!("VERGEN_GIT_SHA").unwrap_or(NONE),
        option_env!("VERGEN_GIT_COMMIT_TIMESTAMP").unwrap_or(NONE),
        option_env!("VERGEN_GIT_BRANCH").unwrap_or(NONE),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
    )
});
