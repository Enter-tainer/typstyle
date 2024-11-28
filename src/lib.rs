#![doc = include_str!("../README.md")]
#[doc(hidden)]
pub mod attr;
#[doc(hidden)]
pub mod ext;
#[doc(hidden)]
pub mod pretty;

#[doc(hidden)]
pub use attr::AttrStore;
#[doc(hidden)]
pub use pretty::PrettyPrinter;

use typst_syntax::Source;

/// Entry point for pretty printing a typst document.
#[derive(Debug, Clone)]
pub struct Typstyle {
    source: Source,
    width: usize,
}

impl Typstyle {
    /// Create a new Typstyle instance from a string.
    /// # Example
    /// ```rust
    /// use typstyle_core::Typstyle;
    /// let content = "#{1+1}";
    /// let res = Typstyle::new_with_content(content.to_string(), 80).pretty_print();
    /// ```
    pub fn new_with_content(content: String, width: usize) -> Self {
        // We should ensure that the source tree is spanned.
        Self::new_with_src(Source::detached(content), width)
    }

    /// Create a new Typstyle instance from a [`Source`].
    ///
    /// This is useful when you have a [`Source`] instance and you can avoid reparsing the content.
    pub fn new_with_src(src: Source, width: usize) -> Self {
        Self { source: src, width }
    }

    /// Pretty print the content to a string.
    pub fn pretty_print(&self) -> String {
        let root = self.source.root();
        if root.erroneous() {
            return self.source.text().to_string();
        }
        let attr_store = AttrStore::new(root);
        let printer = PrettyPrinter::new(self.source.clone(), attr_store);
        let markup = root.cast().unwrap();
        let doc = printer.convert_markup(markup);
        let result = doc.pretty(self.width).to_string();
        strip_trailing_whitespace(&result)
    }
}

#[doc(hidden)]
/// Strip trailing whitespace in each line of the input string.
pub fn strip_trailing_whitespace(s: &str) -> String {
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
    let typstyle = Typstyle::new_with_content(content.to_string(), width);
    typstyle.pretty_print()
}
