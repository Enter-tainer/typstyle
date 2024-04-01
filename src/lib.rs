pub mod attr;
pub mod pretty;
pub mod util;

pub use attr::calculate_attributes;
pub use pretty::PrettyPrinter;

pub fn pretty_print(content: &str, width: usize) -> String {
    let root = typst_syntax::parse(content);
    if root.erroneous() {
        return content.to_string();
    }
    let attr_map = calculate_attributes(root.clone());
    let printer = PrettyPrinter::new(attr_map);
    let markup = root.cast().unwrap();
    let doc = printer.convert_markup(markup);
    strip_trailing_whitespace(&doc.pretty(width).to_string())
}

/// Strip trailing whitespace in each line of the input string.
fn strip_trailing_whitespace(s: &str) -> String {
    let has_trailing_newline = s.ends_with('\n');
    let res = s
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    if has_trailing_newline {
        res + "\n"
    } else {
        res
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub fn pretty_print_wasm(content: &str, width: usize) -> String {
    pretty_print(content, width)
}
