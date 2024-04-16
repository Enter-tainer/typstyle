#![doc = include_str!("../README.md")]
#[doc(hidden)]
pub mod attr;
#[doc(hidden)]
pub mod pretty;
#[doc(hidden)]
pub mod util;

#[doc(hidden)]
pub use attr::calculate_attributes;
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
    /// let res = Typstyle::with_content(content.to_string(), 80).pretty_print();
    /// assert_eq!(res, "#{\n  1 + 1\n}");
    /// ```
    pub fn new_with_content(content: String, width: usize) -> Self {
        let root = typst_syntax::parse(&content);
        Self {
            content,
            root,
            width,
        }
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
        let attr_map = calculate_attributes(self.root.clone());
        let printer = PrettyPrinter::new(attr_map);
        let markup = self.root.cast().unwrap();
        let doc = printer.convert_markup(markup);
        strip_trailing_whitespace(&doc.pretty(self.width).to_string())
    }
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
    let typstyle = Typstyle::new_with_content(content.to_string(), width);
    typstyle.pretty_print()
}
