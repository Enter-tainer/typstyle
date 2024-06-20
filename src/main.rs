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
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {:#?}", path)),
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

fn main() -> CliResults {
    let args = CliArguments::parse();
    if let Err(e) = execute(args) {
        eprintln!("{e}");
        return CliResults::Bad;
    }

    CliResults::Good
}

fn execute(args: CliArguments) -> Result<()> {
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
                        let res = Typstyle::new_with_content(content, width).pretty_print();

                        // `FormatAll` must be done in place without failing in the middle
                        match std::fs::write(entry.path(), res).with_context(|| {
                            format!(
                                "failed to overwrite {path}",
                                path = entry.path().display().to_string()
                            )
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
        }

        return Ok(());
    }

    if args.input.is_empty() {
        format(None, &args)?;
    } else {
        // In case of multiple files, process them in order without failing
        let mut error_count = 0;
        for file in &args.input {
            format(Some(file), &args).unwrap_or_else(|e| {
                eprintln!("{e}");
                error_count += 1;
            });
        }

        if error_count > 0 {
            bail!("failed to format {error_count} files");
        }
    }

    Ok(())
}

fn format(input: Option<&PathBuf>, args: &CliArguments) -> Result<()> {
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
        content
    } else {
        strip_trailing_whitespace(&doc.pretty(*line_width).to_string())
    };
    if *inplace {
        if let Some(input) = input {
            std::fs::write(input, res).with_context(|| {
                format!(
                    "failed to write to the file {file}",
                    file = input.display().to_string()
                )
            })?;
        } else {
            // This branch should never be reached
            unreachable!("cannot perform in-place formatting without at least one file being presented");
        }
    } else {
        print!("{}", res);
    }

    Ok(())
}
