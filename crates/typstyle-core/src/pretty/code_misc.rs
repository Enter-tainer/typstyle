use typst_syntax::ast::*;

use super::{ArenaDoc, PrettyPrinter};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_ident(&'a self, ident: Ident<'a>) -> ArenaDoc<'a> {
        self.convert_verbatim(ident)
    }

    pub(super) fn convert_array_item(&'a self, array_item: ArrayItem<'a>) -> ArenaDoc<'a> {
        match array_item {
            ArrayItem::Pos(p) => self.convert_expr(p),
            ArrayItem::Spread(s) => self.convert_spread(s),
        }
    }

    pub(super) fn convert_dict_item(&'a self, dict_item: DictItem<'a>) -> ArenaDoc<'a> {
        match dict_item {
            DictItem::Named(n) => self.convert_named(n),
            DictItem::Keyed(k) => self.convert_keyed(k),
            DictItem::Spread(s) => self.convert_spread(s),
        }
    }

    pub(super) fn convert_param(&'a self, param: Param<'a>) -> ArenaDoc<'a> {
        match param {
            Param::Pos(p) => self.convert_pattern(p),
            Param::Named(n) => self.convert_named(n),
            Param::Spread(s) => self.convert_spread(s),
        }
    }

    pub fn convert_pattern(&'a self, pattern: Pattern<'a>) -> ArenaDoc<'a> {
        if let Some(res) = self.check_disabled(pattern.to_untyped()) {
            return res;
        }
        match pattern {
            Pattern::Normal(n) => self.convert_expr(n),
            Pattern::Placeholder(p) => self.convert_verbatim(p),
            Pattern::Destructuring(d) => self.convert_destructuring(d),
            Pattern::Parenthesized(p) => self.convert_parenthesized(p),
        }
    }

    pub(super) fn convert_destructuring_item(
        &'a self,
        destructuring_item: DestructuringItem<'a>,
    ) -> ArenaDoc<'a> {
        match destructuring_item {
            DestructuringItem::Spread(s) => self.convert_spread(s),
            DestructuringItem::Named(n) => self.convert_named(n),
            DestructuringItem::Pattern(p) => self.convert_pattern(p),
        }
    }
}
