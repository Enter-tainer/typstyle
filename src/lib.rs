pub mod pretty;
pub mod prop;
pub mod util;

pub use pretty::PrettyPrinter;
pub use prop::get_no_format_nodes;

pub fn pretty_print(content: &str, width: usize) -> String {
    let root = typst_syntax::parse(content);
    if root.erroneous() {
        return content.to_string();
    }
    let disabled_nodes = get_no_format_nodes(root.clone());
    let printer = PrettyPrinter::new(disabled_nodes);
    let markup = root.cast().unwrap();
    let doc = printer.convert_markup(markup);
    doc.pretty(width).to_string()
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn pretty_print_wasm(content: &str, width: usize) -> String {
    pretty_print(content, width)
}
