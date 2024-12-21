use std::{
    io::Read,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{bail, Context, Result};
use log::{error, info, warn};
use typst_syntax::Source;
use walkdir::{DirEntry, WalkDir};

use typstyle_core::{
    attr::AttrStore, strip_trailing_whitespace, PrettyPrinter, PrinterConfig, Typstyle,
};

use crate::cli::CliArguments;

pub enum FormatStatus {
    /// The content was changed (and written back to the file if needed).
    Changed,
    /// The content was already well-formatted and unchanged, or erroneous.
    Unchanged,
}

impl std::ops::BitOrAssign for FormatStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        match (&self, rhs) {
            (Self::Unchanged, FormatStatus::Unchanged) => *self = Self::Unchanged,
            _ => *self = Self::Changed,
        }
    }
}

/// Formats all `.typ` files in the specified directory, or the current directory if none is given.
///
/// This function recursively searches the provided directory for `.typ` files, formats them, and
/// overwrites them with the formatted content if needed.
///
/// # Parameters
/// - `directory`: An optional path to the directory containing `.typ` files. If `None`, the
///   current working directory is used.
/// - `args`: CLI arguments.
///
/// # Returns
/// Returns `Ok(FormatStatus)` indicating whether any file was modified.
pub fn format_all(directory: &Option<PathBuf>, args: &CliArguments) -> Result<FormatStatus> {
    let mut status = FormatStatus::Unchanged;

    let directory = directory
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    #[derive(Default)]
    struct Summary {
        format_count: usize,
        unchanged_count: usize,
        error_count: usize,
    }
    let mut summary = Summary::default();

    let start_time = Instant::now();

    // Walk through all the files in the directory
    let entries = WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !is_hidden(e));
    for entry in entries {
        let Ok(entry) = entry else { continue };
        if !(entry.file_type().is_file() && entry.path().extension() == Some("typ".as_ref())) {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let cfg = PrinterConfig::new_with_width(args.style.column);
        let Ok(res) = Typstyle::new_with_content(content.clone(), cfg).pretty_print() else {
            warn!("Failed to format: {}", entry.path().display());
            continue;
        };

        // Check if the content is already well-formatted (unchanged)
        if res == content {
            summary.unchanged_count += 1;
            continue;
        }
        status = FormatStatus::Changed;

        if args.check {
            info!("Would reformat: {}", entry.path().display());
            summary.format_count += 1
        } else {
            // Attempt to overwrite the file with the formatted content
            match write_back(entry.path(), &res) {
                Ok(_) => summary.format_count += 1,
                Err(e) => {
                    error!("{e}");
                    summary.error_count += 1;
                }
            }
        }
    }
    let duration = start_time.elapsed();

    if args.check {
        info!(
            "{} files would be reformatted ({} already formatted), checked in {:?}",
            summary.format_count, summary.unchanged_count, duration
        );
    } else {
        info!(
            "Successfully formatted {} files ({} unchanged) in {:?}",
            summary.format_count, summary.unchanged_count, duration
        );
    }
    if summary.error_count > 0 {
        // Syntax errors are not counted here.
        bail!(
            "failed to format {} files due to IO error",
            summary.error_count
        );
    }

    Ok(status)
}

/// Formats multiple `.typ` files passed as a list of paths.
///
/// This function processes each file individually, and if any errors occur, they are handled without stopping
/// the entire operation
///
/// # Parameters
/// - `input`: A list of paths to `.typ` files to be formatted.
/// - `args`: CLI arguments.
///
/// # Returns
/// Returns `Ok(FormatStatus)` indicating whether any file was modified.
pub fn format_many(input: &[PathBuf], args: &CliArguments) -> Result<FormatStatus> {
    // In case of multiple files, process them in order without failing
    let mut status = FormatStatus::Unchanged;
    let mut error_count = 0;
    // Format the files one by one
    for file in input {
        status |= format_one(Some(file), args).unwrap_or_else(|e| {
            error!("{e}");
            error_count += 1;
            FormatStatus::Unchanged
        });
    }

    if error_count > 0 {
        bail!("failed to format {error_count} files due to IO error");
    }
    Ok(status)
}

/// Formats a single `.typ` file or input from stdin.
///
/// This function formats the file provided as an argument, or reads from stdin if no file is given.
/// If in-place formatting is requested, it overwrites the file with the formatted content.
///
/// # Parameters
/// - `input`: An optional path to a `.typ` file to be formatted. If `None`, input is read from stdin.
/// - `args`: CLI arguments.
///
/// # Returns
/// Returns `Ok(FormatStatus)` indicating whether the file was modified or remained unchanged.
pub fn format_one(input: Option<&PathBuf>, args: &CliArguments) -> Result<FormatStatus> {
    let content = get_input(input)?;
    let res = format_debug(content, args);
    let status = match &res {
        FormatResult::Changed(_) => FormatStatus::Changed,
        _ => FormatStatus::Unchanged,
    };
    match res {
        FormatResult::Changed(res) if args.inplace => {
            // We have already validated that the input is Some.
            write_back(input.unwrap(), &res)?;
        }
        FormatResult::Changed(_) if args.check => {
            if let Some(path) = input {
                info!("Would reformat: {}", path.display());
            }
        }
        FormatResult::Unchanged(_) if args.check => {}
        FormatResult::Changed(res) | FormatResult::Unchanged(res) => {
            info!("{}", res);
        }
        FormatResult::Erroneous => {
            if let Some(path) = input {
                warn!("Failed to parse {}", path.display());
            } else {
                warn!("Failed to parse stdin");
            }
        }
    }
    Ok(status)
}

enum FormatResult {
    Changed(String),
    Unchanged(String),
    Erroneous,
}

fn format_debug(content: String, args: &CliArguments) -> FormatResult {
    let source = Source::detached(&content);
    let root = source.root();
    let attr_store = AttrStore::new(root);
    if args.debug.ast {
        println!("{:#?}", root);
    }

    // Error formatting document.
    if root.erroneous() {
        return FormatResult::Erroneous;
    }

    let config = PrinterConfig {
        max_width: args.style.column,
        ..Default::default()
    };
    let printer = PrettyPrinter::new(config, attr_store);
    let doc = printer.convert_markup(root.cast().unwrap());
    if args.debug.pretty_doc {
        println!("{:#?}", doc);
    }
    let res = strip_trailing_whitespace(&doc.pretty(args.style.column).to_string());

    // Compare `res` with `content` to perform CI checks
    if res != content {
        FormatResult::Changed(res)
    } else {
        FormatResult::Unchanged(res)
    }
}

fn get_input(input: Option<&PathBuf>) -> Result<String> {
    match input {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display())),
        None => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .with_context(|| "failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn write_back(path: &Path, content: &str) -> Result<()> {
    std::fs::write(path, content)
        .with_context(|| format!("failed to write to the file {}", path.display()))
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.'))
}
