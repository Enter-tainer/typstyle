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

    /// Convert an expression with optional parentheses.
    /// If the expression is a parenthesized expression, a code block, a content block, or a function call,
    /// the expression will be converted without parentheses.
    /// Otherwise, the expression will be converted with parentheses if it is layouted on multiple lines.
    pub(super) fn convert_expr_with_optional_paren<'a>(&'a self, expr: Expr<'a>) -> BoxDoc<'a, ()> {
        if matches!(
            expr,
            Expr::Parenthesized(_)
                | Expr::Code(_)
                | Expr::Content(_)
                | Expr::FuncCall(_)
                | Expr::Array(_)
                | Expr::Dict(_)
        ) {
            return self.convert_expr(expr);
        }
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
