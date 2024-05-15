#[doc(hidden)]
use std::{io::Read, path::PathBuf};

use clap::Parser;
use typst_syntax::parse;
use typstyle_core::{
    attr::calculate_attributes, strip_trailing_whitespace, PrettyPrinter, Typstyle,
};
use walkdir::{DirEntry, WalkDir};

use crate::cli::CliArguments;

mod cli;

fn get_input(input: Option<&PathBuf>) -> String {
    match input {
        Some(path) => std::fs::read_to_string(path).unwrap(),
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
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

fn main() {
    let args = CliArguments::parse();
    if let Some(command) = &args.command {
        match command {
            cli::Command::FormatAll { directory } => {
                let width = args.column;
                let directory = directory
                    .clone()
                    .unwrap_or_else(|| std::env::current_dir().unwrap());
                let walker = WalkDir::new(directory).into_iter();
                let mut format_count = 0;
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
                        std::fs::write(entry.path(), res).unwrap();
                        format_count += 1;
                    }
                }
                println!("Formatted {} files", format_count);
            }
        }
        return;
    }
    if args.input.is_empty() {
        format(None, &args);
    } else {
        for file in &args.input {
            format(Some(file), &args);
        }
    }
}

fn format(input: Option<&PathBuf>, args: &CliArguments) {
    let CliArguments {
        column: line_width,
        ast,
        pretty_doc,
        inplace,
        ..
    } = args;
    let content = get_input(input);
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
            std::fs::write(input, res).unwrap();
        } else {
            panic!("Cannot use inplace formatting with stdin");
        }
    } else {
        print!("{}", res);
    }
}
