use std::{io::Read, path::PathBuf};

use clap::Parser;
use typst_syntax::parse;
use typstyle_lib::{prop::get_no_format_nodes, PrettyPrinter};

use crate::cli::CliArguments;

mod cli;

fn get_input(input: &Option<PathBuf>) -> String {
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
    let CliArguments {
        input,
        column: line_width,
        ast,
        pretty_doc,
        inplace,
    } = CliArguments::parse();
    let content = get_input(&input);
    let root = parse(&content);
    let disabled_nodes = get_no_format_nodes(root.clone());
    if ast {
        println!("{:#?}", root);
    }
    let markup = root.cast().unwrap();
    let printer = PrettyPrinter::new(disabled_nodes);
    let doc = printer.convert_markup(markup);
    if pretty_doc {
        println!("{:#?}", doc);
    }
    let res = if root.erroneous() {
        content
    } else {
        doc.pretty(line_width).to_string()
    };
    if inplace {
        if let Some(input) = input {
            std::fs::write(input, res).unwrap();
        } else {
            panic!("Cannot use inplace formatting with stdin");
        }
    } else {
        print!("{}", res);
    }
}
