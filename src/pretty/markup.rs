use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use super::{flow::FlowItem, ArenaDoc, PrettyPrinter};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_heading(&'a self, heading: Heading<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(heading.to_untyped(), |child| {
            if child.kind() == SyntaxKind::HeadingMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(self.convert_markup(markup))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_list_item(&'a self, list_item: ListItem<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(list_item.to_untyped(), |child| {
            if child.kind() == SyntaxKind::ListMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(self.convert_markup(markup).nest(2))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_enum_item(&'a self, enum_item: EnumItem<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(enum_item.to_untyped(), |child| {
            if child.kind() == SyntaxKind::EnumMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(self.convert_markup(markup).nest(2))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_term_item(&'a self, term: TermItem<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(term.to_untyped(), |child| {
            if child.kind() == SyntaxKind::TermMarker {
                FlowItem::spaced(self.arena.text(child.text().as_str()))
            } else if child.kind() == SyntaxKind::Colon {
                FlowItem::tight_spaced(self.arena.text(child.text().as_str()))
            } else if let Some(markup) = child.cast() {
                FlowItem::spaced(self.convert_markup(markup).nest(2))
            } else {
                FlowItem::none()
            }
        })
    }
}
