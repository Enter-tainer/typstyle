/// Configuration Options for Typstyle Printer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    /// Number of spaces per tab
    pub tab_spaces: usize,
    /// Maximum width of each line.
    pub max_width: usize,
    /// Maximum number of blank lines which can be put between items.
    pub blank_lines_upper_bound: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tab_spaces: 2,
            max_width: 80,
            blank_lines_upper_bound: 2,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_width(mut self, max_width: usize) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn chain_width(&self) -> usize {
        const CHAIN_WIDTH_RATIO: f32 = 0.6;
        (self.max_width as f32 * CHAIN_WIDTH_RATIO) as usize
    }
}
