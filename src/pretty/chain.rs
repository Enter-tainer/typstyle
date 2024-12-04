use itertools::Itertools;
use pretty::DocAllocator;
use std::iter;
use typst_syntax::{SyntaxKind, SyntaxNode};

use super::{ArenaDoc, PrettyPrinter};

enum ChainItem<'a> {
    Commented { body: ArenaDoc<'a> },
}

pub struct ChainStylist<'a> {
    printer: &'a PrettyPrinter<'a>,
    items: Vec<ChainItem<'a>>,
    chain_len: usize,
}

#[derive(Default)]
pub struct ChainStyle {
    /// Do not break line if the chain consists of only one operator.
    pub no_break_single: bool,
}

impl<'a> ChainStylist<'a> {
    pub fn new(printer: &'a PrettyPrinter<'a>) -> Self {
        Self {
            printer,
            items: Default::default(),
            chain_len: 0,
        }
    }

    pub fn process_resolved(
        self,
        nodes: impl Iterator<Item = &'a SyntaxNode>,
        item_kind: SyntaxKind,
        op_pred: impl Fn(&'a SyntaxNode) -> bool,
        rhs_converter: impl Fn(&'a SyntaxNode) -> Option<ArenaDoc<'a>>,
        fallback_converter: impl Fn(&'a SyntaxNode) -> Option<ArenaDoc<'a>>,
    ) -> Self {
        let mut nodes = nodes.collect_vec();
        nodes.reverse();
        self.process(nodes, item_kind, op_pred, rhs_converter, fallback_converter)
    }

    pub fn process(
        mut self,
        nodes: Vec<&'a SyntaxNode>,
        item_kind: SyntaxKind,
        op_pred: impl Fn(&'a SyntaxNode) -> bool,
        rhs_converter: impl Fn(&'a SyntaxNode) -> Option<ArenaDoc<'a>>,
        fallback_converter: impl Fn(&'a SyntaxNode) -> Option<ArenaDoc<'a>>,
    ) -> Self {
        let arena = &self.printer.arena;

        let mut doc = arena.nil();
        for node in nodes {
            if node.kind() == item_kind {
                self.chain_len += 1;
                let mut seen_op = false;
                for child in node.children() {
                    if op_pred(child) {
                        seen_op = true;
                        self.items.push(ChainItem::Commented { body: doc });
                        doc = arena.text(child.text().as_str());
                    } else if seen_op {
                        if let Some(rhs) = rhs_converter(child) {
                            doc += rhs;
                        }
                    }
                }
            } else if let Some(fallback) = fallback_converter(node) {
                doc += fallback;
            }
        }
        self.items.push(ChainItem::Commented { body: doc });

        self
    }

    pub fn print_doc(self, sty: ChainStyle) -> ArenaDoc<'a> {
        let arena = &self.printer.arena;

        let mut docs = vec![];
        for item in self.items {
            match item {
                ChainItem::Commented { body } => docs.push(body),
            }
        }

        let first_doc = docs.remove(0);
        let follow_docs = arena.intersperse(docs, arena.line_());
        if self.chain_len == 1 && sty.no_break_single {
            (first_doc + follow_docs).group()
        } else {
            (first_doc + (arena.line_() + follow_docs).nest(2)).group()
        }
    }
}

/// Iterates over nodes in a syntax tree in a depth-first manner.
///
/// This function takes a starting node and an `accepter` function,
/// which determines the next node to visit. It returns an iterator that yields
/// nodes in a depth-first order based on the logic defined in the `accepter`.
///
/// # Parameters
///
/// - `node`: The starting node from which to begin the iteration.
/// - `accepter`: If it returns `Some`, the iterator will visit this next node;
///   if it returns `None`, the iteration will stop for that path.
pub fn iterate_deep_nodes<'a>(
    node: &'a SyntaxNode,
    accepter: impl Fn(&SyntaxNode) -> Option<&SyntaxNode> + 'a,
) -> impl Iterator<Item = &'a SyntaxNode> {
    let mut current = Some(node);
    iter::from_fn(move || {
        let ret = current;
        if let Some(ret) = ret {
            current = accepter(ret);
            Some(ret)
        } else {
            None
        }
    })
}
