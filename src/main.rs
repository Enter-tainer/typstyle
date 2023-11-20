use clap::Parser;
use typst_geshihua::PrettyPrinter;
use typst_syntax::parse;

use crate::cli::CliArguments;

mod cli;

fn main() {
    let CliArguments { input } = CliArguments::parse();
    let content = std::fs::read_to_string(input).unwrap();
    let root = parse(&content);
    println!("{:#?}", root);
    let markup = root.cast().unwrap();
    let printer = PrettyPrinter::default();
    let doc = printer.convert_markup(markup);
    print!("{:#?}", doc);
    let res = doc.pretty(80).to_string();
    print!("{}", res);
}
