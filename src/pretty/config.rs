/// Configuration Options for Typstyle Printer.
#[derive(Debug)]
pub struct PrinterConfig {
    /// Maximum number of blank lines which can be put between items.
    pub blank_lines_upper_bound: usize,
}

impl Default for PrinterConfig {
    fn default() -> Self {
        Self {
            blank_lines_upper_bound: 2,
        }
    }
}
