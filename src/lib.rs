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

use typst_syntax::{Source, SyntaxNode};

/// Entry point for pretty printing a typst document.
#[derive(Debug, Clone)]
pub struct Typstyle {
    content: String,
    root: SyntaxNode,
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
        let content = src.text().to_string();
        let root = src.root().clone();
        Self {
            content,
            root,
            width,
        }
    }

    /// Pretty print the content to a string.
    pub fn pretty_print(&self) -> String {
        if self.root.erroneous() {
            return self.content.to_string();
        }
        let attr_store = AttrStore::new(&self.root);
        let printer = PrettyPrinter::new(attr_store);
        let markup = self.root.cast().unwrap();
        let doc = printer.convert_markup(markup);
        let result = doc.pretty(self.width).to_string();
        strip_trailing_whitespace(&result)
    }
}

#[doc(hidden)]
/// Strip trailing whitespace in each line of the input string.
pub fn strip_trailing_whitespace(s: &str) -> String {
    let res = s
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    res + "\n"
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub fn pretty_print_wasm(content: &str, width: usize) -> String {
    let typstyle = Typstyle::new_with_content(content.to_string(), width);
    typstyle.pretty_print()
}
