use pretty::{Arena, DocAllocator};
use typst_syntax::ast::*;

use crate::PrettyPrinter;

use super::{mode::Mode, util::has_comment_children, ArenaDoc};

impl<'a> PrettyPrinter<'a> {
    /// We do not care whether it is `Pattern` or `Expr`.
    /// It is safe to treat it as `Pattern`, since `Pattern` can be `Expr`.
    pub(super) fn convert_parenthesized(
        &'a self,
        parenthesized: Parenthesized<'a>,
    ) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::CodeCont);

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
        if !is_paren_needed(expr) {
            return self.convert_expr(expr);
        }
        let _g = self.with_mode(Mode::CodeCont);
        optional_paren(&self.arena, self.convert_expr(expr))
    }

    /// Parenthesize the body if necessary.
    ///
    /// We must enter continued-code mode before evaluating body.
    pub(super) fn parenthesize_if_necessary(
        &'a self,
        body: impl FnOnce() -> ArenaDoc<'a>,
    ) -> ArenaDoc<'a> {
        if self.current_mode().is_code_continued() {
            return body();
        }
        // SAFETY:
        // - If without paren, the entire expression is in one line, thus safe.
        // - If with paren, surely safe.
        let _g = self.with_mode(Mode::CodeCont);
        optional_paren(&self.arena, body())
    }
}

/// Wrap the body with parentheses if the body is layouted on multiple lines.
fn optional_paren<'a>(arena: &'a Arena<'a>, body: ArenaDoc<'a>) -> ArenaDoc<'a> {
    let open = (arena.text("(") + arena.line()).flat_alt(arena.nil());
    let close = (arena.line() + arena.text(")")).flat_alt(arena.nil());
    ((open + body).nest(2) + close).group()
}

/// Checks if parentheses are needed for an expression that may span multiple lines.
fn is_paren_needed(expr: Expr<'_>) -> bool {
    !matches!(
        expr,
        Expr::Parenthesized(_)
            | Expr::Code(_)
            | Expr::Content(_)
            | Expr::FuncCall(_)
            | Expr::Array(_)
            | Expr::Dict(_)
            | Expr::Conditional(_)
            | Expr::While(_)
            | Expr::For(_)
            | Expr::Contextual(_)
            | Expr::Closure(_)
            | Expr::Raw(_)
    )
}
