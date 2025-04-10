use pretty::DocAllocator;
use typst_syntax::{SyntaxKind, SyntaxNode};

use crate::ext::StrExt;

use super::{util::is_comment_node, ArenaDoc, Mode, PrettyPrinter};

/// An item in the flow. A space is added only when the item before and the item after both allow it.
pub struct FlowItem<'a>(Option<FlowItemRepr<'a>>);

struct FlowItemRepr<'a> {
    doc: ArenaDoc<'a>,
    /// Whether a space can be added before the doc.
    space_before: bool,
    /// Whether a space can be added after the doc.
    space_after: bool,
}

impl<'a> FlowItem<'a> {
    pub fn new(doc: ArenaDoc<'a>, space_before: bool, space_after: bool) -> Self {
        Self(Some(FlowItemRepr {
            doc,
            space_before,
            space_after,
        }))
    }

    /// Create an item that allows spaces before and after.
    pub fn spaced(doc: ArenaDoc<'a>) -> Self {
        Self::new(doc, true, true)
    }

    /// Create an item that allows space before.
    pub fn spaced_before(doc: ArenaDoc<'a>, space_after: bool) -> Self {
        Self::new(doc, true, space_after)
    }

    /// Create an item that disallows space before but allows space after.
    pub fn tight_spaced(doc: ArenaDoc<'a>) -> Self {
        Self::new(doc, false, true)
    }

    /// Create an item that disallows space after but allows space before.
    pub fn spaced_tight(doc: ArenaDoc<'a>) -> Self {
        Self::new(doc, true, false)
    }

    /// Create an item that disallows space before and after.
    pub fn tight(doc: ArenaDoc<'a>) -> Self {
        Self::new(doc, false, false)
    }

    pub const fn none() -> Self {
        Self(None)
    }
}

pub struct FlowStylist<'a> {
    printer: &'a PrettyPrinter<'a>,
    doc: ArenaDoc<'a>,
    space_after: bool,
    at_line_start: bool,
}

impl<'a> FlowStylist<'a> {
    pub fn new(printer: &'a PrettyPrinter<'a>) -> Self {
        Self {
            doc: printer.arena.nil(),
            printer,
            space_after: false,
            at_line_start: true,
        }
    }

    pub fn push_comment(&mut self, node: &'a SyntaxNode) {
        let doc = self.printer.convert_comment(node);
        if node.kind() == SyntaxKind::BlockComment {
            self.push_doc(doc, true, true);
        } else {
            if !self.at_line_start {
                self.space_after = true;
            }
            self.push_doc(doc, true, false);
        }
    }

    pub fn push_doc(&mut self, doc: ArenaDoc<'a>, space_before: bool, space_after: bool) {
        if space_before && self.space_after {
            self.doc += self.printer.arena.space();
        }
        self.doc += doc;
        self.space_after = space_after;
        self.at_line_start = false;
    }

    pub fn enter_new_line(&mut self) {
        self.at_line_start = true;
    }

    pub fn into_doc(self) -> ArenaDoc<'a> {
        self.doc
    }
}

impl<'a> PrettyPrinter<'a> {
    /// Convert a flow-like structure with given item producer.
    pub(super) fn convert_flow_like(
        &'a self,
        node: &'a SyntaxNode,
        producer: impl FnMut(&'a SyntaxNode) -> FlowItem<'a>,
    ) -> ArenaDoc<'a> {
        self.convert_flow_like_iter(node.children(), producer)
    }

    pub(super) fn convert_flow_like_iter(
        &'a self,
        children: impl Iterator<Item = &'a SyntaxNode>,
        mut producer: impl FnMut(&'a SyntaxNode) -> FlowItem<'a>,
    ) -> ArenaDoc<'a> {
        let mut flow = FlowStylist::new(self);
        let mut peek_line_comment = false;
        let mut peek_hash = false;
        for child in children {
            let at_line_comment = peek_line_comment;
            peek_line_comment = false;
            let at_hash = peek_hash;
            peek_hash = false;
            if child.kind().is_keyword()
                && !matches!(child.kind(), SyntaxKind::None | SyntaxKind::Auto)
            {
                flow.push_doc(self.arena.text(child.text().as_str()), true, true);
            } else if is_comment_node(child) {
                if child.kind() == SyntaxKind::LineComment {
                    peek_line_comment = true; // defers the linebreak
                }
                flow.push_comment(child);
            } else if at_line_comment
                && child.kind() == SyntaxKind::Space
                && child.text().has_linebreak()
            {
                flow.push_doc(self.arena.hardline(), false, false);
                flow.enter_new_line();
            } else if child.kind() == SyntaxKind::Hash {
                flow.push_doc(self.arena.text("#"), true, false);
                peek_hash = true;
            } else {
                let _g = self.with_mode_if(Mode::Code, at_hash);
                let item = producer(child);
                if let Some(repr) = item.0 {
                    flow.push_doc(repr.doc, repr.space_before, repr.space_after);
                }
            }
        }
        flow.into_doc()
    }

    /// Convert nodes with only keywords, exprs (followed by space), and comments.
    pub(super) fn convert_expr_flow(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        self.convert_flow_like(node, |child| {
            if let Some(expr) = child.cast() {
                FlowItem::spaced(self.convert_expr(expr))
            } else {
                FlowItem::none()
            }
        })
    }
}
