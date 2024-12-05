use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind, SyntaxNode};

use crate::PrettyPrinter;

use super::{
    chain::{iterate_deep_nodes, ChainStyle, ChainStylist},
    ArenaDoc,
};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_field_access(&'a self, field_access: FieldAccess<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.try_convert_dot_chain(field_access.to_untyped()) {
            return res;
        }
        // Comments within field access are not allowed outside code mode
        self.convert_expr(field_access.target())
            + self.arena.text(".")
            + self.convert_ident(field_access.field())
    }

    /// Convert the node as dot chain, if in code, or in markup with at least two FieldAccess and one FuncCall.
    pub(super) fn try_convert_dot_chain(&'a self, node: &'a SyntaxNode) -> Option<ArenaDoc<'a>> {
        if self.current_mode().is_code() {
            return Some(self.convert_dot_chain(node));
        } else if self.current_mode().is_markup() {
            let mut chain_len = 0;
            let mut has_call = false;
            for node in resolve_dot_chain(node) {
                if node.kind() == SyntaxKind::FieldAccess {
                    chain_len += 1;
                } else if node.kind() == SyntaxKind::FuncCall {
                    has_call = true;
                }
            }
            if chain_len > 1 && has_call {
                return Some(self.parenthesize_if_necessary(|| self.convert_dot_chain(node)));
            }
        }
        None
    }

    fn convert_dot_chain(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        ChainStylist::new(self)
            .process_resolved(
                resolve_dot_chain(node),
                |node| node.kind() == SyntaxKind::FieldAccess,
                |child| {
                    if child.kind() == SyntaxKind::Dot {
                        Some(self.arena.text("."))
                    } else {
                        None
                    }
                },
                |child| child.cast().map(|ident| self.convert_ident(ident)),
                |node| {
                    if let Some(func_call) = node.cast::<FuncCall>() {
                        // There is no comment allowed, so we can directly convert args.
                        Some(self.convert_args(func_call.args()))
                    } else {
                        node.cast().map(|expr| self.convert_expr(expr))
                    }
                },
            )
            .print_doc(ChainStyle {
                no_break_single: true,
                ..Default::default()
            })
    }

    pub(super) fn convert_binary_chain(&'a self, binary: Binary<'a>) -> ArenaDoc<'a> {
        let op = binary.op();
        let prec = op.precedence();
        ChainStylist::new(self)
            .process_resolved(
                resolve_binary_chain(binary),
                |node| {
                    node.cast::<Binary>()
                        .is_some_and(|binary| binary.op().precedence() == prec)
                },
                |child| {
                    if child.kind() == SyntaxKind::In && op == BinOp::NotIn {
                        Some(self.arena.text(op.as_str()))
                    } else {
                        BinOp::from_kind(child.kind()).map(|op| self.arena.text(op.as_str()))
                    }
                },
                |child| child.cast().map(|expr| self.convert_expr(expr)),
                |node| node.cast().map(|expr| self.convert_expr(expr)),
            )
            .print_doc(ChainStyle {
                space_around_op: true,
                ..Default::default()
            })
    }
}

pub(super) fn resolve_dot_chain(node: &SyntaxNode) -> impl Iterator<Item = &SyntaxNode> {
    iterate_deep_nodes(node, |current| {
        if let Some(field_access) = current.cast::<FieldAccess>() {
            Some(field_access.target().to_untyped())
        } else if let Some(func_call) = current.cast::<FuncCall>() {
            Some(func_call.callee().to_untyped())
        } else {
            None
        }
    })
}

pub(super) fn resolve_binary_chain(binary: Binary<'_>) -> impl Iterator<Item = &'_ SyntaxNode> {
    let prec = binary.op().precedence();
    iterate_deep_nodes(binary.to_untyped(), move |current| {
        if let Some(binary) = current.cast::<Binary>() {
            if binary.op().precedence() == prec {
                return Some(binary.lhs().to_untyped());
            }
        }
        None
    })
}
