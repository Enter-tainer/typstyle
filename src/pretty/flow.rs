use pretty::{Arena, DocAllocator};
use typst_syntax::{SyntaxKind, SyntaxNode};

use super::{util::is_comment_node, ArenaDoc, PrettyPrinter};

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
    arena: &'a Arena<'a>,
    printer: &'a PrettyPrinter<'a>,
    doc: ArenaDoc<'a>,
    space_after: bool,
}

impl<'a> FlowStylist<'a> {
    pub fn new(printer: &'a PrettyPrinter<'a>) -> Self {
        Self {
            arena: &printer.arena,
            doc: printer.arena.nil(),
            printer,
            space_after: false,
        }
    }

    pub fn push_comment(&mut self, node: &'a SyntaxNode) {
        let doc = self.printer.convert_comment_br(node);
        if node.kind() == SyntaxKind::BlockComment {
            self.push_doc(doc, true, true);
        } else {
            self.space_after = true;
            self.push_doc(doc, true, false);
        }
    }

    pub fn push_doc(&mut self, doc: ArenaDoc<'a>, space_before: bool, space_after: bool) {
        if space_before && self.space_after {
            self.doc += self.arena.space();
        }
        self.doc += doc;
        self.space_after = space_after;
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
        mut producer: impl FnMut(&'a SyntaxNode) -> FlowItem<'a>,
    ) -> ArenaDoc<'a> {
        let mut flow = FlowStylist::new(self);
        for child in node.children() {
            if child.kind().is_keyword() {
                flow.push_doc(self.arena.text(child.text().as_str()), true, true);
            } else if is_comment_node(child) {
                flow.push_comment(child);
            } else {
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
