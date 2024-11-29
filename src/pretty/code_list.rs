use typst_syntax::{ast::*, SyntaxKind};

use super::{
    list::{ListStyle, ListStylist},
    util::is_only_one_and,
    ArenaDoc, PrettyPrinter,
};

impl<'a> PrettyPrinter<'a> {
    pub(super) fn convert_array(&'a self, array: Array<'a>) -> ArenaDoc<'a> {
        ListStylist::new(self)
            .process_list(array.to_untyped(), |node| self.convert_array_item(node))
            .print_doc(ListStyle {
                separator: ",",
                delim: ("(", ")"),
                add_space_if_empty: false,
                add_trailing_sep_single: true,
                omit_delim_single: false,
                omit_delim_flat: false,
            })
    }

    pub(super) fn convert_dict(&'a self, dict: Dict<'a>) -> ArenaDoc<'a> {
        let all_spread = dict.items().all(|item| matches!(item, DictItem::Spread(_)));

        ListStylist::new(self)
            .process_list(dict.to_untyped(), |node| self.convert_dict_item(node))
            .print_doc(ListStyle {
                separator: ",",
                delim: (if all_spread { "(:" } else { "(" }, ")"),
                add_space_if_empty: false,
                add_trailing_sep_single: false,
                omit_delim_single: false,
                omit_delim_flat: false,
            })
    }

    pub(super) fn convert_destructuring(
        &'a self,
        destructuring: Destructuring<'a>,
    ) -> ArenaDoc<'a> {
        let only_one_pattern = is_only_one_and(destructuring.items(), |it| {
            matches!(*it, DestructuringItem::Pattern(_))
        });

        ListStylist::new(self)
            .process_list(destructuring.to_untyped(), |node| {
                self.convert_destructuring_item(node)
            })
            .always_fold_if(|| only_one_pattern)
            .print_doc(ListStyle {
                separator: ",",
                delim: ("(", ")"),
                add_space_if_empty: false,
                add_trailing_sep_single: only_one_pattern,
                omit_delim_single: false,
                omit_delim_flat: false,
            })
    }

    pub(super) fn convert_params(&'a self, params: Params<'a>, is_unnamed: bool) -> ArenaDoc<'a> {
        let is_single_simple = is_unnamed
            && is_only_one_and(params.children(), |it| {
                matches!(
                    *it,
                    Param::Pos(Pattern::Normal(_)) | Param::Pos(Pattern::Placeholder(_))
                )
            });

        ListStylist::new(self)
            .process_list(params.to_untyped(), |node| self.convert_param(node))
            .always_fold_if(|| is_single_simple)
            .print_doc(ListStyle {
                separator: ",",
                delim: ("(", ")"),
                add_space_if_empty: false,
                add_trailing_sep_single: false,
                omit_delim_single: is_single_simple,
                omit_delim_flat: false,
            })
    }

    pub(super) fn convert_import_items(&'a self, import_items: ImportItems<'a>) -> ArenaDoc<'a> {
        // Note that `ImportItem` does not implement `AstNode`.
        ListStylist::new(self)
            .process_list_impl(import_items.to_untyped(), |child| match child.kind() {
                SyntaxKind::RenamedImportItem => child
                    .cast()
                    .map(|item| self.convert_import_item_renamed(item)),
                SyntaxKind::ImportItemPath => {
                    child.cast().map(|item| self.convert_import_item_path(item))
                }
                _ => Option::None,
            })
            .print_doc(ListStyle {
                separator: ",",
                delim: ("(", ")"),
                add_space_if_empty: false,
                add_trailing_sep_single: false,
                omit_delim_single: false,
                omit_delim_flat: true,
            })
    }
}
