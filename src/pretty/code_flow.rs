use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use super::{flow::FlowItem, util::BoolExt, ArenaDoc, PrettyPrinter};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_named(&'a self, named: Named<'a>) -> ArenaDoc<'a> {
        let mut seen_name = false;
        self.convert_flow_like(named.to_untyped(), |child| {
            if child.kind() == SyntaxKind::Colon {
                FlowItem::tight_spaced(self.arena.text(":"))
            } else if child.kind() == SyntaxKind::Hash {
                // name
                FlowItem::spaced_tight(self.arena.text("#"))
            } else if let Some(expr) = child.cast() {
                // expr
                FlowItem::spaced_before(self.convert_expr(expr), seen_name.replace(true))
            } else if let Some(pattern) = child.cast() {
                // pattern
                FlowItem::spaced(self.convert_pattern(pattern))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_keyed(&'a self, keyed: Keyed<'a>) -> ArenaDoc<'a> {
        let mut seen_key = false;
        self.convert_flow_like(keyed.to_untyped(), |child| {
            if child.kind() == SyntaxKind::Colon {
                FlowItem::tight_spaced(self.arena.text(":"))
            } else if let Some(expr) = child.cast() {
                // key, expr
                FlowItem::spaced_before(self.convert_expr(expr), seen_key.replace(true))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_spread(&'a self, spread: Spread<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(spread.to_untyped(), |child| {
            if child.kind() == SyntaxKind::Dots {
                FlowItem::spaced_tight(self.arena.text(".."))
            } else if let Some(expr) = child.cast() {
                // expr, sink_ident, sink_expr
                FlowItem::tight_spaced(self.convert_expr(expr))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_conditional(&'a self, conditional: Conditional<'a>) -> ArenaDoc<'a> {
        self.convert_expr_flow(conditional.to_untyped())
    }

    pub(super) fn convert_while_loop(&'a self, while_loop: WhileLoop<'a>) -> ArenaDoc<'a> {
        self.convert_expr_flow(while_loop.to_untyped())
    }

    pub(super) fn convert_for_loop(&'a self, for_loop: ForLoop<'a>) -> ArenaDoc<'a> {
        enum LookAhead {
            Pattern,
            Iterable,
            Body,
        }
        let mut look_ahead = LookAhead::Pattern;
        self.convert_flow_like(for_loop.to_untyped(), |child| {
            match look_ahead {
                LookAhead::Pattern => {
                    if let Some(pattern) = child.cast() {
                        look_ahead = LookAhead::Iterable;
                        return FlowItem::spaced(self.convert_pattern(pattern));
                    }
                }
                LookAhead::Iterable => {
                    if let Some(expr) = child.cast() {
                        look_ahead = LookAhead::Body;
                        return FlowItem::spaced(self.convert_expr_with_optional_paren(expr));
                    }
                }
                LookAhead::Body => {
                    if let Some(expr) = child.cast() {
                        return FlowItem::spaced(self.convert_expr(expr));
                    }
                }
            }
            FlowItem::none()
        })
    }
}
