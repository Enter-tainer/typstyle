use std::{path::PathBuf, sync::LazyLock};

use clap::{Parser, Subcommand};

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
    /// Run in 'check' mode. Exits with 0 if input is formatted correctly. Exits with 1 if formatting is required.
    #[clap(long, default_value = "false")]
    pub check: bool,
}

#[derive(Debug, Subcommand, Clone)]
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
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },
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
