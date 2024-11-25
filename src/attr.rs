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
    no_format: bool,
    /// If (a) the first space child contains a newline, (b) one of the children is a multiline
    multiline: bool,
}

impl Attributes {
    pub fn no_format(&self) -> bool {
        self.no_format
    }

    pub fn is_multiline_flavor(&self) -> bool {
        self.multiline
    }
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

    pub fn is_node_multiline(&self, node: &SyntaxNode) -> bool {
        self.attr_map
            .get(&node.span())
            .is_some_and(|attr| attr.multiline)
    }

    pub fn is_node_no_format(&self, node: &SyntaxNode) -> bool {
        self.attr_map
            .get(&node.span())
            .is_some_and(|attr| attr.no_format)
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
        if self.is_node_no_format(node) {
            return;
        }
        let mut no_format = false;
        let original_is_math = state.is_math;
        if node.is::<Math>() {
            state.is_math = true;
        }

        for child in node.children() {
            let child_kind = child.kind();
            if child_kind == SyntaxKind::LineComment || child_kind == SyntaxKind::BlockComment {
                // @typstyle off affects the whole next block
                if child.text().contains("@typstyle off") {
                    no_format = true;
                    self.set_no_format(child);
                }
                // currently we only support comments as children of these nodes
                if !matches!(
                    node.kind(),
                    SyntaxKind::ContentBlock | SyntaxKind::CodeBlock | SyntaxKind::Code
                ) {
                    self.set_no_format(node);
                }
                continue;
            }
            // no format multiline single backtick raw block
            if let Some(raw) = child.cast::<Raw>() {
                if !raw.block() && raw.lines().count() > 1 {
                    self.set_no_format(child);
                }
            }
            // no format hash related nodes in math blocks
            if child_kind == SyntaxKind::Hash && state.is_math {
                self.set_no_format(node);
                break;
            }
            // no format args in math blocks
            if child.cast::<Args>().is_some() && state.is_math {
                self.set_no_format(child);
                continue;
            }
            if child.children().count() == 0 {
                continue;
            }
            // no format nodes with @typstyle off
            if no_format {
                self.set_no_format(child);
                no_format = false;
                continue;
            }
            self.compute_no_format_impl(child, state);
        }
        state.is_math = original_is_math;
    }

    fn set_no_format(&mut self, node: &SyntaxNode) {
        self.attr_map.entry(node.span()).or_default().no_format = true;
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
