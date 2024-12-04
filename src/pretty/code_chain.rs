use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxNode};

use crate::PrettyPrinter;

use super::ArenaDoc;

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_field_access(&'a self, field_access: FieldAccess<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_unformattable(field_access.to_untyped()) {
            return res;
        }
        if !self.current_mode().is_code() {
            let left = self.convert_expr(field_access.target());
            let singleline_right = self.arena.text(".") + self.convert_ident(field_access.field());
            return left + singleline_right;
        }
        self.convert_dot_chain(field_access.to_untyped())
    }

    pub(super) fn convert_dot_chain(&'a self, node: &'a SyntaxNode) -> ArenaDoc<'a> {
        let mut chain = self.resolve_dot_chain(node);
        if chain.len() == 2 {
            let last = chain.pop().unwrap();
            let first = chain.pop().unwrap();
            return first + self.arena.text(".") + last;
        }
        let first_doc = chain.remove(0);
        let other_doc = self
            .arena
            .intersperse(chain, self.arena.line_() + self.arena.text("."));
        let chain = first_doc
            + (self.arena.line_() + self.arena.text(".") + other_doc)
                .nest(2)
                .group();
        // if matches!(self.current_mode(), Mode::Markup | Mode::Math) {
        //     optional_paren(chain)
        // } else {
        //     chain
        // }
        chain
    }

    fn resolve_dot_chain(&'a self, node: &'a SyntaxNode) -> Vec<ArenaDoc<'a>> {
        let mut nodes = vec![];
        let mut push_node = |node: ArenaDoc<'a>, last_is_field_access: bool| {
            if last_is_field_access {
                nodes.push(node);
            } else {
                let last = nodes.pop().unwrap();
                nodes.push(node + last);
            }
        };
        let mut current = node;
        let mut last_is_field_access = true;
        loop {
            if let Some(field_access) = current.cast::<FieldAccess>() {
                let rhs = self.convert_ident(field_access.field());
                push_node(rhs, last_is_field_access);
                last_is_field_access = true;
                current = field_access.target().to_untyped();
                continue;
            }
            if let Some(func_call) = current.cast::<FuncCall>() {
                let args = self.convert_args(func_call.args());
                push_node(args, last_is_field_access);
                last_is_field_access = false;
                current = func_call.callee().to_untyped();
                continue;
            }
            let lhs = self.convert_expr(current.cast::<Expr>().unwrap());
            push_node(lhs, last_is_field_access);
            break;
        }
        nodes.reverse();
        nodes
    }
}
