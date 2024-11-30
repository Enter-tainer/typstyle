use pretty::{Arena, DocAllocator};
use typst_syntax::ast::*;

use crate::PrettyPrinter;

use super::{util::has_comment_children, ArenaDoc};

impl<'a> PrettyPrinter<'a> {
    /// We do not care whether it is `Pattern` or `Expr`.
    /// It is safe to treat it as `Pattern`, since `Pattern` can be `Expr`.
    pub(super) fn convert_parenthesized(
        &'a self,
        parenthesized: Parenthesized<'a>,
    ) -> ArenaDoc<'a> {
        let pattern = parenthesized.pattern();
        if let Pattern::Parenthesized(paren) = pattern {
            if !has_comment_children(parenthesized.to_untyped()) {
                // Remove a layer of paren if no comment inside.
                return self.convert_parenthesized(paren);
            }
        }

        // Treat is as a list with a single item.
        self.convert_parenthesized_impl(parenthesized)
    }

    /// Convert an expression with optional parentheses.
    /// If the expression is a parenthesized expression, a code block, a content block, or a function call,
    /// the expression will be converted without parentheses.
    /// Otherwise, the expression will be converted with parentheses if it is layouted on multiple lines.
    pub(super) fn convert_expr_with_optional_paren(&'a self, expr: Expr<'a>) -> ArenaDoc<'a> {
        if matches!(
            expr,
            Expr::Parenthesized(_)
                | Expr::Code(_)
                | Expr::Content(_)
                | Expr::FuncCall(_)
                | Expr::Array(_)
                | Expr::Dict(_)
                | Expr::Conditional(_)
                | Expr::For(_)
                | Expr::Contextual(_)
        ) {
            return self.convert_expr(expr);
        }
        let body_expr = self.convert_expr(expr);
        optional_paren(&self.arena, body_expr)
    }
}

/// Wrap the body with parentheses if the body is layouted on multiple lines.
pub(super) fn optional_paren<'a>(arena: &'a Arena<'a>, body: ArenaDoc<'a>) -> ArenaDoc<'a> {
    let left_paren_or_nil = (arena.text("(") + arena.line()).flat_alt(arena.nil());
    let right_paren_or_nil = (arena.line() + arena.text(")")).flat_alt(arena.nil());
    ((left_paren_or_nil + body).nest(2) + right_paren_or_nil).group()
}
