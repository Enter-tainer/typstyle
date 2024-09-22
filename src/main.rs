#[doc(hidden)]
use std::{io::Read, path::PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use typst_syntax::parse;
use typstyle_core::{
    attr::calculate_attributes, strip_trailing_whitespace, PrettyPrinter, Typstyle,
};
use walkdir::{DirEntry, WalkDir};

use crate::cli::{CliArguments, CliResults};

mod cli;

fn get_input(input: Option<&PathBuf>) -> Result<String> {
    match input {
        Some(path) => {
            std::fs::read_to_string(path).with_context(|| format!("failed to read {:#?}", path))
        }
        None => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .with_context(|| "failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

enum FormatStatus {
    Changed,
    Unchanged,
}

impl From<bool> for FormatStatus {
    fn from(value: bool) -> Self {
        if value {
            FormatStatus::Changed
        } else {
            FormatStatus::Unchanged
        }
    }
}

fn main() -> CliResults {
    let args = CliArguments::parse();

    // Should put the formatter into check mode
    let check = args.check;
    match execute(args) {
        Ok(FormatStatus::Changed) if check => CliResults::Bad,
        Ok(_) => CliResults::Good,
        Err(e) => {
            eprintln!("{e}");
            CliResults::Bad
        }
    }
}

fn execute(args: CliArguments) -> Result<FormatStatus> {
    let mut changed = false;
    if let Some(command) = &args.command {
        match command {
            cli::Command::FormatAll { directory } => {
                let width = args.column;
                let directory = directory
                    .clone()
                    .unwrap_or_else(|| std::env::current_dir().unwrap());
                let walker = WalkDir::new(directory).into_iter();
                let mut format_count = 0;
                let mut error_count = 0;
                for entry in walker.filter_entry(|e| !is_hidden(e)) {
                    let Ok(entry) = entry else {
                        continue;
                    };
                    if entry.file_type().is_file()
                        && entry.path().extension() == Some("typ".as_ref())
                    {
                        let Ok(content) = std::fs::read_to_string(entry.path()) else {
                            continue;
                        };
                        let res = Typstyle::new_with_content(content.clone(), width).pretty_print();

                        // Check if the file is changed.
                        changed |= res != content;

                        // `FormatAll` must be done in place without failing in the middle
                        match std::fs::write(entry.path(), res).with_context(|| {
                            format!("failed to overwrite {path}", path = entry.path().display())
                        }) {
                            Ok(_) => format_count += 1,
                            Err(e) => {
                                eprintln!("{e}");
                                error_count += 1;
                            }
                        }
                    }
                }

                eprintln!("Successfully formatted {format_count} files");
                if error_count > 0 {
                    bail!("failed to format {error_count} files");
                }
            }
            #[cfg(feature = "completion")]
            cli::Command::Completions { shell } => {
                use clap::CommandFactory;
                use std::env::{args_os, current_exe};

                let bin_path = args_os()
                    .next()
                    .map(PathBuf::from)
                    .or_else(|| current_exe().ok());

                let bin_name = bin_path
                    .as_ref()
                    .and_then(|p| p.file_stem().and_then(|p| p.to_str()))
                    .unwrap_or("typstyle");

                clap_complete::generate(
                    *shell,
                    &mut cli::CliArguments::command(),
                    bin_name,
                    &mut std::io::stdout(),
                );
            }
        }

        return Ok(changed.into());
    }

    if args.input.is_empty() {
        changed = format(None, &args)?;
    } else {
        // In case of multiple files, process them in order without failing
        let mut error_count = 0;
        for file in &args.input {
            changed |= format(Some(file), &args).unwrap_or_else(|e| {
                eprintln!("{e}");
                error_count += 1;
                false
            });
        }

        if error_count > 0 {
            bail!("failed to format {error_count} files");
        }
    }

    Ok(changed.into())
}

fn format(input: Option<&PathBuf>, args: &CliArguments) -> Result<bool> {
    let CliArguments {
        column: line_width,
        ast,
        pretty_doc,
        inplace,
        ..
    } = args;
    if *inplace && input.is_none() {
        bail!("cannot perform in-place formatting without at least one file being presented");
    }
    let content = get_input(input)?;
    let root = parse(&content);
    let attr_map = calculate_attributes(root.clone());
    if *ast {
        println!("{:#?}", root);
    }
    let markup = root.cast().unwrap();
    let printer = PrettyPrinter::new(attr_map);
    let doc = printer.convert_markup(markup);
    if *pretty_doc {
        println!("{:#?}", doc);
    }

    let res = if root.erroneous() {
        content.clone()
    } else {
        strip_trailing_whitespace(&doc.pretty(*line_width).to_string())
    };

    // Compare `res` with `content` to perform CI checks
    let changed = res != content;
    if *inplace {
        if let Some(input) = input {
            std::fs::write(input, res).with_context(|| {
                format!("failed to write to the file {file}", file = input.display())
            })?;
        } else {
            // This branch should never be reached
            unreachable!(
                "cannot perform in-place formatting without at least one file being presented"
            );
        }
    } else {
        print!("{}", res);
    }

    Ok(changed)
}
