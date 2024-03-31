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
    doc.pretty(width).to_string()
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub fn pretty_print_wasm(content: &str, width: usize) -> String {
    pretty_print(content, width)
}
