use pretty::BoxDoc;
use typst_syntax::ast::*;

use crate::PrettyPrinter;

impl PrettyPrinter {
    pub fn resolve_dot_chain<'a>(&'a self, field_access: FieldAccess<'a>) -> Vec<BoxDoc<'a, ()>> {
        let mut nodes = vec![];
        let mut push_node = |node: BoxDoc<'a, ()>, last_is_field_access: bool| {
            if last_is_field_access {
                nodes.push(node);
            } else {
                let last = nodes.pop().unwrap();
                nodes.push(node.append(last));
            }
        };
        let mut current = field_access.to_untyped();
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
