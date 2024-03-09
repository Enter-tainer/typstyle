use clap::Parser;
use typst_geshihua::PrettyPrinter;
use typst_syntax::parse;

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
    if ast {
        println!("{:#?}", root);
    }
    let markup = root.cast().unwrap();
    let printer = PrettyPrinter::default();
    let doc = printer.convert_markup(markup);
    if pretty_doc {
        println!("{:#?}", doc);
    }
    let res = doc.pretty(line_width).to_string();
    if inplace {
        std::fs::write(&input, res).unwrap();
    } else {
        print!("{}", res);
    }
}
