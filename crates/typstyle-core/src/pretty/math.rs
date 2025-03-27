use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use super::{flow::FlowItem, ArenaDoc, Mode, PrettyPrinter};

impl<'a> PrettyPrinter<'a> {
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
        fn has_spaces(math_delimited: MathDelimited<'_>) -> (bool, bool) {
            let mut has_space_before_math = false;
            let mut has_space_after_math = false;
            let mut is_before_math = true;
            for child in math_delimited.to_untyped().children() {
                if child.kind() == SyntaxKind::Math {
                    is_before_math = false;
                } else if child.kind() == SyntaxKind::Space {
                    if is_before_math {
                        has_space_before_math = true;
                    } else {
                        has_space_after_math = true;
                    }
                }
            }
            (has_space_before_math, has_space_after_math)
        }
        let open = self.convert_expr(math_delimited.open());
        let close = self.convert_expr(math_delimited.close());
        let body = self.convert_math(math_delimited.body());
        let (has_space_before_math, has_space_after_math) = has_spaces(math_delimited);

        body.enclose(
            if has_space_before_math {
                self.arena.space()
            } else {
                self.arena.nil()
            },
            if has_space_after_math {
                self.arena.space()
            } else {
                self.arena.nil()
            },
        )
        .nest(self.config.tab_spaces as isize)
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
