use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use crate::ext::StrExt;

use super::{flow::FlowItem, ArenaDoc, Mode, PrettyPrinter};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_equation(&'a self, equation: Equation<'a>) -> ArenaDoc<'a> {
        let _g = self.with_mode(Mode::Math);

        let body = self.convert_flow_like(equation.to_untyped(), |child| {
            if let Some(math) = child.cast::<Math>() {
                let has_trailing_linebreak = (math.exprs().last())
                    .is_some_and(|expr| matches!(expr, Expr::Linebreak(_)))
                    && (equation.to_untyped().children().nth_back(1))
                        .is_some_and(|it| it.kind() == SyntaxKind::Space)
                    && (equation.to_untyped().children().nth_back(2))
                        .is_some_and(|it| it.kind() == SyntaxKind::Math);
                let body = self.convert_math(math);
                let body = if !equation.block() && has_trailing_linebreak {
                    body + self.arena.space()
                } else {
                    body
                };
                FlowItem::spaced(body)
            } else {
                FlowItem::none()
            }
        });

        let doc = if equation.block() {
            if self.is_break_suppressed() {
                (self.arena.space() + body).nest(self.config.tab_spaces as isize)
                    + self.arena.space()
            } else if self.attr_store.is_multiline(equation.to_untyped()) {
                (self.arena.hardline() + body).nest(self.config.tab_spaces as isize)
                    + self.arena.hardline()
            } else {
                ((self.arena.line() + body).nest(self.config.tab_spaces as isize)
                    + self.arena.line())
                .group()
            }
        } else {
            body.nest(self.config.tab_spaces as isize)
        };
        doc.enclose("$", "$")
    }

    pub(super) fn convert_math(&'a self, math: Math<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_disabled(math.to_untyped()) {
            return res;
        }
        let _g = self.suppress_breaks();
        let mut doc = self.arena.nil();
        let mut peek_hash = false;
        for node in math.to_untyped().children() {
            let at_hash = peek_hash;
            peek_hash = false;
            if let Some(expr) = node.cast::<Expr>() {
                let _g = self.with_mode_if(Mode::Code, at_hash);
                let expr_doc = self.convert_expr(expr);
                doc += expr_doc;
            } else if let Some(space) = node.cast::<Space>() {
                doc += self.convert_space(space);
            } else if node.kind() == SyntaxKind::Hash {
                doc += self.arena.text("#");
                peek_hash = true;
            } else {
                doc += self.convert_trivia_untyped(node);
            }
        }
        doc
    }

    pub(super) fn convert_math_delimited(
        &'a self,
        math_delimited: MathDelimited<'a>,
    ) -> ArenaDoc<'a> {
        let mut inner_nodes = math_delimited.to_untyped().children().as_slice();
        inner_nodes = &inner_nodes[1..inner_nodes.len() - 1];

        let open_space = if let Some((first, rest)) = inner_nodes.split_first() {
            if first.kind() == SyntaxKind::Space {
                inner_nodes = rest;
                if first.text().has_linebreak() {
                    self.arena.hardline()
                } else {
                    self.arena.space()
                }
            } else {
                self.arena.nil()
            }
        } else {
            self.arena.nil()
        };
        let close_space = if let Some((last, rest)) = inner_nodes.split_last() {
            if last.kind() == SyntaxKind::Space {
                inner_nodes = rest;
                if last.text().has_linebreak() {
                    self.arena.hardline()
                } else {
                    self.arena.space()
                }
            } else {
                self.arena.nil()
            }
        } else {
            self.arena.nil()
        };
        let body = self.convert_flow_like_iter(inner_nodes.iter(), |node| {
            if let Some(math) = node.cast::<Math>() {
                FlowItem::tight(self.convert_math(math))
            } else if node.kind() == SyntaxKind::Space {
                // We can not arbitrarily break line here, as it may become ugly.
                FlowItem::tight(if node.text().has_linebreak() {
                    self.arena.line()
                } else {
                    self.arena.space()
                })
            } else {
                FlowItem::none()
            }
        });
        let open = self.convert_expr(math_delimited.open());
        let close = self.convert_expr(math_delimited.close());
        ((open_space + body).nest(self.config.tab_spaces as isize) + close_space)
            .enclose(open, close)
    }

    pub(super) fn convert_math_attach(&'a self, math_attach: MathAttach<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(math_attach.to_untyped(), |node| {
            if let Some(expr) = node.cast::<Expr>() {
                FlowItem::tight(self.convert_expr(expr))
            } else {
                FlowItem::tight(self.convert_verbatim_untyped(node))
            }
        })
    }

    pub(super) fn convert_math_primes(&'a self, math_primes: MathPrimes<'a>) -> ArenaDoc<'a> {
        self.arena.text("'".repeat(math_primes.count()))
    }

    pub(super) fn convert_math_frac(&'a self, math_frac: MathFrac<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(math_frac.to_untyped(), |node| {
            if let Some(expr) = node.cast::<Expr>() {
                FlowItem::spaced(self.convert_expr(expr))
            } else if node.kind() != SyntaxKind::Space {
                FlowItem::spaced(self.convert_verbatim_untyped(node))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_math_root(&'a self, math_root: MathRoot<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(math_root.to_untyped(), |node| {
            if let Some(expr) = node.cast::<Expr>() {
                FlowItem::tight(self.convert_expr(expr))
            } else {
                FlowItem::tight(self.convert_verbatim_untyped(node))
            }
        })
    }
}
