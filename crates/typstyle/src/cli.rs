use std::{path::PathBuf, sync::LazyLock};

use clap::{error::ErrorKind, Args, CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "typstyle", author, version, about, long_version(LONG_VERSION.as_str()))]
pub struct CliArguments {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to the input files, if not provided, read from stdin. If multiple files are provided, they will be processed in order
    pub input: Vec<PathBuf>,

    /// Format the file in place
    #[arg(short, long, default_value_t = false, conflicts_with = "check")]
    pub inplace: bool,

    /// Run in 'check' mode. Exits with 0 if input is formatted correctly. Exits with 1 if formatting is required.
    #[arg(long, default_value_t = false, global = true)]
    pub check: bool,

    #[command(flatten)]
    pub style: StyleArgs,

    #[command(flatten)]
    pub debug: DebugArgs,
}

impl CliArguments {
    pub fn validate_input(&self) {
        if self.command.is_none() && self.inplace && self.input.is_empty() {
            let mut cmd = Self::command();
            cmd.error(
                ErrorKind::ValueValidation,
                "cannot perform in-place formatting without at least one file being presented",
            )
            .exit();
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    /// Format all files in-place in the given directory
    FormatAll {
        /// The directory to format. If not provided, the current directory is used
        directory: Option<PathBuf>,
    },
    #[cfg(feature = "completion")]
    /// Generate shell completions for the given shell to stdout
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Args)]
pub struct StyleArgs {
    /// The column width of the output
    #[arg(short, long, default_value_t = 80, global = true)]
    pub column: usize,
}

#[derive(Args)]
pub struct DebugArgs {
    /// Print the AST of the input file
    #[arg(short, long, default_value_t = false)]
    pub ast: bool,

    /// Print the pretty document
    #[arg(short, long, default_value_t = false)]
    pub pretty_doc: bool,
}

static NONE: &str = "None";
static LONG_VERSION: LazyLock<String> = LazyLock::new(|| {
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
