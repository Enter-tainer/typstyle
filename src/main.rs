#[doc(hidden)]
use std::{io::Read, path::PathBuf};

use clap::Parser;
use typst_syntax::parse;
use typstyle_core::{attr::calculate_attributes, PrettyPrinter};

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

fn main() {
    let args = CliArguments::parse();
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
        doc.pretty(*line_width).to_string()
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
