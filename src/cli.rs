use std::path::PathBuf;

use clap::Parser;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Parser)]
#[clap(name = "typstyle", author, version, about, long_version(LONG_VERSION.as_str()))]
pub struct CliArguments {
    /// Path to the input file, if not provided, read from stdin
    pub input: Option<PathBuf>,
    /// The width of the output
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
        env!("VERGEN_GIT_DESCRIBE"),
        option_env!("VERGEN_GIT_SHA").unwrap_or(NONE),
        option_env!("VERGEN_GIT_COMMIT_TIMESTAMP").unwrap_or(NONE),
        option_env!("VERGEN_GIT_BRANCH").unwrap_or(NONE),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
    )
});
