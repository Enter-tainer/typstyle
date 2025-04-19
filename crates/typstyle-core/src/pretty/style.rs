/// A style for formatting items
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldStyle {
    /// Fold items if them can fit in a single line
    Fit,
    /// Never fold items
    Never,
    /// Always fold items
    Always,
    /// Try to fold items except the last one in a single line
    Compact,
}
