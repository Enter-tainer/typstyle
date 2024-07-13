use pretty::BoxDoc;
use typst_syntax::ast::*;

use crate::PrettyPrinter;

impl PrettyPrinter {
    pub(super) fn convert_parenthesized<'a>(
        &'a self,
        parenthesized: Parenthesized<'a>,
    ) -> BoxDoc<'a, ()> {
        if let Some(res) = self.check_disabled(parenthesized.to_untyped()) {
            return res;
        }
        let mut doc = BoxDoc::text("(");
        let inner = self.convert_expr(parenthesized.expr());
        let inner = BoxDoc::line_()
            .append(inner)
            .append(BoxDoc::line_())
            .nest(2)
            .group();
        doc = doc.append(inner);
        doc = doc.append(BoxDoc::text(")"));
        doc
    }

    pub(super) fn convert_expr_with_optional_paren<'a>(&'a self, expr: Expr<'a>) -> BoxDoc<'a, ()> {
        let left_paren_or_nil = BoxDoc::text("(")
            .append(BoxDoc::line())
            .flat_alt(BoxDoc::nil());
        let right_paren_or_nil = BoxDoc::line()
            .append(BoxDoc::text(")"))
            .flat_alt(BoxDoc::nil());
        let body_expr = self.convert_expr(expr);
        left_paren_or_nil
            .append(body_expr)
            .nest(2)
            .append(right_paren_or_nil)
            .group()
    }
}
