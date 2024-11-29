use pretty::DocAllocator;
use typst_syntax::{ast::*, SyntaxKind};

use super::{flow::FlowItem, ArenaDoc, PrettyPrinter};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_import(&'a self, import: ModuleImport<'a>) -> ArenaDoc<'a> {
        self.convert_flow_like(import.to_untyped(), |child| {
            if child.kind() == SyntaxKind::Colon {
                FlowItem::tight_spaced(self.arena.text(":"))
            } else if child.kind() == SyntaxKind::Star {
                // wildcard import
                FlowItem::spaced(self.arena.text("*"))
            } else if let Some(ident) = child.cast() {
                // new_name
                FlowItem::spaced(self.convert_ident(ident))
            } else if let Some(expr) = child.cast() {
                // source
                FlowItem::spaced(self.convert_expr(expr))
            } else if let Some(import_items) = child.cast() {
                // imports
                FlowItem::spaced(self.convert_import_items(import_items))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_import_item_path(
        &'a self,
        import_item_path: ImportItemPath<'a>,
    ) -> ArenaDoc<'a> {
        self.convert_flow_like(import_item_path.to_untyped(), |child| {
            if child.kind() == SyntaxKind::Dot {
                FlowItem::tight(self.arena.text("."))
            } else if let Some(ident) = child.cast() {
                FlowItem::tight(self.convert_ident(ident))
            } else {
                FlowItem::none()
            }
        })
    }

    pub(super) fn convert_import_item_renamed(
        &'a self,
        import_item_renamed: RenamedImportItem<'a>,
    ) -> ArenaDoc<'a> {
        self.convert_flow_like(import_item_renamed.to_untyped(), |child| {
            if let Some(path) = child.cast() {
                FlowItem::spaced(self.convert_import_item_path(path))
            } else if let Some(ident) = child.cast() {
                FlowItem::spaced(self.convert_ident(ident))
            } else {
                FlowItem::none()
            }
        })
    }
}
