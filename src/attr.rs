use rustc_hash::FxHashMap;

use typst_syntax::{
    ast::{Args, AstNode, Math, Raw},
    Span, SyntaxKind, SyntaxNode,
};

use crate::ext::StrExt;

#[derive(Clone, Copy)]
struct State {
    is_math: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Attributes {
    /// Indicates whether formatting is explicitly disabled (`@typstyle off`) or always ignored.
    pub(self) is_format_disabled: bool,

    /// Indicates whether any child node contains a comment.
    pub(self) has_comment: bool,

    /// Indicates whether the node text contains a linebreak.
    /// Currently, it is only used for equations.
    pub(self) is_multiline: bool,

    /// Indicates whether the node has a multiline "flavor",
    /// determined by the first space child containing a linebreak.
    pub(self) is_multiline_flavor: bool,
}

/// A storage structure that manages formatting attributes for syntax nodes.
#[derive(Debug, Default)]
pub struct AttrStore {
    /// A mapping between syntax node spans and their associated attributes.
    attr_map: FxHashMap<Span, Attributes>,
}

impl AttrStore {
    /// Creates a new `AttrStore` by computing formatting-related attributes
    /// for all descendants of the given syntax node.
    pub fn new(node: &SyntaxNode) -> AttrStore {
        let mut store = AttrStore {
            attr_map: Default::default(),
        };
        store.compute_no_format(node);
        store.compute_multiline(node);
        store
    }

    /// Checks if a given syntax node contains a comment.
    pub fn has_comment(&self, node: &SyntaxNode) -> bool {
        self.check_node_attr(node, |attr| attr.has_comment)
    }

    /// Checks if a given syntax node or any of its descendants contains a linebreak.
    pub fn is_multiline(&self, node: &SyntaxNode) -> bool {
        self.check_node_attr(node, |attr| attr.is_multiline)
    }

    /// Checks if a given syntax node has a multiline flavor.
    pub fn is_multiline_flavor(&self, node: &SyntaxNode) -> bool {
        self.check_node_attr(node, |attr| attr.is_multiline_flavor)
    }

    /// Checks if formatting is explicitly disabled for a given syntax node.
    pub fn is_format_disabled(&self, node: &SyntaxNode) -> bool {
        self.check_node_attr(node, |attr| attr.is_format_disabled)
    }

    /// Checks if a node is unformattable, defined as having formatting disabled
    /// or containing a comment.
    pub fn is_unformattable(&self, node: &SyntaxNode) -> bool {
        self.check_node_attr(node, |attr| attr.is_format_disabled || attr.has_comment)
    }

    fn check_node_attr(&self, node: &SyntaxNode, pred: impl FnOnce(&Attributes) -> bool) -> bool {
        self.attr_map.get(&node.span()).is_some_and(pred)
    }
}

impl AttrStore {
    fn compute_multiline(&mut self, root: &SyntaxNode) {
        self.compute_multiline_impl(root);
    }

    fn compute_multiline_impl(&mut self, node: &SyntaxNode) -> bool {
        let mut is_multiline = false;
        let mut seen_space = false;
        for child in node.children() {
            if child.kind() == SyntaxKind::Space {
                if child.text().has_linebreak() {
                    is_multiline = true;
                    if !seen_space {
                        // Decide multiline flavor based on the first space
                        self.set_multiline_flavor(node);
                    }
                }
                seen_space = true;
            }
            is_multiline |= self.compute_multiline_impl(child);
        }
        if is_multiline {
            self.set_multiline(node);
        }
        is_multiline
    }

    fn set_multiline(&mut self, node: &SyntaxNode) {
        self.attr_map.entry(node.span()).or_default().is_multiline = true;
    }

    fn set_multiline_flavor(&mut self, node: &SyntaxNode) {
        self.attr_map
            .entry(node.span())
            .or_default()
            .is_multiline_flavor = true;
    }

    fn compute_no_format(&mut self, root: &SyntaxNode) {
        self.compute_no_format_impl(root, State { is_math: false });
    }

    fn compute_no_format_impl(&mut self, node: &SyntaxNode, state: State) {
        let state = if node.is::<Math>() {
            State { is_math: true }
        } else {
            state
        };

        // no format multiline single backtick raw block
        if node
            .cast::<Raw>()
            .is_some_and(|raw| !raw.block() && raw.lines().count() > 1)
        {
            self.set_format_disabled(node);
            return;
        }
        // no format args in math blocks
        if node.kind() == SyntaxKind::Args && state.is_math {
            self.set_format_disabled(node);
            return;
        }

        let mut disable_next = false;
        let mut commented = false;
        for child in node.children() {
            let child_kind = child.kind();
            if child_kind == SyntaxKind::LineComment || child_kind == SyntaxKind::BlockComment {
                commented = true;
                // @typstyle off affects the whole next block
                if child.text().contains("@typstyle off") {
                    disable_next = true;
                    self.set_format_disabled(child);
                }
                continue;
            }
            // no format nodes with @typstyle off
            if child_kind != SyntaxKind::Space && disable_next {
                self.set_format_disabled(child);
                disable_next = false;
                continue;
            }
            // no format hash related nodes in math blocks
            if child_kind == SyntaxKind::Hash && state.is_math {
                self.set_format_disabled(node);
                break;
            }
            self.compute_no_format_impl(child, state);
        }
        if commented {
            self.set_commented(node);
        }
    }

    fn set_format_disabled(&mut self, node: &SyntaxNode) {
        self.attr_map
            .entry(node.span())
            .or_default()
            .is_format_disabled = true;
    }

    fn set_commented(&mut self, node: &SyntaxNode) {
        self.attr_map.entry(node.span()).or_default().has_comment = true;
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
