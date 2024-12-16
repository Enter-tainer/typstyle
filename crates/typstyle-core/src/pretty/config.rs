/// Configuration Options for Typstyle Printer.
#[derive(Debug)]
pub struct PrinterConfig {
    /// Maximum width of each line.
    pub max_width: usize,
    /// The ratio of max width for chains. Not precise.
    pub chain_width_ratio: f32,
    /// Maximum number of blank lines which can be put between items.
    pub blank_lines_upper_bound: usize,
}

impl Default for PrinterConfig {
    fn default() -> Self {
        Self {
            max_width: 80,
            chain_width_ratio: 0.6,
            blank_lines_upper_bound: 2,
        }
    }
}

impl PrinterConfig {
    pub fn chain_width(&self) -> usize {
        (self.max_width as f32 * self.chain_width_ratio) as usize
    }
}
