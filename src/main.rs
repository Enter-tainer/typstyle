use clap::Parser;
use typst_syntax::parse;
use typstyle::{prop::get_no_format_nodes, PrettyPrinter};

use crate::cli::CliArguments;

mod cli;

fn main() {
    let CliArguments {
        input,
        column: line_width,
        ast,
        pretty_doc,
        inplace,
    } = CliArguments::parse();
    let content = std::fs::read_to_string(&input).unwrap();
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
        std::fs::write(&input, res).unwrap();
    } else {
        print!("{}", res);
    }
}
