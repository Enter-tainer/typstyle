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
pub use pretty::PrinterConfig;

use typst_syntax::Source;

#[derive(Debug)]
pub enum FormatError {
    SyntaxError,
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::SyntaxError => write!(f, "The document has syntax errors"),
        }
    }
}

/// Entry point for pretty printing a typst document.
#[derive(Debug, Clone)]
pub struct Typstyle {
    source: Source,
    config: PrinterConfig,
}

impl Typstyle {
    /// Create a new Typstyle instance from a string.
    /// # Example
    /// ```rust
    /// use typstyle_core::{PrinterConfig, Typstyle};
    /// let content = "#{1+1}";
    /// let res = Typstyle::new_with_content(content.to_string(), PrinterConfig {
    ///     max_width: 80,
    ///     ..Default::default()
    /// }).pretty_print();
    /// ```
    pub fn new_with_content(content: impl Into<String>, config: PrinterConfig) -> Self {
        // We should ensure that the source tree is spanned.
        Self::new_with_src(Source::detached(content.into()), config)
    }

    /// Create a new Typstyle instance from a [`Source`].
    ///
    /// This is useful when you have a [`Source`] instance and you can avoid reparsing the content.
    pub fn new_with_src(src: Source, config: PrinterConfig) -> Self {
        Self {
            source: src,
            config,
        }
    }

    /// Pretty print the content to a string.
    pub fn pretty_print(&self) -> Result<String, FormatError> {
        let root = self.source.root();
        if root.erroneous() {
            return Err(FormatError::SyntaxError);
        }
        let attr_store = AttrStore::new(root);
        let printer = PrettyPrinter::new(self.config.clone(), attr_store);
        let markup = root.cast().unwrap();
        let doc = printer.convert_markup(markup);
        let result = doc.pretty(self.config.max_width).to_string();
        let result = strip_trailing_whitespace(&result);
        Ok(result)
    }
}

#[doc(hidden)]
/// Strip trailing whitespace in each line of the input string.
pub fn strip_trailing_whitespace(s: &str) -> String {
    let res = s
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_owned();
    res
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub fn pretty_print_wasm(content: &str, width: usize) -> String {
    let cfg = PrinterConfig::new_with_width(width);
    let typstyle = Typstyle::new_with_content(content, cfg);
    typstyle
        .pretty_print()
        .unwrap_or_else(|_| content.to_string())
}
