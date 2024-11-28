use rustc_hash::FxHashMap;

use typst_syntax::{
    ast::{Args, AstNode, Math, Raw, Space},
    Span, SyntaxKind, SyntaxNode,
};

struct State {
    is_math: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Attributes {
    /// Manually marked `@typstyle off` or always ignored.
    pub format_disabled: bool,
    /// Has a child of comment.
    pub commented: bool,
    /// If (a) the first space child contains a newline, (b) one of the children is a multiline
    pub multiline: bool,
}

#[derive(Debug, Default)]
pub struct AttrStore {
    attr_map: FxHashMap<Span, Attributes>,
}

impl AttrStore {
    pub fn new(node: &SyntaxNode) -> AttrStore {
        let mut store = AttrStore {
            attr_map: Default::default(),
        };
        store.compute_no_format(node);
        store.compute_multiline(node);
        store
    }

    pub fn is_node_commented(&self, node: &SyntaxNode) -> bool {
        self.attr_map
            .get(&node.span())
            .is_some_and(|attr| attr.commented)
    }

    pub fn is_node_multiline(&self, node: &SyntaxNode) -> bool {
        self.attr_map
            .get(&node.span())
            .is_some_and(|attr| attr.multiline)
    }

    pub fn is_node_format_disabled(&self, node: &SyntaxNode) -> bool {
        self.attr_map
            .get(&node.span())
            .is_some_and(|attr| attr.format_disabled)
    }

    pub fn is_node_unformattable(&self, node: &SyntaxNode) -> bool {
        self.attr_map
            .get(&node.span())
            .is_some_and(|attr| attr.format_disabled || attr.commented)
    }

    fn compute_multiline(&mut self, root: &SyntaxNode) {
        self.compute_multiline_impl(root);
    }

    fn compute_multiline_impl(&mut self, node: &SyntaxNode) {
        if self.is_node_multiline(node) {
            return;
        }
        if node.children().count() == 0 {
            return;
        }
        if let Some(space) = node.cast_first_match::<Space>() {
            if space.to_untyped().text().contains('\n') {
                self.set_multiline(node);
            }
        }
        for child in node.children() {
            self.compute_multiline_impl(child);
            if self.is_node_multiline(child) {
                self.set_multiline(node);
            }
        }
    }

    fn set_multiline(&mut self, node: &SyntaxNode) {
        self.attr_map.entry(node.span()).or_default().multiline = true;
    }

    fn compute_no_format(&mut self, root: &SyntaxNode) {
        let mut state = State { is_math: false };
        self.compute_no_format_impl(root, &mut state);
    }

    fn compute_no_format_impl(&mut self, node: &SyntaxNode, state: &mut State) {
        if self.is_node_format_disabled(node) {
            return;
        }
        let mut format_disabled = false;
        let original_is_math = state.is_math;
        if node.is::<Math>() {
            state.is_math = true;
        }

        for child in node.children() {
            let child_kind = child.kind();
            if child_kind == SyntaxKind::LineComment || child_kind == SyntaxKind::BlockComment {
                self.set_commented(node);
                // @typstyle off affects the whole next block
                if child.text().contains("@typstyle off") {
                    format_disabled = true;
                    self.set_format_disabled(child);
                }
                continue;
            }
            // no format multiline single backtick raw block
            if let Some(raw) = child.cast::<Raw>() {
                if !raw.block() && raw.lines().count() > 1 {
                    self.set_format_disabled(child);
                }
            }
            // no format hash related nodes in math blocks
            if child_kind == SyntaxKind::Hash && state.is_math {
                self.set_format_disabled(node);
                break;
            }
            // no format args in math blocks
            if child.is::<Args>() && state.is_math {
                self.set_format_disabled(child);
                continue;
            }
            if child.children().count() == 0 {
                continue;
            }
            // no format nodes with @typstyle off
            if format_disabled {
                self.set_format_disabled(child);
                format_disabled = false;
                continue;
            }
            self.compute_no_format_impl(child, state);
        }
        state.is_math = original_is_math;
    }

    fn set_format_disabled(&mut self, node: &SyntaxNode) {
        self.attr_map
            .entry(node.span())
            .or_default()
            .format_disabled = true;
    }

    fn set_commented(&mut self, node: &SyntaxNode) {
        self.attr_map.entry(node.span()).or_default().commented = true;
    }
}

#[allow(unused)]
fn is_2d_arg(arg: Args) -> bool {
    for child in arg.to_untyped().children() {
        if child.kind() == SyntaxKind::Semicolon {
            return true;
        }
    }
    false
}
